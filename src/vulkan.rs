use crate::common::*;
use ash::{vk::{self, DisplayModePropertiesKHR}, extensions::{ext, khr}, Entry, Instance};
use drm::Connector;

macro_rules! cstr {
    ($str:expr) => {
        concat!($str, "\0").as_ptr() as *const _
    };
}
macro_rules! env_int {
    ($var:expr) => {
        env!($var).parse().expect("Program was compiled with invalid environment variables")
    };
}

/// Get a potentially null-terminated UTF-8 byte array as a `&str`
/// 
/// # Safety
/// `string` must be valid UTF-8
unsafe fn array_str(string: &[i8]) -> &str {
    std::str::from_utf8_unchecked(
        std::slice::from_raw_parts(string.as_ptr() as *const u8, string.len())
    ).trim_end_matches('\0')
}

#[derive(config::Config)]
pub struct Config {
    
}
impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}

pub struct Vulkan {
    #[allow(unused)]
    entry: Entry,
    instance: Instance,
    display_khr: khr::Display,
    acquire_drm_display_ext: ext::AcquireDrmDisplay,
    surface_khr: khr::Surface,
    devices: Vec<Device>,
    err_devices: Vec<(vk::PhysicalDevice, Error)>
}
impl Vulkan {
    const APP_NAME: *const i8 = cstr!("wayvk");
    //#[cfg(debug_assertions)]
    // Validation layers are a debug-build requirement
    //const VALIDATION_LAYERS: &'static[*const i8] = &[cstr!("VK_LAYER_KHRONOS_validation")];
    //#[cfg(not(debug_assertions))]
    const VALIDATION_LAYERS: &'static[*const i8] = &[];

    const EXTENSIONS: &'static [*const i8] = &[
        ash::extensions::khr::Surface::name().as_ptr(),
        ash::extensions::khr::Display::name().as_ptr(),
        ash::extensions::ext::AcquireDrmDisplay::name().as_ptr(),
        //cstr!("VK_EXT_display_surface_counter"),
        ash::extensions::khr::GetPhysicalDeviceProperties2::name().as_ptr(),
        // TODO: Clean up with vkReleaseDisplayEXT - The mesa vulkan driver seems to close the in-use drm fd
        cstr!("VK_EXT_direct_mode_display")
    ];

    pub fn new() -> Result<Self> {
        let entry = Entry::linked();
        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 3, 0),
            p_application_name: Self::APP_NAME,
            application_version: vk::make_api_version(
                0,
                env_int!("CARGO_PKG_VERSION_MAJOR"),
                env_int!("CARGO_PKG_VERSION_MINOR"),
                env_int!("CARGO_PKG_VERSION_PATCH")
            ),
            ..Default::default()
        };
        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&Self::EXTENSIONS)
            .enabled_layer_names(Self::VALIDATION_LAYERS);
        let instance = unsafe { entry.create_instance(&create_info, None) }?;

        let display_khr = khr::Display::new(&entry, &instance);
        let surface_khr = khr::Surface::new(&entry, &instance);
        let acquire_drm_display_ext = ext::AcquireDrmDisplay::new(&entry, &instance);

        let mut vulkan = Self {
            entry,
            instance,

            display_khr,
            surface_khr,
            acquire_drm_display_ext,

            devices: Vec::new(),
            err_devices: Vec::new()
        };

        let physical_devices = unsafe { vulkan.instance.enumerate_physical_devices()? };
        for physical_device in physical_devices {
            match Device::new(&vulkan, physical_device) {
                Ok(device) => vulkan.devices.push(device),
                Err(err) => vulkan.err_devices.push((physical_device, err))
            }
        }

        Ok(vulkan)
    }
}
impl Drop for Vulkan {
    fn drop(&mut self) {
        let mut devices = Vec::new();
        std::mem::swap(&mut devices, &mut self.devices);
        unsafe {
            for device in devices {
                device.drop(self);
            }
            self.instance.destroy_instance(None);
        }
    }
}
impl std::fmt::Debug for Vulkan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vulkan")
            .field("devices", &self.devices)
            .field("err_devices", &self.err_devices)
            .finish()
    }
}

pub struct Device {
    name: String,
    drm: drm::Device,
    physical_device: vk::PhysicalDevice,
    device: ash::Device,
    swapchain_khr: khr::Swapchain,
    graphics_queue_family: u32,
    transfer_queue_family: Option<u32>,
    graphics_queue: vk::Queue,
    transfer_queue: Option<vk::Queue>,

    //display_props: Vec<DisplayProperties>,
    displays: Vec<Display>
}
impl Device {
    const REQUIRED_EXTENSIONS: &'static [&'static std::ffi::CStr] = &[
        ash::extensions::ext::PhysicalDeviceDrm::name(),
        //cstr!("")
        ash::extensions::khr::Swapchain::name(),
        //ash::extensions::khr::DisplaySwapchain::name()
    ];
    const DESIRED_EXTENSIONS: &'static [&'static std::ffi::CStr] = &[
        //"VK_EXT_physical_device_drm"
    ];
    pub fn new(vulkan: &Vulkan, physical_device: vk::PhysicalDevice) -> Result<Self> {
        let mut drm_props = vk::PhysicalDeviceDrmPropertiesEXT::default();
        let mut props = vk::PhysicalDeviceProperties2 {
            p_next: (&mut drm_props) as *mut _ as *mut _,
            ..Default::default()
        };
        unsafe { vulkan.instance.get_physical_device_properties2(physical_device, &mut props) };
        let features = unsafe { vulkan.instance.get_physical_device_features(physical_device) };

        let name = props.properties.device_name;
        let name = unsafe { array_str(&name) };

        if features.geometry_shader != 1 {
            return Err(Error::DeviceNoPresent(name.to_string()))
        }
        if drm_props.has_primary == 0 || drm_props.has_render == 0 {
            return Err(Error::NoDrm(name.to_string()))
        }

        let drm = drm::Device::open_primary(drm_props.primary_minor as u32)?;
        drm.set_master().ok();

        let supported_extensions = unsafe { vulkan.instance.enumerate_device_extension_properties(physical_device)? };
        let extensions = Self::extensions(&supported_extensions)?;
        let extensions: Vec<_> = extensions.iter().map(|e| e.as_ptr()).collect();

        let (
            graphics_queue_family,
            transfer_queue_family
        ) = Self::queue_family(vulkan, physical_device).ok_or_else(|| Error::DeviceNoPresent(name.to_string()))?;

        let mut queue_create_infos = vec![
            vk::DeviceQueueCreateInfo::default()
                .queue_family_index(graphics_queue_family)
                .queue_priorities(&[1.0])
        ];
        if let Some(transfer_queue_family) = transfer_queue_family {
            queue_create_infos.push(
                vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(transfer_queue_family)
                    .queue_priorities(&[1.0])
            )
        }

        let device_create_info = vk::DeviceCreateInfo::default()
            .enabled_extension_names(&extensions)
            .enabled_layer_names(Vulkan::VALIDATION_LAYERS)
            .queue_create_infos(&queue_create_infos);
        
        let device = unsafe { vulkan.instance.create_device(physical_device, &device_create_info, None)? };
        let graphics_queue = unsafe { device.get_device_queue(graphics_queue_family, 0) };
        let transfer_queue = transfer_queue_family.map(|queue_family| unsafe {
            device.get_device_queue(queue_family, 0)
        });

        let swapchain_khr = khr::Swapchain::new(&vulkan.instance, &device);

        let mut device = Self {
            name: name.to_string(),
            drm,
            physical_device,
            device,
            swapchain_khr,
            graphics_queue_family,
            transfer_queue_family,
            graphics_queue,
            transfer_queue,

            //display_props,
            //plane_props,
            displays: Vec::new()
        };


        let planes = unsafe {
            let planes = vulkan.display_khr.get_physical_device_display_plane_properties(physical_device)?;
            let mut planes_with_support = Vec::new();
            for (plane_index, plane) in planes.into_iter().enumerate() {
                let support = vulkan.display_khr.get_display_plane_supported_displays(physical_device, plane_index as u32)?;
                planes_with_support.push((plane, support)) 
            }
            planes_with_support
        };

        // Get displays
        for connector in device.drm.resources()?.connectors {
            if let Ok(mut display) = Display::new(vulkan, &device, connector) {
                for (index, plane) in planes.iter().enumerate() {
                    let index = index.try_into().unwrap();
                    if display.display == plane.0.current_display {
                        display.current_planes.push(DisplayPlane { index, stack_index: plane.0.current_stack_index })
                    }
                    if plane.1.contains(&display.display) {
                        display.supported_planes.push(index)
                    }
                }
                if !display.modes.is_empty() {
                    if let Err(e) = display.create_surface(vulkan, &device) {
                        eprintln!("Failed to create a surface for connector {}: {e:?}", display.connector.id)
                    }
                }
                device.displays.push(display);
            }
        }

        Ok(device)
    }
    pub fn queue_family(vulkan: &Vulkan, physical_device: vk::PhysicalDevice) -> Option<(u32, Option<u32>)> {
        let props = unsafe { vulkan.instance.get_physical_device_queue_family_properties(physical_device) };
        let mut graphics = None;
        let mut transfer = None;
        for (i, prop) in props.into_iter().enumerate() {
            if prop.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                graphics = Some(i as u32)
            } else if prop.queue_flags.contains(vk::QueueFlags::TRANSFER) {
                transfer = Some(i as u32)
            }
        }
        if let Some(graphics) = graphics {
            Some((graphics, transfer))
        } else {
            None
        }
    }
    pub fn extensions(supported: &[vk::ExtensionProperties]) -> Result<Vec<&'static std::ffi::CStr>> {
        let mut extensions = Self::REQUIRED_EXTENSIONS.to_vec();
        for &desired in Self::DESIRED_EXTENSIONS {
            if supported.iter().find(|vk::ExtensionProperties { spec_version: _, extension_name }| {
                let extension_name = unsafe { std::ffi::CStr::from_ptr(extension_name.as_ptr()) };
                // It would be prudent to check for a minimum version too
                desired == extension_name
            }).is_some() {
                extensions.push(desired)
            }
        }

        Ok(extensions)
    }
    pub fn transfer_queue(&self) -> &vk::Queue {
        if let Some(queue) = self.transfer_queue.as_ref() {
            queue
        } else {
            &self.graphics_queue
        }
    }
    pub fn drop(mut self, vulkan: &Vulkan) {
        let mut displays = Vec::new();
        std::mem::swap(&mut displays, &mut self.displays);
        for display in displays {
            display.drop(vulkan, &self)
        }
    }
}
impl Drop for Device {
    fn drop(&mut self) {
        unsafe { self.device.destroy_device(None) };
        self.drm.drop_master().ok();
    }
}
impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("name", &self.name)
            .field("drm", &self.drm)
            .field("physical_device", &self.physical_device)
            .field("graphics_queue_family", &self.graphics_queue_family)
            .field("transfer_queue_family", &self.transfer_queue_family)
            .field("displays", &self.displays)
            .finish()
    }
}

#[derive(Debug)]
pub struct DisplayPlane {
    /// The plane physical device index
    index: u32,
    /// The current z-order of the plane on the display
    stack_index: u32
}

#[derive(Debug)]
pub struct Display {
    current_planes: Vec<DisplayPlane>,
    supported_planes: Vec<u32>,
    display: vk::DisplayKHR,
    connector: Connector,
    modes: Vec<vk::DisplayModePropertiesKHR>,
    surface: Option<Surface>
}
impl Display {
    pub fn new(vulkan: &Vulkan, device: &Device, connector: Connector) -> Result<Self> {
        let display = unsafe { vulkan.acquire_drm_display_ext.get_drm_display(
            device.physical_device,
            device.drm.raw_fd() as i32,
            connector.id
        )? };

        unsafe { vulkan.acquire_drm_display_ext.acquire_drm_display(device.physical_device, device.drm.raw_fd() as i32, display) }.ok();

        let modes = unsafe { vulkan.display_khr.get_display_mode_properties(device.physical_device, display)? };

        Ok(Self {
            current_planes: Vec::new(),
            supported_planes: Vec::new(),
            display,
            connector,
            modes,
            surface: None
        })
    }
    pub fn create_surface(&mut self, vulkan: &Vulkan, device: &Device) -> Result<()> {
        self.surface = Some(Surface::new(vulkan, device, self)?);
        Ok(())
    }
    /// Get the display mode that the user or hardware would prefer to use.
    pub fn preferred_mode(&self, _device: &Device) -> Option<vk::DisplayModePropertiesKHR> {
        // TODO: match to user preference, if set

        // Get the current/native DRM mode (undocumented - is it actually always the first?)
        let mode = self.connector.modes.first()?;
        let (width, height) = mode.resolution;
        let dimensions = vk::Extent2D { width: width as u32, height: height as u32 };
        let refresh_rate = (mode.refresh() * 1000.0) as u32;
        for &mode in &self.modes {
            if mode.parameters.visible_region == dimensions && mode.parameters.refresh_rate == refresh_rate {
                return Some(mode)
            }
        }
        None
    }
    pub fn drop(mut self, vulkan: &Vulkan, device: &Device) {
        if let Some(surface) = self.surface.take() {
            surface.drop(vulkan, device)
        }
    }
}

#[derive(Debug)]
pub struct Surface {
    mode: DisplayModePropertiesKHR,
    surface: vk::SurfaceKHR,
    swapchain: vk::SwapchainKHR,
    images: Vec<vk::Image>,
    views: Vec<vk::ImageView>
}
impl Surface {
    /// Attempt to create a surface on the display.
    /// 
    /// May fail for various legitimate reasons, such as no monitor being connected.
    pub fn new(vulkan: &Vulkan, device: &Device, display: &Display) -> Result<Self> {
        let mode = if let Some(mode) = display.preferred_mode(device) {
            mode
        } else {
            *display.modes.first().ok_or(Error::NoDisplayModes)?
        };
        let plane = if let Some(plane) = display.current_planes.first() {
            plane.index
        } else {
            *display.supported_planes.first().ok_or(Error::NoDisplayPlanes)?
        };
        // TODO: pick the most optimal plane and mode

        let create_info = vk::DisplaySurfaceCreateInfoKHR::default()
            .display_mode(mode.display_mode)
            .image_extent(mode.parameters.visible_region)
            .alpha_mode(vk::DisplayPlaneAlphaFlagsKHR::OPAQUE)
            .plane_index(plane)
            .plane_stack_index(0)
            .transform(vk::SurfaceTransformFlagsKHR::IDENTITY);
        let surface = unsafe { vulkan.display_khr.create_display_plane_surface(&create_info, None) }?;

        if !unsafe { vulkan.surface_khr.get_physical_device_surface_support(device.physical_device, device.graphics_queue_family, surface) }? {
            // The graphics queue must not be capable of presenting
            // As surface creation is done well after device creation, solving this will suck
            // Hardware like that probably doesn't exist anyway, at least for now, so it is in the backlog
            return Err(Error::SurfaceNoPresent)
        }

        let capabilities = unsafe { vulkan.surface_khr.get_physical_device_surface_capabilities(device.physical_device, surface) }?;
        let formats = unsafe { vulkan.surface_khr.get_physical_device_surface_formats(device.physical_device, surface) }?;
        let present_modes = unsafe { vulkan.surface_khr.get_physical_device_surface_present_modes(device.physical_device, surface) }?;

        // TODO: get format from DRM and match DMA format
        let preferred_format = vk::SurfaceFormatKHR { format: vk::Format::B8G8R8A8_SRGB, color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR };
        let format = if formats.contains(&preferred_format) {
            preferred_format
        } else {
            let format = *formats.first().unwrap();
            eprintln!("Warning: Surface format is {:?}!", format);
            format
        };

        // TODO: let user choose, etc. Hardware will normally only have FIFO for direct display
        let preferred_present_mode = vk::PresentModeKHR::MAILBOX;
        let present_mode = if present_modes.contains(&preferred_present_mode) {
            preferred_present_mode
        } else {
            *present_modes.first().unwrap()
        };

        let image_count = (capabilities.min_image_count + 1).min(capabilities.max_image_count.max(capabilities.min_image_count));
        let create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(surface)
            .min_image_count(image_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .present_mode(present_mode)
            .image_extent(mode.parameters.visible_region)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            // Graphics and Presentation is guaranteed to be the same queue
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            // TODO: support transforms
            .pre_transform(capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .clipped(true);
        
        let swapchain = unsafe { device.swapchain_khr.create_swapchain(&create_info, None) }?;
        let images = unsafe { device.swapchain_khr.get_swapchain_images(swapchain) }?;
        let mut views = Vec::with_capacity(images.len());
        for &image in &images {
            let create_info = vk::ImageViewCreateInfo::default()
                .image(image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(format.format)
                .subresource_range(vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1)
                );
            views.push(unsafe { device.device.create_image_view(&create_info, None) }?)
        }
        
        Ok(Self {
            mode,
            surface,
            swapchain,
            images,
            views
        })
    }
    pub fn drop(mut self, vulkan: &Vulkan, device: &Device) {
        unsafe {
            let mut views = Vec::new();
            std::mem::swap(&mut views, &mut self.views);
            for view in views {
                device.device.destroy_image_view(view, None)
            }
            device.swapchain_khr.destroy_swapchain(self.swapchain, None);
            vulkan.surface_khr.destroy_surface(self.surface, None);
        }
    }
}
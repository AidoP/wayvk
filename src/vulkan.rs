use std::ops::{Deref, DerefMut};

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

/// Wrap an object with some glue to run on drop.
/// 
/// Allows a manual memory managed object to participate in RAII
/// for short periods, such as before a wrapping object with a real
/// `drop` implementation is provided.
pub struct DropGlue<T, F: FnMut(&mut T)>(Option<T>, F);
impl<T, F: FnMut(&mut T)> DropGlue<T, F> {
    pub fn new(value: T, glue: F) -> Self {
        Self(Some(value), glue)
    }
    /// Remove the value from the drop glue without running the drop glue.
    pub fn take(mut self) -> T {
        unsafe { self.0.take().unwrap_unchecked() }
    }
}
impl<T, F: FnMut(&mut T)> Drop for DropGlue<T, F> {
    fn drop(&mut self) {
        if let Some(value) = self.0.as_mut() {
            self.1(value)
        }
    }
}
impl<T, F: FnMut(&mut T)> Deref for DropGlue<T, F> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref().unwrap_unchecked() }
    }
}
impl<T, F: FnMut(&mut T)> DerefMut for DropGlue<T, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut().unwrap_unchecked() }
    }
}

pub const WINDOW_SHADER_VERT: &'static [u8] = include_bytes!("shader/window.vert.spv");
pub const WINDOW_SHADER_FRAG: &'static [u8] = include_bytes!("shader/window.frag.spv");

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
        // Core 1.1 ash::extensions::khr::GetPhysicalDeviceProperties2::name().as_ptr(),
        // Core 1.1 vk::KhrExternalMemoryCapabilitiesFn::name().as_ptr(),
        // TODO: Clean up with vkReleaseDisplayEXT - The mesa vulkan driver seems to close the in-use drm fd
        vk::ExtDirectModeDisplayFn::name().as_ptr(),
    ];

    /// Create a vulkan instance without initialising any devices
    pub fn dummy() -> Result<Self> {
        let entry = Entry::linked();
        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 2, 0),
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

        Ok(Self {
            entry,
            instance,

            display_khr,
            surface_khr,
            acquire_drm_display_ext,

            devices: Vec::new(),
            err_devices: Vec::new()
        })
    }

    pub fn new() -> Result<Self> {
        let entry = Entry::linked();
        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 2, 0),
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

    pub fn no_devices(&self) -> bool {
        self.devices.is_empty()
    }
    pub fn render(&mut self) -> Result<()> {
        for device in &mut self.devices {
            device.render()?;
        }
        Ok(())
    }
}
impl Drop for Vulkan {
    fn drop(&mut self) {
        let mut devices = Vec::new();
        std::mem::swap(&mut devices, &mut self.devices);
        unsafe {
            for mut device in devices {
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
    graphics_command_pool: vk::CommandPool,

    //display_props: Vec<DisplayProperties>,
    displays: Vec<Display>,
    shader: ShaderModule
}
impl Device {
    const REQUIRED_EXTENSIONS: &'static [&'static std::ffi::CStr] = &[
        ash::extensions::ext::PhysicalDeviceDrm::name(),
        //cstr!("")
        ash::extensions::khr::Swapchain::name(),

        // dma_buf sharing - optional?
        // Core 1.1 vk::KhrExternalMemoryFn::name(),
        ash::extensions::khr::ExternalMemoryFd::name(),
        vk::ExtExternalMemoryDmaBufFn::name(),
        ash::extensions::ext::ImageDrmFormatModifier::name()

        // Core 1.1 vk::KhrBindMemory2Fn::name(),
        // Core 1.2 vk::KhrImageFormatListFn::name(),

        // Core 1.1 ash::extensions::khr::Maintenance1::name(),
        // Core 1.1 ash::extensions::khr::GetMemoryRequirements2::name(),


        // Core 1.1 - optional capability vk::KhrSamplerYcbcrConversionFn::name()
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
        
        let device = unsafe { vulkan.instance.create_device(physical_device, &device_create_info, None) }?;
        let device = DropGlue::new(device, |device| unsafe {
            device.destroy_device(None);
        });
        let graphics_queue = unsafe { device.get_device_queue(graphics_queue_family, 0) };
        let transfer_queue = transfer_queue_family.map(|queue_family| unsafe {
            device.get_device_queue(queue_family, 0)
        });

        let create_info = vk::CommandPoolCreateInfo::default()
            // TODO: what is our command buffer record strategy? Windows are pretty static... .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(graphics_queue_family);
        let graphics_command_pool = unsafe { device.create_command_pool(&create_info, None) }?;
        let graphics_command_pool = DropGlue::new(graphics_command_pool, |command_pool| unsafe {
            device.destroy_command_pool(*command_pool, None);
        });

        let swapchain_khr = khr::Swapchain::new(&vulkan.instance, &device);

        let shader = ShaderModule::new(device.deref(), WINDOW_SHADER_VERT, WINDOW_SHADER_FRAG)?;

        let mut device = DropGlue::new(Self {
            name: name.to_string(),
            drm,
            physical_device,
            graphics_command_pool: graphics_command_pool.take(),
            device: device.take(),
            swapchain_khr,
            graphics_queue_family,
            transfer_queue_family,
            graphics_queue,
            transfer_queue,

            //display_props,
            //plane_props,
            displays: Vec::new(),
            shader
        }, |device| device.drop(vulkan) );


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

        Ok(device.take())
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
    pub fn render(&mut self) -> Result<()> {
        let mut displays = Vec::new();
        std::mem::swap(&mut displays, &mut self.displays);
        let result = (|| {
            for display in &mut displays {
                if let Some(mut surface) = display.surface.take() {
                    surface.render(self)?;
                }
            }
            Ok(())
        })();
        std::mem::swap(&mut displays, &mut self.displays);
        result
    }
    pub fn drop(&mut self, vulkan: &Vulkan) {
        let mut displays = Vec::new();
        std::mem::swap(&mut displays, &mut self.displays);
        for display in displays {
            display.drop(vulkan, &self)
        }
    }
}
impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.shader.drop(&self.device);
            self.device.destroy_command_pool(self.graphics_command_pool, None);
            self.device.destroy_device(None);
        }
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
        use syslib::FileDescriptor;
        let display = unsafe { vulkan.acquire_drm_display_ext.get_drm_display(
            device.physical_device,
            device.drm.fd().raw() as i32,
            connector.id
        )? };

        unsafe { vulkan.acquire_drm_display_ext.acquire_drm_display(device.physical_device, device.drm.fd().raw() as i32, display) }.ok();

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
    format: vk::SurfaceFormatKHR,
    surface: vk::SurfaceKHR,
    swapchain: vk::SwapchainKHR,
    pipeline: WindowPipeline,
    images: Vec<vk::Image>,
    views: Vec<vk::ImageView>,
    framebuffers: Vec<vk::Framebuffer>,
    frames: Vec<Frame>,
    current_frame: usize
}
impl Surface {
    /// Attempt to create a surface on the display.
    /// 
    /// May fail for various legitimate reasons, such as no monitor being connected.
    //  TODO: Memory leaks on error...
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
        let surface = DropGlue::new(surface, |surface| unsafe { 
            vulkan.surface_khr.destroy_surface(*surface, None)
        });

        if !unsafe { vulkan.surface_khr.get_physical_device_surface_support(device.physical_device, device.graphics_queue_family, *surface) }? {
            // The graphics queue must not be capable of presenting
            // As surface creation is done well after device creation, solving this will suck
            // Hardware like that probably doesn't exist anyway, at least for now, so it is in the backlog
            return Err(Error::SurfaceNoPresent)
        }

        let capabilities = unsafe { vulkan.surface_khr.get_physical_device_surface_capabilities(device.physical_device, *surface) }?;
        let formats = unsafe { vulkan.surface_khr.get_physical_device_surface_formats(device.physical_device, *surface) }?;
        let present_modes = unsafe { vulkan.surface_khr.get_physical_device_surface_present_modes(device.physical_device, *surface) }?;

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
            .surface(*surface)
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
        let swapchain = DropGlue::new(swapchain, |swapchain| unsafe {
            device.swapchain_khr.destroy_swapchain(*swapchain, None)
        });
        let images = unsafe { device.swapchain_khr.get_swapchain_images(*swapchain) }?;
        let mut views = DropGlue::new(Vec::with_capacity(images.len()), |views| unsafe {
            for view in views {
                device.device.destroy_image_view(*view, None)
            }
        });
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

        let pipeline = DropGlue::new(WindowPipeline::new(device, mode, format.format)?, |pipeline| {
            pipeline.drop(device)
        });

        let mut framebuffers = DropGlue::new(Vec::new(), |framebuffers| unsafe {
            for framebuffer in framebuffers {
                device.device.destroy_framebuffer(*framebuffer, None);
            }
        });
        let create_info = vk::FramebufferCreateInfo::default()
            .render_pass(pipeline.render_pass)
            .layers(1)
            .width(mode.parameters.visible_region.width)
            .height(mode.parameters.visible_region.height);
        for &view in views.deref() {
            let views = [view];
            let create_info = create_info.clone()
                .attachments(&views);
            framebuffers.push(unsafe { device.device.create_framebuffer(&create_info, None) }?)
        }

        let frames = Frame::new(device, (framebuffers.len() + 1).try_into().unwrap())?;
        
        Ok(Self {
            mode,
            format,
            surface: surface.take(),
            swapchain: swapchain.take(),
            pipeline: pipeline.take(),
            images,
            views: views.take(),
            framebuffers: framebuffers.take(),
            frames,
            current_frame: 0
        })
    }
    pub fn render(&mut self, device: &Device) -> Result<()> {
        const CLEAR_VALUES: &'static [vk::ClearValue] = &[
            vk::ClearValue {
                color: vk::ClearColorValue { float32: [0.0, 0.0, 1.0, 1.0] }
            }
        ];
        let frame = &self.frames[self.current_frame];
        // Ensure there is a resource available
        let fences = [frame.ready_fence];
        unsafe {
            device.device.wait_for_fences(&fences, false, u64::MAX)?;
            device.device.reset_fences(&fences)?;
        }

        // Acquire a swapchain image
        let (image, suboptimal) = unsafe { device.swapchain_khr.acquire_next_image(self.swapchain, u64::MAX, frame.image_semaphore, vk::Fence::null()) }?;
        let image: usize = image.try_into().unwrap();
        let framebuffer = self.framebuffers[image];

        unsafe {
            let begin_info = vk::CommandBufferBeginInfo::default();
            device.device.reset_command_buffer(frame.command_buffer, vk::CommandBufferResetFlags::empty())?;
            device.device.begin_command_buffer(frame.command_buffer, &begin_info)?;
            let begin_info = vk::RenderPassBeginInfo::default()
                .render_pass(self.pipeline.render_pass)
                .framebuffer(framebuffer)
                .render_area(vk::Rect2D {
                    offset: Default::default(),
                    extent: self.mode.parameters.visible_region
                })
                .clear_values(CLEAR_VALUES);
            device.device.cmd_begin_render_pass(frame.command_buffer, &begin_info, vk::SubpassContents::INLINE);
            device.device.cmd_bind_pipeline(frame.command_buffer, vk::PipelineBindPoint::GRAPHICS, self.pipeline.pipeline);


            device.device.cmd_end_render_pass(frame.command_buffer);
            device.device.end_command_buffer(frame.command_buffer)?;

            let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
            let wait_semaphores = [frame.image_semaphore];
            let signal_semaphores = [frame.render_semaphore];
            let command_buffers = [frame.command_buffer];
            let submit_info = vk::SubmitInfo::default()
                .command_buffers(&command_buffers)
                .wait_semaphores(&wait_semaphores)
                .signal_semaphores(&signal_semaphores)
                .wait_dst_stage_mask(&wait_stages);
            let submits = [submit_info];
            device.device.queue_submit(device.graphics_queue, &submits, frame.ready_fence)?;

            let wait_semaphores = [frame.render_semaphore];
            let swapchains = [self.swapchain];
            let image_indices = [image as u32];
            let present_info = vk::PresentInfoKHR::default()
                .wait_semaphores(&wait_semaphores)
                .swapchains(&swapchains)
                .image_indices(&image_indices);
            let suboptimal = device.swapchain_khr.queue_present(device.graphics_queue, &present_info)?;
        }

        self.current_frame += 1;
        if self.current_frame >= self.framebuffers.len() {
            self.current_frame = 0
        }
        Ok(())
    }
    pub fn drop(mut self, vulkan: &Vulkan, device: &Device) {
        unsafe {
            device.device.device_wait_idle().ok();
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

/// A graphics-pipeline for drawing windows
#[derive(Debug)]
pub struct WindowPipeline {
    pipeline_layout: vk::PipelineLayout,
    render_pass: vk::RenderPass,
    pipeline: vk::Pipeline
}
impl WindowPipeline {
    pub fn new(device: &Device, mode: vk::DisplayModePropertiesKHR, format: vk::Format) -> Result<Self> {
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default();
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::TRIANGLE_STRIP)
            .primitive_restart_enable(false);
        let viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: mode.parameters.visible_region.width as f32,
            height: mode.parameters.visible_region.height as f32,
            min_depth: 0.0,
            max_depth: 1.0
        };
        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: mode.parameters.visible_region
        };
        let dynamic_state = vk::PipelineDynamicStateCreateInfo::default()
            .dynamic_states(&[
                vk::DynamicState::VIEWPORT,
                vk::DynamicState::SCISSOR
            ]);
        let viewports = [viewport];
        let scissors = [scissor];
        let viewport = vk::PipelineViewportStateCreateInfo::default()
            .viewports(&viewports)
            .scissors(&scissors);

        let rasterizer = vk::PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::CLOCKWISE)
            .depth_bias_enable(false);
        let multisampler = vk::PipelineMultisampleStateCreateInfo::default()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);
        let color_blend_attachment = vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(vk::ColorComponentFlags::RGBA)
            .blend_enable(false);
        let color_blend_attachments = [color_blend_attachment];
        let color_blend = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .attachments(&color_blend_attachments);
        let create_info = vk::PipelineLayoutCreateInfo::default();
        let pipeline_layout = unsafe { device.device.create_pipeline_layout(&create_info, None) }?;
        let pipeline_layout = DropGlue::new(pipeline_layout, |pipeline_layout| unsafe {
            device.device.destroy_pipeline_layout(*pipeline_layout, None);
        });

        let color_attachment = vk::AttachmentDescription::default()
            .format(format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);
        let color_attachment_ref = vk::AttachmentReference::default()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
        let color_attachments = [color_attachment];
        let color_attachment_refs = [color_attachment_ref];
        let render_subpass = vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_attachment_refs);
        let subpasses = [render_subpass];
        let color_attachment_dependency = vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE);
        let dependencies = [color_attachment_dependency];
        let create_info = vk::RenderPassCreateInfo::default()
            .attachments(&color_attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);
        let render_pass = unsafe { device.device.create_render_pass(&create_info, None) }?;
        let render_pass = DropGlue::new(render_pass, |render_pass| unsafe {
            device.device.destroy_render_pass(*render_pass, None)
        });

        // TODO: file bug report - radv seems to have a segfault if the entry point is misnamed
        let vertex_shader = vk::PipelineShaderStageCreateInfo::default()
            .module(device.shader.vertex)
            .name(unsafe { std::ffi::CStr::from_ptr(cstr!("vert")) })
            .stage(vk::ShaderStageFlags::VERTEX);
        let fragment_shader = vk::PipelineShaderStageCreateInfo::default()
            .module(device.shader.fragment)
            .name(unsafe { std::ffi::CStr::from_ptr(cstr!("frag")) })
            .stage(vk::ShaderStageFlags::FRAGMENT);

        let shader_stages = [vertex_shader, fragment_shader];
        let create_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisampler)
            .color_blend_state(&color_blend)
            .dynamic_state(&dynamic_state)
            .layout(*pipeline_layout)
            .render_pass(*render_pass)
            .subpass(0);
        let create_infos = [create_info];
        let pipeline = unsafe { device.device.create_graphics_pipelines(vk::PipelineCache::null(), &create_infos, None) };
        let pipeline = match pipeline {
            Ok(pipeline) => pipeline[0],
            Err((_, error)) => return Err(Error::Vulkan(error))
        };
        
        Ok(Self {
            pipeline_layout: pipeline_layout.take(),
            render_pass: render_pass.take(),
            pipeline
        })
    }
    pub fn drop(&mut self, device: &Device) {
        unsafe {
            device.device.destroy_pipeline(self.pipeline, None);
            device.device.destroy_render_pass(self.render_pass, None);
            device.device.destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}

/// The resources required to render a frame.
/// 
/// Not tied to swapchain images as the CPU may want to start rendering while all swapchain images are busy.
#[derive(Debug)]
pub struct Frame {
    command_buffer: vk::CommandBuffer,
    ready_fence: vk::Fence,
    image_semaphore: vk::Semaphore,
    render_semaphore: vk::Semaphore
}
impl Frame {
    pub fn new(device: &Device, count: u32) -> Result<Vec<Self>> {
        let create_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(device.graphics_command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(count);
        let command_buffers = unsafe { device.device.allocate_command_buffers(&create_info) }?;
        let fence_create_info = vk::FenceCreateInfo::default()
            .flags(vk::FenceCreateFlags::SIGNALED);
        let semaphore_create_info = vk::SemaphoreCreateInfo::default();
        let mut frames = DropGlue::new(Vec::with_capacity(count as usize), |frames: &mut Vec<Self>| for frame in frames {
            frame.drop(device)
        });
        for command_buffer in command_buffers {

            let ready_fence = unsafe { device.device.create_fence(&fence_create_info, None) }?;
            let ready_fence = DropGlue::new(ready_fence, |fence| unsafe {
                device.device.destroy_fence(*fence, None)
            });
            let image_semaphore = unsafe { device.device.create_semaphore(&semaphore_create_info, None) }?;
            let image_semaphore = DropGlue::new(image_semaphore, |semaphore| unsafe {
                device.device.destroy_semaphore(*semaphore, None)
            });
            let render_semaphore = unsafe { device.device.create_semaphore(&semaphore_create_info, None) }?;
            frames.push(Self {
                command_buffer,
                ready_fence: ready_fence.take(),
                image_semaphore: image_semaphore.take(),
                render_semaphore
            });
        }
        Ok(frames.take())
    }

    pub fn drop(&mut self, device: &Device) {
        unsafe {
            device.device.destroy_fence(self.ready_fence, None);
            device.device.destroy_semaphore(self.image_semaphore, None);
            device.device.destroy_semaphore(self.render_semaphore, None);
        }
    }
}

pub struct ShaderModule {
    vertex: vk::ShaderModule,
    fragment: vk::ShaderModule
}
impl ShaderModule {
    pub fn new(device: &ash::Device, vert: &[u8], frag: &[u8]) -> Result<Self> {
        let mut reader = std::io::Cursor::new(vert);
        let vert = ash::util::read_spv(&mut reader)?;
        let create_info = vk::ShaderModuleCreateInfo::default()
            .code(&vert);
        let vertex = unsafe { device.create_shader_module(&create_info, None) }?;
        let vertex = DropGlue::new(vertex, |shader| unsafe {
            device.destroy_shader_module(*shader, None)
        });

        let mut reader = std::io::Cursor::new(frag);
        let frag = ash::util::read_spv(&mut reader)?;
        let create_info = vk::ShaderModuleCreateInfo::default()
            .code(&frag);
        let fragment = unsafe { device.create_shader_module(&create_info, None) }?;
        let fragment = DropGlue::new(fragment, |shader| unsafe {
            device.destroy_shader_module(*shader, None)
        });
        Ok(Self {
            vertex: vertex.take(),
            fragment: fragment.take()
        })
    }
    pub fn drop(&mut self, device: &ash::Device) {
        unsafe {
            device.destroy_shader_module(self.vertex, None);
            device.destroy_shader_module(self.fragment, None);
        }
    }
}
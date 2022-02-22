use std::{fmt, collections::{HashMap, HashSet}, ops::{Deref, DerefMut}};
use ash::{vk, extensions::khr, Entry, Instance};
macro_rules! cstr {
    ($str:expr) => {
        concat!($str, "\0").as_ptr() as _
    };
}
macro_rules! env_int {
    ($var:expr) => {
        env!($var).parse().expect("Program was compiled with invalid environment variables")
    };
}

mod window;

const APP_NAME: *const i8 = cstr!("wayvk");

#[cfg(debug_assertions)]
// Validation layers are a debug-build requirement
const VALIDATION_LAYERS: &'static[*const i8] = &[cstr!("VK_LAYER_KHRONOS_validation")];
#[cfg(not(debug_assertions))]
const VALIDATION_LAYERS: &'static[*const i8] = &[];

fn required_instance_extensions() -> Vec<*const i8> {
    vec![
        cstr!("VK_KHR_surface"),
        cstr!("VK_KHR_display"),
        //cstr!("VK_EXT_direct_mode_display"),
        //cstr!("VK_EXT_acquire_drm_display"),
        cstr!("VK_EXT_display_surface_counter")
    ]
}
fn required_device_extensions() -> Vec<*const i8> {
    vec![
        cstr!("VK_EXT_display_control"),
        cstr!("VK_KHR_swapchain")
    ]
}

macro_rules! shader {
    ($name:expr, $device:expr) => {
        {
            #[inline]
            fn shader(device: &ash::Device) -> Result<ShaderModule> {
                use ash::util::read_spv;
                use std::io::Cursor;
                let frag = include_bytes!(concat!("../shader/", $name, ".frag.spv"));
                let vert = include_bytes!(concat!("../shader/", $name, ".vert.spv"));
                let frag = read_spv(&mut Cursor::new(frag)).map_err(|e| Error::ShaderLoad($name, e))?;
                let vert = read_spv(&mut Cursor::new(vert)).map_err(|e| Error::ShaderLoad($name, e))?;
                let create_info = vk::ShaderModuleCreateInfo::builder()
                    .code(&frag);
                let frag = unsafe { device.create_shader_module(&create_info, None) }?;
                let create_info = vk::ShaderModuleCreateInfo::builder()
                    .code(&vert);
                let vert = unsafe { device.create_shader_module(&create_info, None) }?;
                Ok(ShaderModule {
                    frag,
                    vert
                })
            }
            shader($device)
        }
    };
}

struct ShaderModule {
    frag: vk::ShaderModule,
    vert: vk::ShaderModule
}
impl ShaderModule {
    fn vertex(&self) -> vk::PipelineShaderStageCreateInfo {
        vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(self.vert)
            .name(unsafe {std::ffi::CStr::from_ptr(cstr!("vert"))})
            .build()
    }
    fn fragment(&self) -> vk::PipelineShaderStageCreateInfo {
        vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(self.frag)
            .name(unsafe {std::ffi::CStr::from_ptr(cstr!("frag"))})
            .build()
    }
    fn destroy(&mut self, device: &ash::Device) {
        unsafe {
            device.destroy_shader_module(self.frag, None);
            device.destroy_shader_module(self.vert, None);
        }
    }
}

pub struct Image {
    image: vk::Image,
    view: vk::ImageView
}
impl Image {
    pub fn create_view(device: &DeviceHandle, image: vk::Image, format: vk::Format) -> Result<Self> {
        let subresource = vk::ImageSubresourceRange::builder()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1);
        let create_info = vk::ImageViewCreateInfo::builder()
            .image(image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(format)
            .components(vk::ComponentMapping::default())
            .subresource_range(*subresource);
        let view = unsafe { device.device.create_image_view(&create_info, None) }?;
        Ok(Self {
            image,
            view
        })
    }
}

/// A frame that is currently being processed
struct InFlight {
    /// Signalled once the acquired frame is writable
    acquire_semaphore: vk::Semaphore,
    /// Signalled once the frame has finished rendering
    render_semaphore: vk::Semaphore,
    /// Signalled once presentation has completed
    present_fence: vk::Fence,
    /// The index of the frame being recorded
    frame: Option<u32>
}
impl InFlight {
    fn new(device: &DeviceHandle) -> Result<Self> {
        unsafe {
            let create_info = vk::SemaphoreCreateInfo::builder();
            let acquire_semaphore = device.device.create_semaphore(&create_info, None)?;
            let destroy = &mut || device.device.destroy_semaphore(acquire_semaphore, None);
            let render_semaphore = cleanup(destroy, device.device.create_semaphore(&create_info, None))?;
            let destroy = &mut || {
                device.device.destroy_semaphore(render_semaphore, None);
                destroy()
            };
            let create_info = vk::FenceCreateInfo::builder();
            let present_fence = cleanup(destroy, device.device.create_fence(&create_info, None))?;
            Ok(Self {
                acquire_semaphore,
                render_semaphore,
                present_fence,
                frame: None
            })
        }
    }
    fn destroy(&mut self, device: &DeviceHandle) {

    }
}

struct Frame {
    image: Image,
    framebuffer: vk::Framebuffer,
    command_buffer: vk::CommandBuffer,
}
impl Frame {
    fn new(device: &DeviceHandle, command_buffer: vk::CommandBuffer, renderpass: vk::RenderPass, image: vk::Image, extent: vk::Extent2D, format: vk::Format) -> Result<Self> {
        let destroy = &mut || unsafe {
            device.device.free_command_buffers(device.graphics_queue.command_pool, &[command_buffer]);
        };
        let image = cleanup(destroy, Image::create_view(device, image, format))?;
        let destroy = &mut || unsafe {
            device.device.destroy_image_view(image.view, None);
            destroy()
        };

        let attachments = [image.view];
        let create_info = vk::FramebufferCreateInfo::builder()
            .attachments(&attachments)
            .render_pass(renderpass)
            .width(extent.width)
            .height(extent.height)
            .layers(1);
        let framebuffer = cleanup(destroy, unsafe { device.device.create_framebuffer(&create_info, None) })?;
        Ok(Self {
            image,
            framebuffer,
            command_buffer
        })
    }
    fn destroy(&mut self, device: &DeviceHandle) {
        unsafe {
            device.device.free_command_buffers(device.graphics_queue.command_pool, &[self.command_buffer]);
            device.device.destroy_framebuffer(self.framebuffer, None);
            device.device.destroy_image_view(self.image.view, None);
        }
    }
}

pub struct Swapchain {
    format: vk::SurfaceFormatKHR,
    present_mode: vk::PresentModeKHR,
    extent: vk::Extent2D,
    image_count: u32,
    swapchain: vk::SwapchainKHR,
    pipeline_layout: vk::PipelineLayout,
    renderpass: vk::RenderPass,
    pipeline: vk::Pipeline,
    frames: Vec<Frame>,
    in_flight: [InFlight; 2],
    /// The in_flight frame that is currently recording
    current: usize
}
impl Swapchain {
    fn new(vulkan: &VulkanHandle, device: &DeviceHandle, display: &DisplayHandle, surface: &SurfaceHandle) -> Result<Self> {
        let capabilities = unsafe { vulkan.surface_khr.get_physical_device_surface_capabilities(device.physical_device, surface.surface) }?;
        let formats = unsafe { vulkan.surface_khr.get_physical_device_surface_formats(device.physical_device, surface.surface) }?;
        let format = Swapchain::best_format(formats)?;
        let present_modes = unsafe { vulkan.surface_khr.get_physical_device_surface_present_modes(device.physical_device, surface.surface) }?;
        let present_mode = Swapchain::best_present_mode(present_modes);
        let extent = Swapchain::best_extent(display);
        let image_count = Swapchain::best_image_count(&capabilities);

        let create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(surface.surface)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_array_layers(1)
            .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .clipped(true) // There should never be any occluded pixels
            .min_image_count(image_count)
            .image_extent(extent)
            .present_mode(present_mode)
            .image_format(format.format)
            .image_color_space(format.color_space);
        let mut queue_families = [device.graphics_queue.queue_family, 0];
        let create_info = if let Some(present_queue) = surface.present_queue {
            queue_families[1] = present_queue;
            create_info.image_sharing_mode(vk::SharingMode::CONCURRENT).queue_family_indices(&queue_families)
        } else {
            create_info.image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        };
        let swapchain = unsafe { device.swapchain_khr.create_swapchain(&create_info, None) }?;
        let destroy = &mut || unsafe { device.swapchain_khr.destroy_swapchain(swapchain, None) };

        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::builder();
        let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_STRIP)
            .primitive_restart_enable(false);
        let viewport = vk::Viewport::builder()
            .width(extent.width as f32)
            .height(extent.height as f32)
            .max_depth(1.0);
        let viewports = [*viewport];
        let scissor = vk::Rect2D {
            offset: vk::Offset2D::default(),
            extent
        };
        let scissors = [scissor];
        let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(&viewports)
            .scissors(&scissors);
        let rasterization_state = vk::PipelineRasterizationStateCreateInfo::builder()
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::CLOCKWISE);
        let multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);
        let color_blend_attachment_state = vk::PipelineColorBlendAttachmentState::builder()
            .color_write_mask(vk::ColorComponentFlags::RGBA)
            .blend_enable(false);
        let attachments = [*color_blend_attachment_state];
        let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .attachments(&attachments);
        let create_info = vk::PipelineLayoutCreateInfo::builder();
        let pipeline_layout = cleanup(destroy, unsafe{ device.device.create_pipeline_layout(&create_info, None) })?;
        let destroy = &mut || unsafe {
            device.device.destroy_pipeline_layout(pipeline_layout, None);
            destroy()
        };
        let attachment_info = vk::AttachmentDescription::builder()
            .format(format.format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);
        let attachment_ref = vk::AttachmentReference::builder()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
        let attachments = [*attachment_ref];
        let subpass_info = vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&attachments);
        let attachments = [*attachment_info];
        let subpasses = [*subpass_info];
        let create_info = vk::RenderPassCreateInfo::builder()
            .attachments(&attachments)
            .subpasses(&subpasses);
        let renderpass = cleanup(destroy, unsafe { device.device.create_render_pass(&create_info, None) })?;
        let destroy = &mut || unsafe {
            device.device.destroy_render_pass(renderpass, None);
            destroy()
        };
        let vertex = device.window_shader.vertex();
        let fragment = device.window_shader.fragment();
        let stages = [vertex, fragment];
        let create_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(&stages)
            .vertex_input_state(&vertex_input_state)
            .input_assembly_state(&input_assembly_state)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterization_state)
            .multisample_state(&multisample_state)
            .color_blend_state(&color_blend_state)
            .layout(pipeline_layout)
            .render_pass(renderpass)
            .subpass(0);
        let create_infos = [*create_info];
        let pipelines = unsafe { device.device.create_graphics_pipelines(vk::PipelineCache::null(), &create_infos, None) };
        let pipelines = pipelines.map_err(|(pipelines, r)| {
            for pipeline in pipelines {
                unsafe { device.device.destroy_pipeline(pipeline, None) };
            }
            r
        });
        let pipelines = cleanup(destroy, pipelines)?;
        let pipeline = pipelines[0];
        let destroy = &mut || unsafe {
            device.device.destroy_pipeline(pipeline, None);
            destroy()
        };
        
        let images = cleanup(destroy, unsafe { device.swapchain_khr.get_swapchain_images(swapchain) })?;

        let create_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(device.graphics_queue.command_pool)
            .command_buffer_count(images.len() as u32);
        let mut command_buffers = cleanup(destroy, unsafe { device.device.allocate_command_buffers(&create_info) })?;
        let mut frames: Vec<Frame> = Vec::with_capacity(images.len());
        let destroy = &mut |frames: &mut [Frame], command_buffers: &mut [vk::CommandBuffer]| unsafe {
            device.device.free_command_buffers(device.graphics_queue.command_pool, &command_buffers);
            for frame in frames {
                frame.destroy(device)
            }
            destroy()
        };

        for image in images {
            let frame = Frame::new(device, command_buffers.pop().unwrap(), renderpass, image, extent, format.format);
            let frame = cleanup(&mut || destroy(&mut frames, &mut command_buffers), frame)?;
            frames.push(frame)
        };

        let destroy = &mut || destroy(&mut frames, &mut command_buffers);
        let mut frame1 = cleanup(destroy, InFlight::new(device))?;
        let destroy = &mut || {
            frame1.destroy(device);
            destroy()
        };
        let frame2 = cleanup(destroy, InFlight::new(device))?;
        let in_flight = [frame1, frame2];

        Ok(Self {
            format,
            present_mode,
            extent,
            image_count,
            swapchain,
            pipeline_layout,
            renderpass,
            pipeline,
            frames,
            in_flight,
            current: 0
        })
    }
    fn best_format(formats: Vec<vk::SurfaceFormatKHR>) -> Result<vk::SurfaceFormatKHR> {
        formats.into_iter()
            .find(|&format| format == vk::SurfaceFormatKHR { format: vk::Format::B8G8R8A8_SRGB, color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR })
            .ok_or(Error::MissingSupport("No applicable format is available for the surface"))
    }
    fn best_present_mode(modes: Vec<vk::PresentModeKHR>) -> vk::PresentModeKHR {
        // TODO: Persistent displays?, immediate mode if supported and the user really wants to for some reason?
        /*if modes.contains(vk::PresentModeKHR::MAILBOX) {
            vk::PresentModeKHR::MAILBOX
        } else {
            vk::PresentModeKHR::FIFO
        }*/
        vk::PresentModeKHR::FIFO
    }
    fn best_extent(display: &DisplayHandle) -> vk::Extent2D {
        display.mode.parameters.visible_region
    }
    fn best_image_count(capabilities: &vk::SurfaceCapabilitiesKHR) -> u32 {
        if capabilities.max_image_count > 0 {
            (capabilities.min_image_count + 1).min(capabilities.max_image_count)
        } else {
            capabilities.min_image_count + 1
        }
    }
    /// Returns true if a frame is pending submission.
    /// If a frame is pending submission it should be drawn to in a busy loop to ensure it does not miss the next frame presentation target and cause high latency
    fn draw(&mut self, device: &DeviceHandle) -> Result<()> {
        let (current, next) = if self.current == 0 {
            let [current, next] = &mut self.in_flight;
            (current, next)
        } else {
            let [current, next] = &mut self.in_flight;
            (next, current)
        };
        // If possible, get another image so that the existing draw commands can be submitted as soon as possible for lower latency.
        // Otherwise we leave the image unsubmitted so that further draw commands can be submitted without waiting
        if let Ok((frame, suboptimal)) = unsafe { device.swapchain_khr.acquire_next_image(self.swapchain, 0, next.acquire_semaphore, vk::Fence::null()) } {
            // A new frame is available, submit the old one
            if let Some(frame) = current.frame {
                self.current ^= 1;
                let frame = &self.frames[frame as usize];
                unsafe {
                    device.device.cmd_end_render_pass(frame.command_buffer);
                    device.device.end_command_buffer(frame.command_buffer)?;
                }
            }

            // Prepare the next frame for use
            let current = next;
            current.frame = Some(frame);
            let frame = &self.frames[frame as usize];
        } else {

        }
        
        Ok(())
    }
    fn destroy(&mut self, device: &DeviceHandle) {
        unsafe {
            for frame in self.frames.iter_mut() {
                frame.destroy(device)
            }
            device.device.destroy_pipeline(self.pipeline, None);
            device.device.destroy_render_pass(self.renderpass, None);
            device.device.destroy_pipeline_layout(self.pipeline_layout, None);
            device.swapchain_khr.destroy_swapchain(self.swapchain, None);
        }
    }
}

struct Surface {
    surface: SurfaceHandle,
    swapchain: Option<Swapchain>
}
impl Surface {
    fn new(vulkan: &VulkanHandle, physical_device: vk::PhysicalDevice, display: &DisplayHandle, graphics_queue: u32, present_queues: &mut HashSet<u32>, queue_family_properties: &[vk::QueueFamilyProperties], plane: u32) -> Result<Self> {
        let plane_capabilities = unsafe { vulkan.display_khr.get_display_plane_capabilities(physical_device, display.mode.display_mode, plane) }?;
        let create_info = vk::DisplaySurfaceCreateInfoKHR::builder()
            .alpha_mode(plane_capabilities.supported_alpha)
            // TODO: monitor rotating support
            .transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
            .display_mode(display.mode.display_mode);
        let surface = unsafe { vulkan.display_khr.create_display_plane_surface(&create_info, None) }?;
        let destroy = &mut || unsafe { vulkan.surface_khr.destroy_surface(surface, None) };
        // Determine the queue family that is capable of presenting this surface
        let mut present_queue = None;
        if !cleanup(destroy, unsafe { vulkan.surface_khr.get_physical_device_surface_support(physical_device, graphics_queue, surface) })? {
            for (index, properties) in queue_family_properties.iter().enumerate() {
                let index = index as u32;
                if cleanup(destroy, unsafe { vulkan.surface_khr.get_physical_device_surface_support(physical_device, index, surface) })? {
                    present_queues.insert(index);
                    present_queue = Some(index);
                    break
                }
            }
        }
        Ok(Self {
            surface: SurfaceHandle {
                plane,
                surface,
                present_queue,
            },
            swapchain: None
        })
    }
    /// Recreate the surface if it is out of date, such as when the display mode changes
    pub fn recreate(&mut self, vulkan: &VulkanHandle) {
        //unsafe { vulkan.surface_khr.destroy_surface(self.surface.surface, None) };
        todo!()
    }
    pub fn create_swapchain(&mut self, vulkan: &VulkanHandle, device: &DeviceHandle, display: &DisplayHandle) -> Result<()> {
        if self.swapchain.is_none() {
            Ok(self.swapchain = Some(Swapchain::new(vulkan, device, display, self)?))
        } else {
            Ok(())
        }
    }
    fn destroy(&mut self, vulkan: &VulkanHandle, device: &DeviceHandle) {
        if let Some(mut swapchain) = self.swapchain.take() {
            swapchain.destroy(device)
        }
        unsafe { vulkan.surface_khr.destroy_surface(self.surface.surface, None) };
    }
}
impl Deref for Surface {
    type Target = SurfaceHandle;
    fn deref(&self) -> &Self::Target {
        &self.surface
    }
}
impl DerefMut for Surface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.surface
    }
}
pub struct SurfaceHandle {
    plane: u32,
    surface: vk::SurfaceKHR,
    /// The queue family used for presentation, or none if the graphics queue can be used
    present_queue: Option<u32>
}

struct Display {
    display: DisplayHandle,
    surface: Option<Surface>
}
impl Display {
    fn new(vulkan: &VulkanHandle, physical_device: vk::PhysicalDevice, properties: vk::DisplayPropertiesKHR) -> Result<Self> {
        let display_modes = unsafe { vulkan.display_khr.get_display_mode_properties(physical_device, properties.display) }?;
        // Choose the display mode that best matches the native resolution
        let mode = display_modes.iter()
            .find(|mode| mode.parameters.visible_region == properties.physical_resolution);
        // TODO: Attempt creating a mode matching the native resolution, then choose the best alternative (closest matching? highest resolution?)
        let mode = *mode.unwrap_or_else(|| display_modes.first().unwrap(/* TODO: is having a display mode guaranteed? */));
        let mut modes = HashMap::new();
        for mode in display_modes {
            modes.insert(mode.display_mode, mode);
        }
        Ok(Self {
            display: DisplayHandle {
                properties,
                modes,
                mode
            },
            surface: None
        })
    }
    pub fn name(&self) -> Option<&str> {
        unsafe {
            if self.display.properties.display_name.is_null() {
                None
            } else {
                // RADV currently uses the string "monitor" for all displays
                // TODO: fix the driver
                // https://gitlab.freedesktop.org/mesa/mesa/-/blob/main/src/vulkan/wsi/wsi_common_display.c#L302
                std::ffi::CStr::from_ptr(self.display.properties.display_name).to_str().ok()
            }
        }
    }
    fn destroy(&mut self, vulkan: &VulkanHandle, device: &DeviceHandle) {
        // TODO: do we need to clean up displays that are lost?
        // TODO: support for acquire_display extensions and sharing with other programs
        if let Some(mut surface) = self.surface.take() {
            surface.destroy(vulkan, device)
        }
    }
}
impl Deref for Display {
    type Target = DisplayHandle;
    fn deref(&self) -> &Self::Target {
        &self.display
    }
}
impl DerefMut for Display {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.display
    }
}
struct DisplayHandle {
    properties: vk::DisplayPropertiesKHR,
    modes: HashMap<vk::DisplayModeKHR, vk::DisplayModePropertiesKHR>,
    mode: vk::DisplayModePropertiesKHR,
}

pub struct Queue {
    queue_family: u32,
    queue: vk::Queue,
    command_pool: vk::CommandPool
}
impl Queue {
    fn new(device: &ash::Device, queue_family: u32, queue: vk::Queue) -> Result<Self> {
        let create_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(queue_family);
        let command_pool = unsafe{ device.create_command_pool(&create_info, None) }?;
        Ok(Self {
            queue_family,
            queue,
            command_pool
        })
    }
    fn destroy(&mut self, device: &ash::Device) {
        unsafe{ device.destroy_command_pool(self.command_pool, None) };
    }
}
impl std::hash::Hash for Queue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.queue_family.hash(state)
    }
}

/// An opaque type representing a window's resources on the GPU
type WindowHandle = u32;

struct Device {
    device: DeviceHandle,
    displays: HashMap<vk::DisplayKHR, Display>,
    //windows: HashMap<WindowHandle, Window>,
    // / The windows that have been damaged and therefore must be signalled/waited when the window draw commands are submitted
    //damaged: HashSet<WindowHandle>
}
impl Device {
    fn new(vulkan: &VulkanHandle, physical_device: vk::PhysicalDevice) -> Result<Self> {
        // TODO: physical device groups
        let extensions = required_device_extensions();
        let physical_device_properties = unsafe { vulkan.instance.get_physical_device_properties(physical_device) };

        let queue_family_properties = unsafe { vulkan.instance.get_physical_device_queue_family_properties(physical_device) };
        let mut graphics_queue_family = None;
        let mut transfer_queue_family = None;
        for (queue_family, properties) in queue_family_properties.iter().enumerate() {
            if graphics_queue_family.is_none() && properties.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                graphics_queue_family = Some(queue_family as u32)
            }
            // Prefer a dedicated transfer queue
            if (transfer_queue_family.is_none() || !properties.queue_flags.contains(vk::QueueFlags::GRAPHICS)) && properties.queue_flags.contains(vk::QueueFlags::TRANSFER) {
                transfer_queue_family = Some(queue_family as u32)
            }
        }
        let graphics_queue_family = graphics_queue_family.ok_or(Error::MissingSupport("device does not support graphics operations"))?;
        let mut present_queues = HashSet::new();

        let mut displays = HashMap::new();
        let display_properties = unsafe { vulkan.display_khr.get_physical_device_display_properties(physical_device) }?;
        for display_properties in display_properties {
            displays.insert(display_properties.display, Display::new(vulkan, physical_device, display_properties)?);
        }
        let display_plane_properties = unsafe { vulkan.display_khr.get_physical_device_display_plane_properties(physical_device) }?;
        for (plane, display_plane_property) in display_plane_properties.into_iter().enumerate() {
            if display_plane_property.current_display != vk::DisplayKHR::null() {
                if let Some(display) = displays.get_mut(&display_plane_property.current_display) {
                    if display.surface.is_none() {
                        display.surface = Some(Surface::new(vulkan, physical_device, &display.display, graphics_queue_family, &mut present_queues, &queue_family_properties, plane as u32)?);
                        continue // No need to check for all supported planes
                    }
                }
            }
            let supported_displays = unsafe { vulkan.display_khr.get_display_plane_supported_displays(physical_device, plane as _) }?;
            for supported in supported_displays {
                if let Some(display) = displays.get_mut(&supported) {
                    if display.surface.is_none() {
                        display.surface = Some(Surface::new(vulkan, physical_device, &display.display, graphics_queue_family, &mut present_queues, &queue_family_properties, plane as u32)?);
                        break // Don't assign this plane more than once
                    }
                }
            }
        }
        if present_queues.remove(&graphics_queue_family) {
            panic!("Implementation error: graphics queue requested as a present queue")
        }
        if let Some(transfer_queue_family) = transfer_queue_family {
            if present_queues.remove(&transfer_queue_family) {
                panic!("Unhandled edge case: transfer queue family and present queue families overlap")
            }
        }

        let queue_priorities = [1.0];
        let mut queue_create_infos = Vec::new();
        queue_create_infos.push(
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(graphics_queue_family)
                .queue_priorities(&queue_priorities)
                .build()
        );
        if let Some(transfer_queue_family) = transfer_queue_family {
            queue_create_infos.push(
                vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(transfer_queue_family)
                    .queue_priorities(&queue_priorities)
                    .build()
            )
        }
        let present_queues_ordered: Vec<_> = present_queues.into_iter().collect();
        for &present_queue in present_queues_ordered.iter() {
            queue_create_infos.push(
                vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(present_queue)
                    .queue_priorities(&queue_priorities)
                    .build()
            )
        }
        let create_info = vk::DeviceCreateInfo::builder()
            .enabled_extension_names(&extensions)
            .queue_create_infos(&queue_create_infos);
        let device = unsafe { vulkan.instance.create_device(physical_device, &create_info, None) }?;
        let destroy = &mut || unsafe { device.destroy_device(None) };
        let swapchain_khr = khr::Swapchain::new(&vulkan.instance, &device);

        let mut graphics_queue = cleanup(destroy, Queue::new(&device, graphics_queue_family, unsafe { device.get_device_queue(graphics_queue_family, 0) }))?;
        let destroy = &mut || {
            graphics_queue.destroy(&device);
            destroy();
        };

        let mut transfer_queue = if let Some(queue_family) = transfer_queue_family {
            let queue = cleanup(destroy, Queue::new(&device, queue_family, unsafe { device.get_device_queue(queue_family, 0) }))?;
            Some(queue)
        } else {
            None
        };
        let destroy = &mut || {
            if let Some(transfer_queue) = transfer_queue.as_mut() {
                transfer_queue.destroy(&device)
            }
            destroy();
        };

        let mut present_queues: HashMap<u32, Queue> = HashMap::new();
        let destroy = &mut |present_queues: &mut HashMap<u32, Queue>| {
            for present_queue in present_queues.values_mut() {
                present_queue.destroy(&device)
            }
            destroy();
        };
        for present_queue in present_queues_ordered {
            let queue = cleanup(&mut || destroy(&mut present_queues), Queue::new(&device, present_queue, unsafe { device.get_device_queue(present_queue, 0) }))?;
            present_queues.insert(present_queue, queue);
        }
        let destroy = &mut || destroy(&mut present_queues);
        let window_shader = cleanup(destroy, shader!("window", &device))?;
        let mut device = DeviceHandle {
            physical_device,
            physical_device_properties,
            swapchain_khr,
            device,
            queue_family_properties,
            graphics_queue,
            present_queues,
            transfer_queue,
            window_shader
        };
        for display in displays.values_mut() {
            if let Some(surface) = &mut display.surface {
                let result = surface.create_swapchain(vulkan, &device, &display.display);
                cleanup(&mut || unsafe {
                    device.window_shader.destroy(&device.device);
                    device.device.destroy_device(None)
                }, result)?;
            } else {
                eprintln!("Implementation error: failed to designate display {:?} a display plane", display.name())
            }
        }
        Ok(Self {
            device,
            displays
        })
    }
    fn destroy(&mut self, vulkan: &VulkanHandle) {
        unsafe {
            self.device.device.device_wait_idle().ok();
            for display in self.displays.values_mut() {
                display.destroy(vulkan, &self.device)
            }
            self.device.window_shader.destroy(&self.device.device);
            self.device.graphics_queue.destroy(&self.device.device);
            if let Some(transfer_queue) = self.device.transfer_queue.as_mut() {
                transfer_queue.destroy(&self.device.device);
            }
            for present_queue in self.device.present_queues.values_mut() {
                present_queue.destroy(&self.device.device);
            }
            self.device.device.destroy_device(None);
        }
    }
}
impl Deref for Device {
    type Target = DeviceHandle;
    fn deref(&self) -> &Self::Target {
        &self.device
    }
}
impl DerefMut for Device {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.device
    }
}

pub type DeviceId = [u8; vk::UUID_SIZE];
/// Represents an available device that clients may draw to
/// 
/// May represent multiple physical devices in a group with shared memory
pub struct DeviceHandle {
    physical_device: vk::PhysicalDevice,
    physical_device_properties: vk::PhysicalDeviceProperties,
    device: ash::Device,
    swapchain_khr: khr::Swapchain,
    queue_family_properties: Vec<vk::QueueFamilyProperties>,
    graphics_queue: Queue,
    present_queues: HashMap<u32, Queue>,
    transfer_queue: Option<Queue>,
    window_shader: ShaderModule
}

pub struct Vulkan {
    vulkan: VulkanHandle,
    devices: HashMap<DeviceId, Device>
}
impl Vulkan {
    pub fn new() -> Result<Self> {
        let extensions = required_instance_extensions();
        let entry = Entry::linked();
        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 0, 0),
            p_application_name: APP_NAME,
            application_version: vk::make_api_version(0, env_int!("CARGO_PKG_VERSION_MAJOR"), env_int!("CARGO_PKG_VERSION_MINOR"), env_int!("CARGO_PKG_VERSION_PATCH")),
            ..Default::default()
        };
        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&extensions)
            .enabled_layer_names(VALIDATION_LAYERS);
        let instance = unsafe { entry.create_instance(&create_info, None) }?;

        let display_khr = khr::Display::new(&entry, &instance);
        let surface_khr = khr::Surface::new(&entry, &instance);
        
        let mut devices = Self {
            vulkan: VulkanHandle {
                entry,
                instance,
                display_khr,
                surface_khr
            },
            devices: HashMap::new()
        };
        devices.scan()?;
        Ok(devices)
    }
    pub fn scan(&mut self) -> Result<()> {
        for physical_device in unsafe { self.instance.enumerate_physical_devices() }? {
            let properties = unsafe { self.instance.get_physical_device_properties(physical_device) };
            if !self.devices.contains_key(&properties.pipeline_cache_uuid) {
                // Create and add the device if it supports everything required
                if let Ok(device) = Device::new(self, physical_device) {
                    self.devices.insert(properties.pipeline_cache_uuid, device);
                } 
            }
        }
        Ok(())
    }
    pub fn draw_all(&mut self) -> Result<()> {
        for device in self.devices.values_mut() {
            for display in device.displays.values_mut() {
                if let Some(surface) = display.surface.as_mut() {
                    if let Some(swapchain) = surface.swapchain.as_mut() {
                        unsafe {
                            let create_info = vk::SemaphoreCreateInfo::builder();
                            let available_semaphore = device.device.device.create_semaphore(&create_info, None)?;
                            let render_semaphore = device.device.device.create_semaphore(&create_info, None)?;
                            let create_info = vk::FenceCreateInfo::builder();
                            let fence = device.device.device.create_fence(&create_info, None)?;
                            let (i, _) = device.device.swapchain_khr.acquire_next_image(swapchain.swapchain, -1i64 as u64, available_semaphore, fence)?;
                            let frame = &swapchain.frames[i as usize];
                            let begin_info = vk::CommandBufferBeginInfo::builder()
                                .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
                            device.device.device.begin_command_buffer(frame.command_buffer, &begin_info)?;
                            let begin_info = vk::RenderPassBeginInfo::builder()
                                .clear_values(&[vk::ClearValue {
                                    color: vk::ClearColorValue {
                                        float32: [1.0, 0.0, 0.0, 1.0]
                                    }
                                }])
                                .render_area(vk::Rect2D {
                                    offset: vk::Offset2D::default(),
                                    extent: swapchain.extent
                                })
                                .render_pass(swapchain.renderpass)
                                .framebuffer(frame.framebuffer);
                            device.device.device.cmd_begin_render_pass(frame.command_buffer, &begin_info, vk::SubpassContents::INLINE);
                            device.device.device.cmd_end_render_pass(frame.command_buffer);
                            device.device.device.end_command_buffer(frame.command_buffer)?;
                            
                            let command_buffer = [frame.command_buffer];
                            let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
                            let wait_semaphores = [available_semaphore];
                            let submits = [vk::SubmitInfo::builder()
                                .command_buffers(&command_buffer)
                                .wait_dst_stage_mask(&wait_stages)
                                .wait_semaphores(&wait_semaphores)
                                .build()
                            ];
                            device.device.device.queue_submit(device.device.graphics_queue.queue, &submits, vk::Fence::null())?;

                            let images = [i];
                            let swapchains = [swapchain.swapchain];
                            let present_info = vk::PresentInfoKHR::builder()
                                .image_indices(&images)
                                .swapchains(&swapchains);
                            device.device.swapchain_khr.queue_present(device.device.graphics_queue.queue, &present_info)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
impl Deref for Vulkan {
    type Target = VulkanHandle;
    fn deref(&self) -> &Self::Target {
        &self.vulkan
    }
}
impl DerefMut for Vulkan {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vulkan
    }
}
impl Drop for Vulkan {
    fn drop(&mut self) {
        for device in self.devices.values_mut() {
            device.destroy(&self.vulkan)
        }
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
pub struct VulkanHandle {
    #[allow(unused)]
    entry: Entry,
    instance: Instance,
    display_khr: khr::Display,
    surface_khr: khr::Surface
}

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    MissingSupport(&'static str),
    Vulkan(vk::Result),
    ShaderLoad(&'static str, std::io::Error)
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSupport(msg) => write!(f, "Not all required features are present, {}", msg),
            Self::Vulkan(e) => write!(f, "Vulkan Error, {}", e),
            Self::ShaderLoad(name, error) => write!(f, "Error loading shader {:?}, {}", name, error)
        }
    }
}
impl From<vk::Result> for Error {
    fn from(r: vk::Result) -> Self {
        match r {
            vk::Result::ERROR_EXTENSION_NOT_PRESENT => Self::MissingSupport("the required Vulkan extensions are not present"),
            vk::Result::ERROR_FEATURE_NOT_PRESENT => Self::MissingSupport("the required Vulkan features are not present"),
            _ => Self::Vulkan(r)
        }
    }
}
fn cleanup<F: FnMut(), T, E: Into<Error>>(f: &mut F, result: std::result::Result<T, E>) -> Result<T> {
    match result {
        Ok(t) => Ok(t),
        Err(e) => {
            f();
            Err(e.into())
        }
    }
}
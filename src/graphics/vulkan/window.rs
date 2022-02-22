use std::{fmt, collections::{HashMap, HashSet}, ops::{Deref, DerefMut}};
use ash::{vk, extensions::khr, Entry, Instance};

use super::*;

/// The framebuffer and associated resources for a display region
pub struct Window {
    surface: Image,
    /// Waited on when ready for drawing to the window
    ready_semaphore: vk::Semaphore,
    /// Signalled when the window has done drawing
    done_semaphore: vk::Semaphore,
}
impl Window {
    /*fn new(device: &DeviceHandle, vk::Extent2D { width, height }: vk::Extent2D, format: vk::Format) -> Result<Self> {
        let create_info = vk::ImageCreateInfo::builder()
            .array_layers(1)
            .extent(vk::Extent3D { width, height, depth: 0 })
            .format(format)
            .image_type(vk::ImageType::TYPE_2D)
            .initial_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL);
        let image = unsafe { device.device.create_image(&create_info, None) }?;
        let destroy = &mut || unsafe { device.device.destroy_image(image, None) };
        let view = cleanup(destroy, unsafe { device.device.create_image_view(&create_info, None) })?;
        let surface = Image {
            image,
            view
        };
        Ok(Self {
            surface
        })
    }*/
}
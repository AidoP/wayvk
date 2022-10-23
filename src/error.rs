use std::fmt;


pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Vulkan(ash::vk::Result),
    Drm(drm::Error),
    /// The Vulkan device does not support `VK_EXT_physical_device_drm` or did not provide DRM device major/minors.
    NoDrm(String),
    /// The Vulkan instance is missing required extension
    NoExtensions(Vec<String>),
    /// The Vulkan device is missing required extension
    DeviceNoExtensions(String, Vec<String>),
    /// The Vulkan device is not capable of presenting to a screen
    DeviceNoPresent(String),
    /// Unable to prepare a display for presentation as it has no valid modes
    NoDisplayModes,
    /// Unable to prepare a display for presentation as it has no valid planes
    NoDisplayPlanes,
    /// The Vulkan surface cannot be presented to a screen
    SurfaceNoPresent,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl From<ash::vk::Result> for Error {
    fn from(result: ash::vk::Result) -> Self {
        Self::Vulkan(result)
    }
}

impl From<drm::Error> for Error {
    fn from(result: drm::Error) -> Self {
        Self::Drm(result)
    }
}
use std::fmt;
use config::prelude::*;

mod vulkan;

#[derive(Config)]
pub enum GraphicsBackend {
    Vulkan,
}
impl GraphicsBackend {
    pub fn new(&self) -> Result<Backend> {
        match self {
            Self::Vulkan => Ok(Backend::Vulkan(vulkan::Vulkan::new()?))
        }
    }
}
#[derive(Config)]
pub struct Config {
    pub backend: GraphicsBackend
}
impl Default for Config {
    fn default() -> Self {
        Self {
            backend: GraphicsBackend::Vulkan
        }
    }
}

pub enum Backend {
    Vulkan(vulkan::Vulkan)
}

/// The graphics subsystem capable of displaying windows and coordinating display devices
pub trait Graphics {
    type Device: Device;
    type Display: Display;
    type Window: Window;
    /// Return the available graphics devices
    fn devices(&self) -> Vec<&Self::Device>;
}

pub trait Device {
    type Graphics: Graphics;
    fn name(&self) -> String;
    fn kind(&self) -> DeviceKind;
    /// Return the available displays
    fn displays(&self) -> Vec<&<Self::Graphics as Graphics>::Display>;
    /// Create a new window surface on the device
    fn new_window(&mut self, width: u32, height: u32, format: Format) -> <Self::Graphics as Graphics>::Window;
}

pub enum DeviceKind {
    Dedicated,
    Integrated,
    Software
}
impl fmt::Display for DeviceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Dedicated => write!(f, "Dedicated"),
            Self::Integrated => write!(f, "Integrated"),
            Self::Software => write!(f, "Software Rasterizer")
        }
    }
}

pub trait Display {
    fn name(&self) -> String;
    fn mode(&self) -> Mode;
    fn modes(&self) -> Vec<Mode>;
}

/// A display mode representing the size and refresh rate of a display
pub struct Mode {
    /// The refresh rate of the display, or None if it is a variable refresh rate
    refresh_rate: Option<u32>,
    width: u32,
    height: u32
}
impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{} @ ", self.width, self.height)?;
        if let Some(rr) = self.refresh_rate { write!(f, "{}Hz", rr) } else { write!(f, "Variable Refresh Rate") }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    ARGB8888,
    XRGB8888
}
impl Format {
    fn wayland(&self) -> wl_wayland::server::Format {
        use wl_wayland::server::Format;
        match self {
            Self::ARGB8888 => Format::ARGB8888,
            Self::XRGB8888 => Format::XRGB8888,
        }
    }
}

pub trait Window {
    /// Draw to the window from a raw buffer
    fn draw(&mut self, buffer: &[u8], area: Rect, format: Format);
}
pub struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32
}

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    /// Some other error originating from the Vulkan backend
    Vulkan(vulkan::Error)
}
impl From<vulkan::Error> for Error {
    fn from(error: vulkan::Error) -> Self {
        Self::Vulkan(error)
    }
}
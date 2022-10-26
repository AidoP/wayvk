use std::fmt;
use config::prelude::*;
use wl::server::prelude::*;
use wl_wayland::server::prelude::*;

mod error;
mod vulkan;
mod wayland;
mod xdg_shell;

pub mod common {
    pub use crate::{ error::{ Error, Result }, Wayvk };
}

#[derive(Config)]
pub struct Wayvk {
    graphics: vulkan::Config
}
impl Default for Wayvk {
    fn default() -> Self {
        Self {
            graphics: Default::default()
        }
    }
}

fn main() {
    let config = Wayvk::load("wayvk");
    let mut vulkan = vulkan::Vulkan::new().unwrap();

    loop {
        vulkan.render().unwrap();
    }

    println!("{:#?}", vulkan);

    return;

    //dri::Device::open(path)
    
    let mut listener = EventListener::new().unwrap();
    let mut display = WlDisplay::new(vec![]);
    display.register_global(wayland::Compositor);
    display.register_global(wayland::Subcompositor);
    display.register_global(wayland::Seat);
    display.register_global(wayland::Output);
    display.register_global(xdg_shell::WmBase::new());
    let server = Server::listen(display, DispatchErrorHandler, drop_handler).expect("Unable to create socket");
    listener.register(server).expect("Unable to register the server as an event source");
    listener.start()
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2D<T: Copy + Clone + fmt::Debug + fmt::Display> {
    x: T,
    y: T
}
impl<T: Copy + Clone + fmt::Debug + fmt::Display + Default> Default for Vec2D<T> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default()
        }
    }
}
impl<T: Copy + Clone + fmt::Debug + fmt::Display> fmt::Display for Vec2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


/// A double-buffered object on which writes affect the pending state whilst reads use the commited state
/// 
/// Note: DerefMut is not implemented so that reads from the pending state cannot accidentally be done implicitly
#[derive(Debug, Clone)]
pub struct DoubleBuffer<T: Clone> {
    commited: T,
    pub pending: T
}
impl<T: Clone + Default> DoubleBuffer<T> {
    /// Clones the pending state into the commited state so that future reads see updates up to this point
    pub fn commit(&mut self) {
        self.commited = self.pending.clone()
    }
    /// Access the commited state to modify values that should not be double-buffered
    pub fn commit_mut(&mut self) -> &mut T {
        &mut self.commited
    }
}
impl<T: Clone + Default> Default for DoubleBuffer<T> {
    fn default() -> Self {
        Self {
            commited: T::default(),
            pending: T::default()
        }
    }
}
impl<T: Clone> std::ops::Deref for DoubleBuffer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.commited
    }
}
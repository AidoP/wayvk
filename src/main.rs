use std::fmt;
use config::prelude::*;
use yutani::server::prelude::*;

mod error;
mod vulkan;
mod wayland;

pub mod common {
    pub use crate::{ error::{ Error, Result }, Wayvk };
}

#[derive(Config)]
pub struct Wayvk {
    pub graphics: vulkan::Config
}
impl Default for Wayvk {
    fn default() -> Self {
        Self {
            graphics: Default::default()
        }
    }
}

pub struct State {
    config: Wayvk,
    vulkan: vulkan::Vulkan
}

fn main() {
    let state = State {
        config: Wayvk::load("wayvk"),
        vulkan: vulkan::Vulkan::dummy().unwrap()//::new().unwrap()
    };

    //dri::Device::open(path)
    let path = yutani::find_free_socket();
    syslib::unlink(&path).ok();
    let mut event_loop = Server::event_loop(path, state, wayland::Display::constructor).unwrap();
    loop {
        event_loop.wait(100).unwrap();
    }
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
pub struct DoubleBuffer<T>(T, T);
impl<T: Clone> DoubleBuffer<T> {
    /// Update the commited state with the pending state
    pub fn commit(&mut self) {
        self.0.clone_from(&self.1)
    }
    /// Mutably access the pending state
    pub fn pending(&mut self) -> &mut T {
        &mut self.1
    }
}
impl<T: Default> Default for DoubleBuffer<T> {
    fn default() -> Self {
        Self(T::default(), T::default())
    }
}
impl<T> std::ops::Deref for DoubleBuffer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Clone> Clone for DoubleBuffer<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}
impl<T: Copy> Copy for DoubleBuffer<T> {}
impl<T: fmt::Debug> fmt::Debug for DoubleBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DoubleBuffer")
            .field("commited", &self.0)
            .field("pending", &self.1)
            .finish()
    }
}
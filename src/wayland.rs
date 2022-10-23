use std::{fmt, num::NonZeroU32};

use wl::server::prelude::*;
use wl_wayland::server::{
    error,
    DisplayError,
    BufferAccess,
    wayland,
    WlDisplay as Display,
    WlCallback as Callback,
    WlRegistry as Registry,
    WlShm as Shm,
    WlShmPool as ShmPool,
    WlBuffer as Buffer,

    Global,
    OnBind
};
use crate::{xdg_shell, DoubleBuffer, Vec2D};

pub use protocol::*;

use self::notification::WpNotificationLevel;

#[protocol("protocol/notification.toml")]
pub mod notification {
    use super::Buffer as WlBuffer;
    use super::Surface as WlSurface;
    type WpNotifier = super::Notifier;
    type WpNotification = super::Notification;
}

pub struct Notifier;
impl notification::WpNotifier for Lease<Notifier> {
    fn notify(&mut self, client: &mut Client, id: NewId, title: String, persist: u32) -> Result<()>  {
        todo!()
    }
}

pub struct Notification {
    title: String,
    persistent: bool,
    kind: NotificationKind
}
pub enum NotificationKind {
    None,
    Plain {
        body: String,
        level: NotificationLevel
    },
    Confirmation {
        request: String,
        accept: String,
        decline: String
    },
    Media {
        name: String,
        from: String,
        creator: String,
        length: u32,
        current: u32
    },
    Surface {
        surface: Surface
    }
}
pub enum NotificationLevel {
    Information,
    Warning,
    Critical,
    Logging
}
impl NotificationLevel {
    fn new(raw: u32) -> Result<Self> {
        match raw {
            WpNotificationLevel::INFORMATIONAL => Ok(Self::Information),
            WpNotificationLevel::WARNING => Ok(Self::Warning),
            WpNotificationLevel::CRITICAL => Ok(Self::Critical),
            WpNotificationLevel::LOGGING => Ok(Self::Logging),
            v => Err(DisplayError::method(&0, format!("{} is not a valid notification level", v)))
        }
    }
}
impl notification::WpNotification for Lease<Notification> {
    fn set_title(&mut self, client: &mut Client) -> Result<()>  {
        todo!()
    }
    fn set_image(&mut self, client: &mut Client, file: std::fs::File) -> Result<()>  {
        todo!()
    }
    fn set_buffer(&mut self, client: &mut Client, buffer: Lease<Buffer>) -> Result<()>  {
        todo!()
    }
    fn set_plain(&mut self, client: &mut Client, description: String, level: u32) -> Result<()>  {
        todo!()
    }
    fn set_confirm(&mut self, client: &mut Client, request: String, accept: String, decline: String, level: u32) -> Result<()>  {
        todo!()
    }
    fn set_media(&mut self, client: &mut Client, name: String, from: String, creator: String, length: u32) -> Result<()>  {
        todo!()
    }
    fn set_current_timestamp(&mut self, client: &mut Client, timestamp: u32) -> Result<()>  {
        todo!()
    }
    fn set_input(&mut self, client: &mut Client, request: String, submit: String) -> Result<()>  {
        todo!()
    }
    fn set_surface(&mut self, client: &mut Client, surface: Lease<Surface>) -> Result<()>  {
        todo!()
    }
}

#[protocol("protocol/wayland.toml")]
pub mod protocol {
    use super::Display as WlDisplay;
    use super::Callback as WlCallback;
    use super::Registry as WlRegistry;
    use super::Shm as WlShm;
    use super::ShmPool as WlShmPool;
    use super::Buffer as WlBuffer;

    type WlCompositor = super::Compositor;
    type WlSurface = super::Surface;
    type WlRegion = super::Region;
    type WlSubcompositor = super::Subcompositor;
    type WlSubsurface = super::Subsurface;
    type WlOutput = super::Output;

    type WlSeat = super::Seat;
    type WlKeyboard = super::Keyboard;
    type WlPointer = super::Pointer;
    type WlTouch = super::Touch;
}

#[derive(Global, Clone, Copy)]
pub struct Compositor;
impl OnBind for Compositor {}
impl WlCompositor for Lease<Compositor> {
    fn create_surface(&mut self, client: &mut Client, id: NewId) -> Result<()> {
        client.insert(id, Surface::new())?;
        Ok(())
    }
    fn create_region(&mut self, client: &mut Client, id: NewId) -> Result<()> {
        client.insert(id, Region)?;
        Ok(())
    }
}
error! {
    SurfaceError {
        DeprecatedOffset = WlSurfaceError::INVALID_OFFSET => deprecated_offset {
            "specifying a non-zero offset is deprecated"
        },
        InvalidOffset = WlSurfaceError::INVALID_OFFSET => offset {
            "off"
        }
    }
}
#[derive(Clone)]
pub struct Surface {
    /// The associated xdg_surface
    pub xdg_surface: Option<NonZeroU32>,
    buffer: DoubleBuffer<Option<NonZeroU32>>
}
impl Surface {
    fn new() -> Self {
        Self {
            xdg_surface: None,
            buffer: Default::default()
        }
    }
}
impl WlSurface for Lease<Surface> {
    fn destroy(&mut self, client: &mut Client) -> Result<()> {
        client.delete(self)
    }
    fn attach(&mut self, client: &mut Client, buffer: Nullable<Lease<Buffer>>, x: i32, y: i32) -> Result<()> {
        if self.version() >= 5 && (x != 0 || y != 0) {
            return Err(SurfaceError::deprecated_offset(self))
        } else if self.version() < 5 {
            todo!()
        }
        self.buffer.pending = buffer.option_object();
        Ok(())
    }
    fn damage(&mut self, client: &mut Client, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        //todo!()
        Ok(())
    }
    fn frame(&mut self, client: &mut Client, callback: NewId) -> Result<()> {
        use wayland::WlCallback;
        // TODO: signal when a good time would be
        //let mut display = Display::get(client)?;
        //display.delete_id(client, callback.object())?;
        let mut callback = client.insert(callback, Callback)?;
        callback.done(client, 0)?;
        client.delete(&callback)?;
        Ok(())
    }
    fn set_opaque_region(&mut self, client: &mut Client, region: Nullable<Lease<Region>>) -> Result<()> {
        todo!()
    }
    fn set_input_region(&mut self, client: &mut Client, region: Nullable<Lease<Region>>) -> Result<()> {
        todo!()
    }
    fn commit(&mut self, client: &mut Client) -> Result<()> {
        if let Some(xdg_surface) = self.xdg_surface {
            let xdg_surface: Lease<xdg_shell::Surface> = client.get(xdg_surface.get())?;
            if let xdg_shell::SurfaceRole::Toplevel(toplevel) = xdg_surface.role {
                let mut toplevel: Lease<xdg_shell::Toplevel> = client.get(toplevel)?;
                toplevel.size_bounds.commit();
                if toplevel.size_bounds.invalid() {
                    return Err(DisplayError::method(self, "requested minimum size of an xdg_toplevel cannot exceed the maximum".into()))
                }
                // TODO: Request resize if current surface is larger than bounds
            }
        }
        self.buffer.commit();
        if let Some(buffer) = *self.buffer {
            let mut buffer: Lease<Buffer> = client.get(buffer.get())?;

            // TODO: Once vulkan finishes drawing
            // buffer.release(client)?;
            // *self.buffer.commit_mut() = None;
        }
        Ok(())
    }
    fn set_buffer_transform(&mut self, client: &mut Client, transform: i32) -> Result<()> {
        todo!()
    }
    fn set_buffer_scale(&mut self, client: &mut Client, scale: i32) -> Result<()> {
        todo!()
    }
    fn damage_buffer(&mut self, client: &mut Client, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        todo!()
    }
    fn offset(&mut self, client: &mut Client, x: i32, y: i32) -> Result<()> {
        todo!()
    }
}
pub struct Region;
impl WlRegion for Lease<Region> {
    fn destroy(&mut self, client: &mut Client) -> Result<()> {
        client.delete(self)
    }
    fn add(&mut self, client: &mut Client, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        todo!()
    }
    fn subtract(&mut self, client: &mut Client, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        todo!()
    }
}

#[derive(Global, Clone, Copy)]
pub struct Subcompositor;
impl OnBind for Subcompositor {}
impl WlSubcompositor for Lease<Subcompositor> {
    fn destroy(&mut self, client: &mut Client) -> Result<()> {
        client.delete(self)
    }
    fn get_subsurface(&mut self, client: &mut Client, id: NewId, surface: Lease<Surface>, parent: Lease<Surface>) -> Result<()> {
        client.insert(id, Subsurface::new(surface.object(), parent.object()))?;
        Ok(())
    }
}
pub struct Subsurface {
    surface: u32,
    parent: u32
}
impl Subsurface {
    pub fn new(surface: u32, parent: u32) -> Self {
        Self {
            surface,
            parent
        }
    }
}
impl WlSubsurface for Lease<Subsurface> {
    fn destroy(&mut self, client: &mut Client) -> Result<()> {
        client.delete(self)
    }
    fn set_position(&mut self, client: &mut Client, x: i32, y: i32) -> Result<()> {
        todo!()
    }
    fn place_above(&mut self, client: &mut Client, sibling: Lease<Surface>) -> Result<()> {
        todo!()
    }
    fn place_below(&mut self, client: &mut Client, sibling: Lease<Surface>) -> Result<()> {
        todo!()
    }
    fn set_sync(&mut self, client: &mut Client) -> Result<()> {
        todo!()
    }
    fn set_desync(&mut self, client: &mut Client) -> Result<()> {
        todo!()
    }
}
#[derive(Global, Clone, Copy)]
pub struct Output;
impl OnBind for Output {}
impl WlOutput for Lease<Output> {
    fn release(&mut self, client: &mut Client) -> Result<()> {
        client.delete(self)
    }
}

#[derive(Global, Clone, Copy)]
pub struct Seat;
impl OnBind for Seat {}
impl WlSeat for Lease<Seat> {
    fn release(&mut self, client: &mut Client) -> Result<()> {
        client.delete(self)
    }
    fn get_keyboard(&mut self, client: &mut Client, id: NewId) -> Result<()> {
        todo!()
    }
    fn get_pointer(&mut self, client: &mut Client, id: NewId) -> Result<()> {
        todo!()
    }
    fn get_touch(&mut self, client: &mut Client, id: NewId) -> Result<()> {
        todo!()
    }
}
pub struct Keyboard;
impl WlKeyboard for Lease<Keyboard> {
    fn release(&mut self, client: &mut Client) -> Result<()> {
        client.delete(self)
    }
}
pub struct Pointer;
impl WlPointer for Lease<Pointer> {
    fn release(&mut self, client: &mut Client) -> Result<()> {
        client.delete(self)
    }
    fn set_cursor(&mut self, client: &mut Client, serial: u32, surface: Nullable<Lease<Surface>>, hotspot_x: i32, hotspot_y: i32) -> Result<()> {
        todo!()
    }
}
pub struct Touch;
impl WlTouch for Lease<Touch> {
    fn release(&mut self, client: &mut Client) -> Result<()> {
        client.delete(self)
    }
}
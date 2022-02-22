use std::{time::Instant, num::NonZeroU32};

use wl::server::prelude::*;
use wl_wayland::server::{
    error,
    DisplayError,
    WlDisplay as Display,
    WlCallback as Callback,
    WlRegistry as Registry,
    WlShm as Shm,
    WlShmPool as ShmPool,
    WlBuffer as Buffer,

    Global,
    OnBind
};
use crate::{wayland, DoubleBuffer, Vec2D};

pub use protocol::*;

#[protocol("protocol/xdg-shell.toml")]
pub mod protocol {
    use super::Display as WlDisplay;
    use super::Callback as WlCallback;
    use super::Registry as WlRegistry;
    use super::Shm as WlShm;
    use super::ShmPool as WlShmPool;
    use super::Buffer as WlBuffer;

    use super::wayland::Compositor as WlCompositor;
    use super::wayland::Surface as WlSurface;
    use super::wayland::Region as WlRegion;
    use super::wayland::Subcompositor as WlSubcompositor;
    use super::wayland::Subsurface as WlSubsurface;
    use super::wayland::Output as WlOutput;
    use super::wayland::Seat as WlSeat;

    type XdgWmBase = super::WmBase;
    type XdgToplevel = super::Toplevel;
    type XdgPopup = super::Popup;
    type XdgPositioner = super::Positioner;
    type XdgSurface = super::Surface;
}

error!{
    WmBaseError {
        Role = XdgWmBaseError::ROLE => role {
            "the associated wl_surface is already assigned a role"
        },
        Defunct = XdgWmBaseError::DEFUNCT_SURFACES => defunct {
            "the xdg_wm_base object was destroyed before its children surfaces"
        },
        NotTopmostPopup = XdgWmBaseError::NOT_THE_TOPMOST_POPUP => not_topmost_popup {
            "the popup is not the topmost popup in the popup stack"
        },
        InvalidPopupParent = XdgWmBaseError::INVALID_POPUP_PARENT => popup_parent {
            "the parent is not a valid popup parent surface"
        },
        InvalidSurfaceState = XdgWmBaseError::INVALID_SURFACE_STATE => surface {
            "wl_surface@{surface} is in an invalid state",
            surface: u32
        },
        InvalidPositioner = XdgWmBaseError::INVALID_POSITIONER => positioner {
            "wl_positioner@{positioner} is not valid",
            positioner: u32
        }
    }
}
#[derive(Global)]
pub struct WmBase {
    /// The last moment that a pong request was received from the client
    last_pong: Instant
}
impl OnBind for WmBase {}
impl WmBase {
    pub fn new() -> Self {
        Self {
            last_pong: Instant::now()
        }
    }
}
impl Clone for WmBase {
    fn clone(&self) -> Self {
        Self {
            last_pong: Instant::now()
        }
    }
}
impl XdgWmBase for Lease<WmBase> {
    fn destroy(&mut self, client: &mut Client) -> Result<()> {
        client.delete(self)
    }
    fn create_positioner(&mut self, client: &mut Client, id: NewId) -> Result<()> {
        client.insert(id, Positioner)?;
        Ok(())
    }
    fn get_xdg_surface(&mut self, client: &mut Client, id: NewId, mut surface: Lease<wayland::Surface>) -> Result<()> {
        let xdg_surface = client.insert(id, Surface::new(&mut surface))?;
        surface.xdg_surface = NonZeroU32::new(xdg_surface.object());
        Ok(())
    }
    fn pong(&mut self, _: &mut Client, _serial: u32) -> Result<()> {
        self.last_pong = Instant::now();
        Ok(())
    }
}
#[derive(Debug, Copy, Clone, Default)]
pub struct SizeBounds {
    pub min: Vec2D<u32>,
    pub max: Vec2D<u32>
}
impl SizeBounds {
    /// Returns true if the bounds are invalid, that is, the minimum is larger than the maximum
    pub fn invalid(&self) -> bool {
        //(self.min.x != 0 && self.min.x > self.max.x) || (self.min.y != 0 && self.min.y > self.max.y) ||
        //(self.min.x == 0 && self.max.x != 0) || (self.min.y == 0 && self.max.y != 0)
        false
    }
}
pub struct Toplevel {
    surface: u32,
    pub parent: Option<NonZeroU32>,
    pub title: String,
    pub app_id: String,
    pub size_bounds: DoubleBuffer<SizeBounds>
}
impl Toplevel {
    pub fn new(surface: &mut Lease<Surface>) -> Self {
        Self {
            surface: surface.object(),
            parent: None,
            title: "Untitled Window".into(),
            app_id: "unknown".into(),
            size_bounds: Default::default()
        }
    }
}
impl XdgToplevel for Lease<Toplevel> {
    fn destroy(&mut self, client: &mut Client) -> Result<()> {
        let mut parent: Lease<Surface> = client.get(self.surface)?;
        parent.role = SurfaceRole::None;
        client.delete(self)
    }
    fn set_parent(&mut self, _: &mut Client, parent: Nullable<Lease<Toplevel>>) -> Result<()> {
        self.parent = parent.option_object();
        Ok(())
    }
    fn set_title(&mut self, _: &mut Client, title: std::string::String) -> Result<()> {
        self.title = title;
        Ok(())
    }
    fn set_app_id(&mut self, _: &mut Client, app_id: std::string::String) -> Result<()> {
        self.app_id = app_id;
        Ok(())
    }
    fn show_window_menu(&mut self, client: &mut Client, seat: Lease<wayland::Seat>, serial: u32, x: i32, y: i32) -> Result<()> {
        todo!()
    }
    fn r#move(&mut self, client: &mut Client, seat: Lease<wayland::Seat>, serial: u32) -> Result<()> {
        todo!()
    }
    fn resize(&mut self, client: &mut Client, seat: Lease<wayland::Seat>, serial: u32, edges: u32) -> Result<()> {
        todo!()
    }
    fn set_max_size(&mut self, _: &mut Client, width: i32, height: i32) -> Result<()> {
        if width < 0 || height < 0 {
            return Err(DisplayError::method(self, "width and height of xdg_toplevel.set_max_size must be positive".into()))
        }
        self.size_bounds.pending.max = Vec2D { x: width as _, y: height as _ };
        Ok(())
    }
    fn set_min_size(&mut self, _: &mut Client, width: i32, height: i32) -> Result<()> {
        if width < 0 || height < 0 {
            return Err(DisplayError::method(self, "width and height of xdg_toplevel.set_min_size must be positive".into()))
        }
        self.size_bounds.pending.min = Vec2D { x: width as _, y: height as _ };
        Ok(())
    }
    fn set_maximized(&mut self, client: &mut Client) -> Result<()> {
        todo!()
    }
    fn unset_maximized(&mut self, client: &mut Client) -> Result<()> {
        todo!()
    }
    fn set_fullscreen(&mut self, client: &mut Client, output: Nullable<Lease<wayland::Output>>) -> Result<()> {
        todo!()
    }
    fn unset_fullscreen(&mut self, client: &mut Client) -> Result<()> {
        todo!()
    }
    fn set_minimized(&mut self, client: &mut Client) -> Result<()> {
        todo!()
    }
}
pub struct Popup {
    surface: u32,
    parent: Option<NonZeroU32>,
    positioner: u32
}
impl Popup {
    pub fn new(surface: &mut Lease<Surface>, parent: &mut Nullable<Lease<Surface>>, positioner: &mut Lease<Positioner>) -> Self {
        Self {
            surface: surface.object(),
            parent: parent.option_object(),
            positioner: positioner.object()
        }
    }
}
impl XdgPopup for Lease<Popup> {
    fn destroy(&mut self, client: &mut Client) -> Result<()> {
        let mut surface: Lease<Surface> = client.get(self.surface)?;
        surface.role = SurfaceRole::None;
        client.delete(self)
    }
    fn grab(&mut self, client: &mut Client, seat: Lease<wayland::Seat>, serial: u32) -> Result<()> {
        todo!()
    }
    fn reposition(&mut self, client: &mut Client, positioner: Lease<Positioner>, token: u32) -> Result<()> {
        todo!()
    }
}
pub struct Positioner;
impl XdgPositioner for Lease<Positioner> {
    fn destroy(&mut self, client: &mut Client) -> Result<()> {
        client.delete(self)
    }
    fn set_size(&mut self, client: &mut Client, width: i32, height: i32) -> Result<()> {
        todo!()
    }
    fn set_anchor_rect(&mut self, client: &mut Client, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        todo!()
    }
    fn set_anchor(&mut self, client: &mut Client, anchor: u32) -> Result<()> {
        todo!()
    }
    fn set_gravity(&mut self, client: &mut Client, gravity: u32) -> Result<()> {
        todo!()
    }
    fn set_constraint_adjustment(&mut self, client: &mut Client, constraint_adjustment: u32) -> Result<()> {
        todo!()
    }
    fn set_offset(&mut self, client: &mut Client, x: i32, y: i32) -> Result<()> {
        todo!()
    }
    fn set_reactive(&mut self, client: &mut Client) -> Result<()> {
        todo!()
    }
    fn set_parent_size(&mut self, client: &mut Client, width: i32, height: i32) -> Result<()> {
        todo!()
    }
    fn set_parent_configure(&mut self, client: &mut Client, serial: u32) -> Result<()> {
        todo!()
    }
}
error!{
    SurfaceError {
        NotConstructed = XdgSurfaceError::NOT_CONSTRUCTED => not_constructed {
            "not constructed"
        },
        AlreadyConstructed = XdgSurfaceError::ALREADY_CONSTRUCTED => already_constructed {
            "already constructed"
        },
        UnconfiguredBuffer = XdgSurfaceError::UNCONFIGURED_BUFFER => unconfigured_buffer {
            "buffer is unconfigured"
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32
}
pub struct Surface {
    /// The associated wl_surface
    surface: u32,
    pub window_geometry: Option<WindowGeometry>,
    pub role: SurfaceRole
}
impl Surface {
    fn new(surface: &mut Lease<wayland::Surface>) -> Self {
        Self {
            surface: surface.object(),
            window_geometry: None,
            role: SurfaceRole::None
        }
    }
}
impl XdgSurface for Lease<Surface> {
    fn destroy(&mut self, client: &mut Client) -> Result<()> {
        if self.role.is_some() {
            return Err(WmBaseError::role(self))
        }
        let mut surface: Lease<wayland::Surface> = client.get(self.surface)?;
        surface.xdg_surface = None;
        client.delete(self)
    }
    fn get_toplevel(&mut self, client: &mut Client, id: NewId) -> Result<()> {
        if self.role.is_some() {
            Err(WmBaseError::role(self))
        } else {
            let toplevel = client.insert(id, Toplevel::new(self))?;
            self.role = SurfaceRole::Toplevel(toplevel.object());
            Ok(())
        }
    }
    fn get_popup(&mut self, client: &mut Client, id: NewId, mut parent: Nullable<Lease<Surface>>, mut positioner: Lease<Positioner>) -> Result<()> {
        if self.role.is_some() {
            Err(WmBaseError::role(self))
        } else {
            let popup = client.insert(id, Popup::new(self, &mut parent, &mut positioner))?;
            self.role = SurfaceRole::Popup(popup.object());
            Ok(())
        }
    }
    fn set_window_geometry(&mut self, client: &mut Client, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        if width <= 0 || height <= 0 {
            return Err(DisplayError::method(self, "width and height of xdg_surface.set_window_geometry must be greater than 0".into()))
        }
        self.window_geometry = Some(WindowGeometry {
            x,
            y,
            width: width as _,
            height: height as _ 
        });
        Ok(())
    }
    fn ack_configure(&mut self, client: &mut Client, serial: u32) -> Result<()> {
        todo!()
    }
}
pub enum SurfaceRole {
    None,
    Toplevel(u32),
    Popup(u32)
}
impl SurfaceRole {
    pub fn is_some(&self) -> bool {
        match self {
            Self::None => false,
            _ => true
        }
    }
}
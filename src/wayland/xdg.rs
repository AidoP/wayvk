use std::num::NonZeroU32;

use super::prelude::*;

pub struct WmBase;
impl WmBase {
    pub fn new(client: &mut Client, id: NewId) -> Result<(), WlError<'static>> {
        let object = Self.into_versioned_object(id.id(), id.version())?;
        client.insert(object.into_any())?;

        Ok(())
    }
}
impl XdgWmBase<State> for WmBase {
    fn destroy(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        client.remove(this.id())?;
        Ok(())
    }

    fn create_positioner(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        id: Id,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn get_xdg_surface(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        id: Id,
        wl_surface: Id,
    ) -> Result<(), WlError<'static>> {
        let xdg_surface = Surface::new(wl_surface).into_object(id);
        client.insert(xdg_surface.into_any())
    }

    fn pong(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        serial: u32,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }
}

#[derive(Clone, Default)]
pub struct SurfaceState {
    role: Option<Id>,
    geometry: Option<Geometry>
}
pub struct Surface {
    wl_surface: Id,
    state: DoubleBuffer<SurfaceState>
}
impl Surface {
    pub fn new(wl_surface: Id) -> Self {
        Self {
            wl_surface,
            state: Default::default()
        }
    }
    pub fn commit(&mut self, client: &mut Client) -> Result<(), WlError<'static>> {
        if let Some(role) = self.state.pending().role {
            let object = client.lease(role)?;
            if let Some(mut toplevel) = object.downcast::<Toplevel>() {
                toplevel.commit()
            }
        }
        self.state.commit();
        Ok(())
    }
}
impl XdgSurface<State> for Surface {
    fn destroy(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        if this.state.role.is_some() {
            return Err(WlError::LEAK)
        }
        client.remove(this.id())?;
        Ok(())
    }

    fn get_toplevel(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        id: Id,
    ) -> Result<(), WlError<'static>> {
        let xdg_surface = Toplevel::new().into_object(id);
        client.insert(xdg_surface.into_any())
    }

    fn get_popup(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        id: Id,
        parent: Option<Id>,
        positioner: Id,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn set_window_geometry(
        mut this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), WlError<'static>> {
        if width < 1 || height < 1 {
            Err(WlError::DOMAIN)
        } else {
            this.state.pending().geometry = Some(Geometry {
                x,
                y,
                width: width as u32,
                height: height as u32
            });
            Ok(())
        }
    }

    fn ack_configure(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        serial: u32,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }
}

#[derive(Clone, Default)]
pub struct ToplevelState {
    title: String,
    app_id: String,
    min_size: (Option<NonZeroU32>, Option<NonZeroU32>),
    max_size: (Option<NonZeroU32>, Option<NonZeroU32>)
}
pub struct Toplevel {
    state: DoubleBuffer<ToplevelState>
}
impl Toplevel {
    pub fn new() -> Self {
        Self {
            state: Default::default()
        }
    }
    pub fn commit(&mut self) {
        self.state.commit()
    }
}
impl XdgToplevel<State> for Toplevel {
    fn destroy(
        this: yutani::lease::Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        client.remove(this.id())?;
        Ok(())
    }

    fn set_parent(
        this: yutani::lease::Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        parent: core::option::Option<Id>,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn set_title(
        mut this: yutani::lease::Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        title: std::string::String,
    ) -> Result<(), WlError<'static>> {
        this.state.pending().title = title;
        Ok(())
    }

    fn set_app_id(
        mut this: yutani::lease::Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        app_id: std::string::String,
    ) -> Result<(), WlError<'static>> {
        this.state.pending().app_id = app_id;
        Ok(())
    }

    fn show_window_menu(
        this: yutani::lease::Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        seat: Id,
        serial: u32,
        x: i32,
        y: i32,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn r#move(
        _: yutani::lease::Lease<Self>,
        _: &mut EventLoop,
        _: &mut Client,
        _: Id,
        _: u32,
    ) -> Result<(), WlError<'static>> {
        // Ignored - tiling WM
        Ok(())
    }

    fn resize(
        _: yutani::lease::Lease<Self>,
        _: &mut EventLoop,
        _: &mut Client,
        _: Id,
        _: u32,
        _: u32,
    ) -> Result<(), WlError<'static>> {
        // Ignored - tiling WM
        Ok(())
    }

    fn set_max_size(
        mut this: yutani::lease::Lease<Self>,
        _: &mut EventLoop,
        _: &mut Client,
        width: i32,
        height: i32,
    ) -> Result<(), WlError<'static>> {
        if width < 0 || height < 0 {
            Err(WlError::DOMAIN)
        } else {
            this.state.pending().max_size = (
                NonZeroU32::new(width as u32),
                NonZeroU32::new(height as u32)
            );
            Ok(())
        }
    }

    fn set_min_size(
        mut this: yutani::lease::Lease<Self>,
        _: &mut EventLoop,
        _: &mut Client,
        width: i32,
        height: i32,
    ) -> Result<(), WlError<'static>> {
        if width < 0 || height < 0 {
            Err(WlError::DOMAIN)
        } else {
            this.state.pending().min_size = (
                NonZeroU32::new(width as u32),
                NonZeroU32::new(height as u32)
            );
            Ok(())
        }
    }

    fn set_maximized(
        this: yutani::lease::Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn unset_maximized(
        this: yutani::lease::Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn set_fullscreen(
        this: yutani::lease::Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        output: core::option::Option<Id>,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn unset_fullscreen(
        this: yutani::lease::Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn set_minimized(
        this: yutani::lease::Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }
}
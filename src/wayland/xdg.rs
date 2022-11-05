use super::prelude::*;

pub struct WmBase;
impl proto::XdgWmBase<State> for WmBase {
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
        surface: Id,
    ) -> Result<(), WlError<'static>> {
        todo!()
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

pub mod proto;
use proto::*;

pub use wl::server::prelude::*;

pub struct Display;
impl Display {
    pub fn constructor<T>(event_loop: &mut EventLoop<T>, client: &mut Client<T>, id: Id, version: u32) -> Resident<T> {
        Display.into_versioned_object(id, version).into_any()
    }
}
impl<T> WlDisplay<T> for Display {
    fn sync(
        this: wl::lease::Lease<Self>,
        event_loop: &mut wl::wire::EventLoop<T>,
        client: &mut wl::server::Client<T>,
        callback: wl::Id,
    ) -> core::result::Result<(), wl::wire::WlError<'static>> {
        let serial = client.next_event();
        let stream = client.stream();
        // Send done event to the callback
        let key = stream.start_message(callback, 0);
        stream.send_u32(serial)?;
        stream.commit(key)?;
        // Delete the callback object
        Self::delete_id(this, client, callback.into())
    }

    fn get_registry(
        this: wl::lease::Lease<Self>,
        event_loop: &mut wl::wire::EventLoop<T>,
        client: &mut wl::server::Client<T>,
        registry: wl::Id,
    ) -> core::result::Result<(), wl::wire::WlError<'static>> {
        Ok(())
    }
}
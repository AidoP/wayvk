pub mod proto;
use proto::*;

pub mod wl;
pub mod xdg;

use crate::State;

use yutani::server::prelude::*;
type EventLoop = yutani::EventLoop<State>;
type Client = yutani::server::Client<State>;
type Resident = yutani::server::Resident<State>;

pub mod prelude {
    pub use super::*;
}

pub struct Display;
impl Display {
    pub fn constructor(
        event_loop: &mut EventLoop,
        client: &mut Client,
        id: Id,
        version: u32,
    ) -> Result<Resident, WlError<'static>> {
        Display
            .into_versioned_object(id, version)
            .map(|object| object.into_any())
    }
}
impl WlDisplay<State> for Display {
    fn sync(
        mut this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        callback: Id,
    ) -> Result<(), WlError<'static>> {
        let serial = client.next_event();
        let stream = client.stream();
        // Send done event to the callback
        let key = stream.start_message(callback, 0);
        stream.send_u32(serial)?;
        stream.commit(key)?;
        // Delete the callback object
        Self::delete_id(&mut this, client, callback.into())
    }

    fn get_registry(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        registry: yutani::Id,
    ) -> Result<(), WlError<'static>> {
        let mut object = Registry.into_object(registry);
        let registry = object.lease().unwrap();
        client.insert(object.into_any())?;

        Registry::advertise(registry, event_loop, client)
    }
}

pub struct Registry;
impl Registry {
    pub const WL_COMPOSITOR: u32 = 0;
    pub const WL_SHM: u32 = 1;
    pub const WL_DATA_DEVICE_MANAGER: u32 = 2;
    pub const WL_SUBCOMPOSITOR: u32 = 5;
    pub const XDG_WM_BASE: u32 = 7;
    fn advertise(
        mut this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        Self::global(
            &mut this,
            client,
            Self::WL_COMPOSITOR,
            wl::Compositor::INTERFACE,
            wl::Compositor::VERSION,
        )?;
        Self::global(
            &mut this,
            client,
            Self::WL_SHM,
            wl::Shm::INTERFACE,
            wl::Shm::VERSION,
        )?;
        Self::global(
            &mut this,
            client,
            Self::WL_SUBCOMPOSITOR,
            wl::Subcompositor::INTERFACE,
            wl::Subcompositor::VERSION,
        )?;
        Self::global(
            &mut this,
            client,
            Self::XDG_WM_BASE,
            xdg::WmBase::INTERFACE,
            xdg::WmBase::VERSION,
        )?;
        Ok(())
    }
}
impl WlRegistry<State> for Registry {
    fn bind(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        name: u32,
        id: NewId,
    ) -> Result<(), WlError<'static>> {
        match name {
            Self::WL_COMPOSITOR => wl::Compositor
                .into_versioned_object(id.id(), id.version())
                .and_then(|o| client.insert(o.into_any())),
            Self::WL_SHM => wl::Shm::create(client, id),
            Self::WL_SUBCOMPOSITOR => wl::Subcompositor
                .into_versioned_object(id.id(), id.version())
                .and_then(|o| client.insert(o.into_any())),
            Self::XDG_WM_BASE => xdg::WmBase
                .into_versioned_object(id.id(), id.version())
                .and_then(|o| client.insert(o.into_any())),
            _ => return Err(WlError::NO_GLOBAL),
        }
    }
}
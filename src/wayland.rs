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
    pub use crate::DoubleBuffer;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Geometry {
    x: i32,
    y: i32,
    width: u32,
    height: u32
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
    pub fn delete_id(client: &mut Client, id: Id) -> Result<(), WlError<'static>> {
        let stream = client.stream();
        let key = stream.start_message(Id::DISPLAY, 1);
        stream.send_object(Some(id))?;
        stream.commit(key)
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
        Callback::done(client, callback, serial)
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

pub struct Callback;
impl Callback {
    /// Send the `done` event to a virtual callback object.
    /// The virtual callback object is released for reuse by the client and is therefore invalidated. 
    pub fn done(client: &mut Client, id: Id, data: u32) -> Result<(), WlError<'static>> {
        let stream = client.stream();
        let key = stream.start_message(id, 0);
        stream.send_u32(data)?;
        stream.commit(key)?;
        // Tell the client that it is safe to reuse the callback
        Display::delete_id(client, id)
    }
}

pub struct Registry;
impl Registry {
    pub const WL_COMPOSITOR: u32 = 0;
    pub const WL_SHM: u32 = 1;
    pub const WL_DATA_DEVICE_MANAGER: u32 = 2;
    pub const WL_SEAT: u32 = 3;
    pub const WL_OUTPUT: u32 = 4;
    pub const WL_SUBCOMPOSITOR: u32 = 5;
    pub const XDG_WM_BASE: u32 = 6;
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
            Self::WL_COMPOSITOR => wl::Compositor::new(client, id),
            Self::WL_SHM => wl::Shm::new(client, id),
            Self::WL_SUBCOMPOSITOR => wl::Subcompositor::new(client, id),
            Self::XDG_WM_BASE => xdg::WmBase::new(client, id),
            _ => return Err(WlError::NO_GLOBAL),
        }
    }
}
use super::prelude::*;

pub struct Compositor;
impl WlCompositor<State> for Compositor {
    fn create_surface(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        id: Id,
    ) -> Result<(), WlError<'static>> {
        client.insert(Surface.into_object(id).into_any())
    }

    fn create_region(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        id: Id,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }
}

pub struct Shm;
impl Shm {
    pub fn create(client: &mut Client, id: NewId) -> Result<(), WlError<'static>> {
        let mut object = Self.into_versioned_object(id.id(), id.version())?;
        let this = &mut object.lease().unwrap();
        client.insert(object.into_any())?;

        // Advertise supported formats
        Self::format(this, client, wl_shm::Format::ARGB8888.into())?;
        Self::format(this, client, wl_shm::Format::XRGB8888.into())?;

        Ok(())
    }
}
impl WlShm<State> for Shm {
    fn create_pool(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        id: Id,
        fd: File,
        size: i32,
    ) -> Result<(), WlError<'static>> {
        println!("create pool: file {fd:?} with alleged size {size}");
        Ok(())
    }
}

pub struct Surface;
impl WlSurface<State> for Surface {
    fn destroy(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn attach(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        buffer: Option<Id>,
        x: i32,
        y: i32,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn damage(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn frame(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        callback: yutani::Id,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn set_opaque_region(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        region: Option<yutani::Id>,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn set_input_region(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        region: Option<yutani::Id>,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn commit(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn set_buffer_transform(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        transform: i32,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn set_buffer_scale(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        scale: i32,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn damage_buffer(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn offset(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        x: i32,
        y: i32,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }
}

pub struct Subcompositor;
impl WlSubcompositor<State> for Subcompositor {
    fn destroy(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        client.remove(this.id())?;
        Ok(())
    }

    fn get_subsurface(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        id: Id,
        surface: Id,
        parent: Id,
    ) -> Result<(), WlError<'static>> {
        let object = Subsurface.into_object(id);
        client.insert(object.into_any())
    }
}

pub struct Subsurface;
impl WlSubsurface<State> for Subsurface {
    fn destroy(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        client.remove(this.id())?;
        Ok(())
    }

    fn set_position(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        x: i32,
        y: i32,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn place_above(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        sibling: Id,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn place_below(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        sibling: Id,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn set_sync(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }

    fn set_desync(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        todo!()
    }
}
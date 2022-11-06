use std::{ops::{Deref, DerefMut}, num::NonZeroU32};

use super::prelude::*;

pub struct Compositor;
impl Compositor {
    pub fn new(client: &mut Client, id: NewId) -> Result<(), WlError<'static>> {
        let object = Self.into_versioned_object(id.id(), id.version())?;
        client.insert(object.into_any())?;

        Ok(())
    }
}
impl WlCompositor<State> for Compositor {
    fn create_surface(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        id: Id,
    ) -> Result<(), WlError<'static>> {
        client.insert(Surface::new().into_object(id).into_any())
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

pub struct ShmPool {
    file: File,
    buffer: *mut u8,
    size: usize
}
impl ShmPool {
    pub fn new(id: Id, file: File, size: usize) -> Result<Self, WlError<'static>> {
        use syslib::mmap::{Flags, Protection};
        let buffer = match syslib::mmap(0, size, Protection::READ | Protection::WRITE, Flags::SHARED, &file, 0) {
            Ok(buffer) => buffer as *mut u8,
            Err(error) => return Err(WlError { object: id, error: wl_shm::Error::INVALID_FD.into(), description: format!("mmap failed: {error:?}").into() })
        };
        
        Ok(Self { file, buffer, size })
    }
}
impl Drop for ShmPool {
    fn drop(&mut self) {
        unsafe {
            let _ = syslib::munmap(self.buffer as *mut _, self.size);
        }
    }
}
impl WlShmPool<State> for ShmPool {
    fn create_buffer(
        this: Lease<Self>,
        _: &mut EventLoop,
        client: &mut Client,
        id: Id,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: u32,
    ) -> Result<(), WlError<'static>> {
        let offset = offset.try_into().map_err(|_| WlError::DOMAIN)?;
        let stride = stride.try_into().map_err(|_| WlError::DOMAIN)?;
        let Ok(Some(width)) = width.try_into().map(|width| NonZeroU32::new(width)) else {
            return Err(WlError::DOMAIN)
        };
        let Ok(Some(height)) = height.try_into().map(|height| NonZeroU32::new(height)) else {
            return Err(WlError::DOMAIN)
        };
        client.insert(Buffer::new(this.id(), offset, width, height, stride, format.into()).into_object(id).into_any())
    }

    fn destroy(
        this: Lease<Self>,
        _: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        // The protocol seems to allow buffers to outlive their pool annoyingly
        client.remove(this.id())?;
        Ok(())
    }

    fn resize(
        mut this: Lease<Self>,
        _: &mut EventLoop,
        _: &mut Client,
        size: i32,
    ) -> Result<(), WlError<'static>> {
        use syslib::mmap::RemapFlags;
        let size = size.try_into().map_err(|_| WlError::DOMAIN)?;
        if size <= this.size {
            return Err(WlError::DOMAIN)
        }
        let buffer = match unsafe { syslib::mremap(this.buffer as *mut _, this.size, size, RemapFlags::MAY_MOVE) } {
            Ok(buffer) => buffer as *mut u8,
            Err(error) => return Err(WlError { object: this.id(), error: wl_shm::Error::INVALID_FD.into(), description: format!("mremap failed: {error:?}").into() })
        };
        this.buffer = buffer;
        this.size = size;
        Ok(())
    }
}

pub struct Shm;
impl Shm {
    pub fn new(client: &mut Client, id: NewId) -> Result<(), WlError<'static>> {
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
        _: Lease<Self>,
        _: &mut EventLoop,
        client: &mut Client,
        id: Id,
        fd: File,
        size: i32,
    ) -> Result<(), WlError<'static>> {
        let size = size.try_into().map_err(|_| WlError::DOMAIN)?;
        client.insert(
            ShmPool::new(id, fd, size)?
                .into_object(id)
                .into_any(),
        )
    }
}

pub struct Buffer {
    pool: Id,
    offset: usize,
    width: NonZeroU32,
    height: NonZeroU32,
    stride: usize,
    format: wl_shm::Format
}
impl Buffer {
    pub fn new(pool: Id, offset: usize, width: NonZeroU32, height: NonZeroU32, stride: usize, format: wl_shm::Format) -> Self {
        Self {
            pool,
            offset,
            width,
            height,
            stride,
            format
        }
    }
}
impl WlBuffer<State> for Buffer {
    fn destroy(
        this: Lease<Self>,
        _: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        client.remove(this.id())?;
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct SurfaceState {
    xdg_surface: Option<Id>,
    buffer: Option<Id>
}
pub struct Surface {
    state: DoubleBuffer<SurfaceState>,
    frame: Option<Id>
}
impl Surface {
    pub fn new() -> Self {
        Self {
            state: Default::default(),
            frame: None
        }
    }
}
impl WlSurface<State> for Surface {
    fn destroy(
        this: Lease<Self>,
        _: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        client.remove(this.id())?;
        Ok(())
    }

    fn attach(
        mut this: Lease<Self>,
        _: &mut EventLoop,
        _: &mut Client,
        buffer: Option<Id>,
        x: i32,
        y: i32,
    ) -> Result<(), WlError<'static>> {
        if this.version() >= 5 && (x != 0 || y != 0) {
            return Err(WlError {
                object: this.id(),
                error: wl_surface::Error::INVALID_OFFSET.into(),
                description: "wl_surface version 5 and higher do not support non-0 x/y values.".into()
            })
        }
        // TODO: support setting top left location for old clients
        this.state.pending().buffer = buffer;
        Ok(())
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
        // TODO...
        Ok(())
    }

    fn frame(
        mut this: Lease<Self>,
        _: &mut EventLoop,
        client: &mut Client,
        callback: Id,
    ) -> Result<(), WlError<'static>> {
        if let Some(frame) = this.frame {
            Display::delete_id(client, frame)?;
        }
        this.frame = Some(callback);
        Ok(())
    }

    fn set_opaque_region(
        this: Lease<Self>,
        event_loop: &mut EventLoop,
        client: &mut Client,
        region: Option<Id>,
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
        mut this: Lease<Self>,
        _: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        this.state.commit();
        if let Some(xdg_surface) = this.state.xdg_surface {
            let mut xdg_surface: Lease<xdg::Surface> = client.lease(xdg_surface)?.downcast().ok_or(WlError::INTERNAL)?;
            xdg_surface.commit(client)?
        }
        
        // Send the frame callback
        if let Some(frame) = this.frame {
            // TODO: get a time in milliseconds to return
            Callback::done(client, frame, 0)?;
        }
        this.frame = None;

        Ok(())
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
        // TODO...
        Ok(())
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
impl Subcompositor {
    pub fn new(client: &mut Client, id: NewId) -> Result<(), WlError<'static>> {
        let object = Self.into_versioned_object(id.id(), id.version())?;
        client.insert(object.into_any())?;

        Ok(())
    }
}
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
        let object = Subsurface::new(surface, parent).into_object(id);
        client.insert(object.into_any())
    }
}

#[derive(Clone, Default)]
pub struct SubsurfaceState {
    position: (i32, i32)
}
pub struct Subsurface {
    surface: Id,
    parent: Id,
    state: DoubleBuffer<SubsurfaceState>
}
impl Subsurface {
    pub fn new(surface: Id, parent: Id) -> Self {
        Self {
            surface,
            parent,
            state: Default::default()
        }
    }
}
impl WlSubsurface<State> for Subsurface {
    fn destroy(
        this: Lease<Self>,
        _: &mut EventLoop,
        client: &mut Client,
    ) -> Result<(), WlError<'static>> {
        client.remove(this.id())?;
        Ok(())
    }

    fn set_position(
        mut this: Lease<Self>,
        _: &mut EventLoop,
        _: &mut Client,
        x: i32,
        y: i32,
    ) -> Result<(), WlError<'static>> {
        this.state.pending().position = (x, y);
        Ok(())
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

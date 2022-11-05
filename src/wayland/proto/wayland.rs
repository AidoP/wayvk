// Auto-Generated file. Do not edit.
#![allow(dead_code)]
#![doc = "# Wayland"]
#![doc = ""]
#![doc = "## Copyright"]
#![doc = "Copyright © 2008-2011 Kristian Høgsberg\nCopyright © 2010-2011 Intel Corporation\nCopyright © 2012-2013 Collabora, Ltd.\n\nPermission is hereby granted, free of charge, to any person\nobtaining a copy of this software and associated documentation files\n(the \"Software\"), to deal in the Software without restriction,\nincluding without limitation the rights to use, copy, modify, merge,\npublish, distribute, sublicense, and/or sell copies of the Software,\nand to permit persons to whom the Software is furnished to do so,\nsubject to the following conditions:\n\nThe above copyright notice and this permission notice (including the\nnext paragraph) shall be included in all copies or substantial\nportions of the Software.\n\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND,\nEXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF\nMERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND\nNONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS\nBE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN\nACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN\nCONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE\nSOFTWARE."]
#[doc = "`Version 1`"]
#[doc = ""]
#[doc = "Core Global Object"]
#[doc = ""]
#[doc = "The core global object. This is a special singleton object. It\nis used for internal Wayland protocol features."]
pub trait r#WlDisplay<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_display";
    const VERSION: ::core::primitive::u32 = 1u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                let r#callback = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                Self::r#sync(_this, _event_loop, _client, r#callback)
            }
            1u16 => {
                let _stream = _client.stream();
                let r#registry = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                Self::r#get_registry(_this, _event_loop, _client, r#registry)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Asynchronous Roundtrip"]
    #[doc = ""]
    #[doc = "The sync request asks the server to emit the 'done' event\non the returned wl_callback object. Since requests are\nhandled in-order and events are delivered in-order, this can\nbe used as a barrier to ensure all previous requests and the\nresulting events have been handled.\n\nThe object returned by this request will be destroyed by the\ncompositor after the callback is fired and as such the client must not\nattempt to use it after that point.\n\nThe callback_data passed in the callback is the event serial."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`callback`: callback object for the sync request"]
    fn r#sync(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#callback: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Get Global Registry Object"]
    #[doc = ""]
    #[doc = "This request creates a registry object that allows the client\nto list and bind the global objects available from the\ncompositor.\n\nIt should be noted that the server side resources consumed in\nresponse to a get_registry request can only be released when the\nclient disconnects, not when the client side proxy is destroyed.\nTherefore, clients should invoke get_registry as infrequently as\npossible to avoid wasting memory."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`registry`: global registry object"]
    fn r#get_registry(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#registry: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Fatal Error Event"]
    #[doc = ""]
    #[doc = "The error event is sent out when a fatal (non-recoverable)\nerror has occurred. The object_id argument is the object\nwhere the error occurred, most often in response to a request\nto that object. The code identifies the error and is defined\nby the object interface. As such, each interface defines its\nown set of error codes. The message is a brief description\nof the error, for (debugging) convenience."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`object_id`: object where the error occurred"]
    #[doc = "\n`code`: error code"]
    #[doc = "\n`message`: error description"]
    fn r#error(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#object_id: ::wl::Id,
        r#code: ::core::primitive::u32,
        r#message: &'_ ::core::primitive::str,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 0u16);
        _stream.send_object(Some(r#object_id))?;
        _stream.send_u32(r#code)?;
        _stream.send_string(::core::option::Option::Some(r#message))?;
        _stream.commit(_key)
    }
    #[doc = "Acknowledge Object Id Deletion"]
    #[doc = ""]
    #[doc = "This event is used internally by the object ID management\nlogic. When a client deletes an object that it had created,\nthe server will send this event to acknowledge that it has\nseen the delete request. When the client receives this event,\nit will know that it can safely reuse the object ID."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`id`: deleted object ID"]
    fn r#delete_id(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#id: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 1u16);
        _stream.send_u32(r#id)?;
        _stream.commit(_key)
    }
}
pub mod r#wl_display {
    #[doc = "Global Error Values"]
    #[doc = ""]
    #[doc = "These errors are global and can be emitted in response to any\nserver request."]
    #[repr(transparent)]
    pub struct r#Error(u32);
    impl r#Error {
        #[doc = "Server Couldn T Find Object"]
        #[doc = ""]
        pub const r#INVALID_OBJECT: Self = Self(0u32);
        #[doc = "Method Doesn T Exist On The Specified Interface Or Malformed Request"]
        #[doc = ""]
        pub const r#INVALID_METHOD: Self = Self(1u32);
        #[doc = "Server Is Out Of Memory"]
        #[doc = ""]
        pub const r#NO_MEMORY: Self = Self(2u32);
        #[doc = "Implementation Error In Compositor"]
        #[doc = ""]
        pub const r#IMPLEMENTATION: Self = Self(3u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Error {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Error {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "INVALID_OBJECT", 0u32),
                1u32 => ::core::write!(f, "{}({})", "INVALID_METHOD", 1u32),
                2u32 => ::core::write!(f, "{}({})", "NO_MEMORY", 2u32),
                3u32 => ::core::write!(f, "{}({})", "IMPLEMENTATION", 3u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "`Version 1`"]
#[doc = ""]
#[doc = "Global Registry Object"]
#[doc = ""]
#[doc = "The singleton global registry object. The server has a number of\nglobal objects that are available to all clients. These objects\ntypically represent an actual object in the server (for example,\nan input device) or they are singleton objects that provide\nextension functionality.\n\nWhen a client creates a registry object, the registry object\nwill emit a global event for each global currently in the\nregistry. Globals come and go as a result of device or\nmonitor hotplugs, reconfiguration or other events, and the\nregistry will send out global and global_remove events to\nkeep the client up to date with the changes. To mark the end\nof the initial burst of events, the client can use the\nwl_display.sync request immediately after calling\nwl_display.get_registry.\n\nA client can bind to a global object by using the bind\nrequest. This creates a client-side handle that lets the object\nemit events to the client and lets the client invoke requests on\nthe object."]
pub trait r#WlRegistry<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_registry";
    const VERSION: ::core::primitive::u32 = 1u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                let r#name = _stream.u32()?;
                let r#id = _stream.new_id()?;
                Self::r#bind(_this, _event_loop, _client, r#name, r#id)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Bind An Object To The Display"]
    #[doc = ""]
    #[doc = "Binds a new, client-created object to the server using the\nspecified name as the identifier."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`name`: unique numeric name of the object"]
    #[doc = "\n`id`: bounded object"]
    fn r#bind(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#name: ::core::primitive::u32,
        r#id: ::wl::NewId,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Announce Global Object"]
    #[doc = ""]
    #[doc = "Notify the client of global objects.\n\nThe event notifies the client that a global object with\nthe given name is now available, and it implements the\ngiven version of the given interface."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`name`: numeric name of the global object"]
    #[doc = "\n`interface`: interface implemented by the object"]
    #[doc = "\n`version`: interface version"]
    fn r#global(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#name: ::core::primitive::u32,
        r#interface: &'_ ::core::primitive::str,
        r#version: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 0u16);
        _stream.send_u32(r#name)?;
        _stream.send_string(::core::option::Option::Some(r#interface))?;
        _stream.send_u32(r#version)?;
        _stream.commit(_key)
    }
    #[doc = "Announce Removal Of Global Object"]
    #[doc = ""]
    #[doc = "Notify the client of removed global objects.\n\nThis event notifies the client that the global identified\nby name is no longer available. If the client bound to\nthe global using the bind request, the client should now\ndestroy that object.\n\nThe object remains valid and requests to the object will be\nignored until the client destroys it, to avoid races between\nthe global going away and a client sending a request to it."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`name`: numeric name of the global object"]
    fn r#global_remove(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#name: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 1u16);
        _stream.send_u32(r#name)?;
        _stream.commit(_key)
    }
}
pub mod r#wl_registry {}
#[doc = "`Version 1`"]
#[doc = ""]
#[doc = "Callback Object"]
#[doc = ""]
#[doc = "Clients can handle the 'done' event to get notified when\nthe related request is done."]
pub trait r#WlCallback<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_callback";
    const VERSION: ::core::primitive::u32 = 1u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Done Event"]
    #[doc = ""]
    #[doc = "Notify the client when the related request is done."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`callback_data`: request-specific data for the callback"]
    fn r#done(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#callback_data: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 0u16);
        _stream.send_u32(r#callback_data)?;
        _stream.commit(_key)
    }
}
pub mod r#wl_callback {}
#[doc = "`Version 5`"]
#[doc = ""]
#[doc = "The Compositor Singleton"]
#[doc = ""]
#[doc = "A compositor. This object is a singleton global. The\ncompositor is in charge of combining the contents of multiple\nsurfaces into one displayable output."]
pub trait r#WlCompositor<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_compositor";
    const VERSION: ::core::primitive::u32 = 5u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                let r#id = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                Self::r#create_surface(_this, _event_loop, _client, r#id)
            }
            1u16 => {
                let _stream = _client.stream();
                let r#id = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                Self::r#create_region(_this, _event_loop, _client, r#id)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Create New Surface"]
    #[doc = ""]
    #[doc = "Ask the compositor to create a new surface."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`id`: the new surface"]
    fn r#create_surface(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#id: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Create New Region"]
    #[doc = ""]
    #[doc = "Ask the compositor to create a new region."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`id`: the new region"]
    fn r#create_region(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#id: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
}
pub mod r#wl_compositor {}
#[doc = "`Version 1`"]
#[doc = ""]
#[doc = "A Shared Memory Pool"]
#[doc = ""]
#[doc = "The wl_shm_pool object encapsulates a piece of memory shared\nbetween the compositor and client. Through the wl_shm_pool\nobject, the client can allocate shared memory wl_buffer objects.\nAll objects created through the same pool share the same\nunderlying mapped memory. Reusing the mapped memory avoids the\nsetup/teardown overhead and is useful when interactively resizing\na surface or for many small buffers."]
pub trait r#WlShmPool<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_shm_pool";
    const VERSION: ::core::primitive::u32 = 1u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                let r#id = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                let r#offset = _stream.i32()?;
                let r#width = _stream.i32()?;
                let r#height = _stream.i32()?;
                let r#stride = _stream.i32()?;
                let r#format = _stream.u32()?;
                Self::r#create_buffer(
                    _this,
                    _event_loop,
                    _client,
                    r#id,
                    r#offset,
                    r#width,
                    r#height,
                    r#stride,
                    r#format,
                )
            }
            1u16 => {
                let _stream = _client.stream();
                Self::r#destroy(_this, _event_loop, _client)
            }
            2u16 => {
                let _stream = _client.stream();
                let r#size = _stream.i32()?;
                Self::r#resize(_this, _event_loop, _client, r#size)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Create A Buffer From The Pool"]
    #[doc = ""]
    #[doc = "Create a wl_buffer object from the pool.\n\nThe buffer is created offset bytes into the pool and has\nwidth and height as specified. The stride argument specifies\nthe number of bytes from the beginning of one row to the beginning\nof the next. The format is the pixel format of the buffer and\nmust be one of those advertised through the wl_shm.format event.\n\nA buffer will keep a reference to the pool it was created from\nso it is valid to destroy the pool immediately after creating\na buffer from it."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`id`: buffer to create"]
    #[doc = "\n`offset`: buffer byte offset within the pool"]
    #[doc = "\n`width`: buffer width, in pixels"]
    #[doc = "\n`height`: buffer height, in pixels"]
    #[doc = "\n`stride`: number of bytes from the beginning of one row to the beginning of the next row"]
    #[doc = "\n`format`: buffer pixel format"]
    fn r#create_buffer(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#id: ::wl::Id,
        r#offset: ::core::primitive::i32,
        r#width: ::core::primitive::i32,
        r#height: ::core::primitive::i32,
        r#stride: ::core::primitive::i32,
        r#format: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Destroy The Pool"]
    #[doc = ""]
    #[doc = "Destroy the shared memory pool.\n\nThe mmapped memory will be released when all\nbuffers that have been created from this pool\nare gone."]
    fn r#destroy(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Change The Size Of The Pool Mapping"]
    #[doc = ""]
    #[doc = "This request will cause the server to remap the backing memory\nfor the pool from the file descriptor passed when the pool was\ncreated, but using the new size. This request can only be\nused to make the pool bigger.\n\nThis request only changes the amount of bytes that are mmapped\nby the server and does not touch the file corresponding to the\nfile descriptor passed at creation time. It is the client's\nresponsibility to ensure that the file is at least as big as\nthe new pool size."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`size`: new size of the pool, in bytes"]
    fn r#resize(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#size: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
}
pub mod r#wl_shm_pool {}
#[doc = "`Version 1`"]
#[doc = ""]
#[doc = "Shared Memory Support"]
#[doc = ""]
#[doc = "A singleton global object that provides support for shared\nmemory.\n\nClients can create wl_shm_pool objects using the create_pool\nrequest.\n\nOn binding the wl_shm object one or more format events\nare emitted to inform clients about the valid pixel formats\nthat can be used for buffers."]
pub trait r#WlShm<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_shm";
    const VERSION: ::core::primitive::u32 = 1u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                let r#id = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                let r#fd = _stream.file()?;
                let r#size = _stream.i32()?;
                Self::r#create_pool(_this, _event_loop, _client, r#id, r#fd, r#size)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Create A Shm Pool"]
    #[doc = ""]
    #[doc = "Create a new wl_shm_pool object.\n\nThe pool can be used to create shared memory based buffer\nobjects. The server will mmap size bytes of the passed file\ndescriptor, to use as backing memory for the pool."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`id`: pool to create"]
    #[doc = "\n`fd`: file descriptor for the pool"]
    #[doc = "\n`size`: pool size, in bytes"]
    fn r#create_pool(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#id: ::wl::Id,
        r#fd: ::wl::File,
        r#size: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Pixel Format Description"]
    #[doc = ""]
    #[doc = "Informs the client about a valid pixel format that\ncan be used for buffers. Known formats include\nargb8888 and xrgb8888."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`format`: buffer pixel format"]
    fn r#format(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#format: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 0u16);
        _stream.send_u32(r#format)?;
        _stream.commit(_key)
    }
}
pub mod r#wl_shm {
    #[doc = "Wl Shm Error Values"]
    #[doc = ""]
    #[doc = "These errors can be emitted in response to wl_shm requests."]
    #[repr(transparent)]
    pub struct r#Error(u32);
    impl r#Error {
        #[doc = "Buffer Format Is Not Known"]
        #[doc = ""]
        pub const r#INVALID_FORMAT: Self = Self(0u32);
        #[doc = "Invalid Size Or Stride During Pool Or Buffer Creation"]
        #[doc = ""]
        pub const r#INVALID_STRIDE: Self = Self(1u32);
        #[doc = "Mmapping The File Descriptor Failed"]
        #[doc = ""]
        pub const r#INVALID_FD: Self = Self(2u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Error {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Error {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "INVALID_FORMAT", 0u32),
                1u32 => ::core::write!(f, "{}({})", "INVALID_STRIDE", 1u32),
                2u32 => ::core::write!(f, "{}({})", "INVALID_FD", 2u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
    #[doc = "Pixel Formats"]
    #[doc = ""]
    #[doc = "This describes the memory layout of an individual pixel.\n\nAll renderers should support argb8888 and xrgb8888 but any other\nformats are optional and may not be supported by the particular\nrenderer in use.\n\nThe drm format codes match the macros defined in drm_fourcc.h, except\nargb8888 and xrgb8888. The formats actually supported by the compositor\nwill be reported by the format event.\n\nFor all wl_shm formats and unless specified in another protocol\nextension, pre-multiplied alpha is used for pixel values."]
    #[repr(transparent)]
    pub struct r#Format(u32);
    impl r#Format {
        #[doc = "32 Bit Argb Format 31 0 A R G B 8 8 8 8 Little Endian"]
        #[doc = ""]
        pub const r#ARGB8888: Self = Self(0u32);
        #[doc = "32 Bit Rgb Format 31 0 X R G B 8 8 8 8 Little Endian"]
        #[doc = ""]
        pub const r#XRGB8888: Self = Self(1u32);
        #[doc = "8 Bit Color Index Format 7 0 C"]
        #[doc = ""]
        pub const r#C8: Self = Self(538982467u32);
        #[doc = "8 Bit Rgb Format 7 0 R G B 3 3 2"]
        #[doc = ""]
        pub const r#RGB332: Self = Self(943867730u32);
        #[doc = "8 Bit Bgr Format 7 0 B G R 2 3 3"]
        #[doc = ""]
        pub const r#BGR233: Self = Self(944916290u32);
        #[doc = "16 Bit X Rgb Format 15 0 X R G B 4 4 4 4 Little Endian"]
        #[doc = ""]
        pub const r#XRGB4444: Self = Self(842093144u32);
        #[doc = "16 Bit X Bgr Format 15 0 X B G R 4 4 4 4 Little Endian"]
        #[doc = ""]
        pub const r#XBGR4444: Self = Self(842089048u32);
        #[doc = "16 Bit Rg Bx Format 15 0 R G B X 4 4 4 4 Little Endian"]
        #[doc = ""]
        pub const r#RGBX4444: Self = Self(842094674u32);
        #[doc = "16 Bit Bg Rx Format 15 0 B G R X 4 4 4 4 Little Endian"]
        #[doc = ""]
        pub const r#BGRX4444: Self = Self(842094658u32);
        #[doc = "16 Bit Argb Format 15 0 A R G B 4 4 4 4 Little Endian"]
        #[doc = ""]
        pub const r#ARGB4444: Self = Self(842093121u32);
        #[doc = "16 Bit Abgr Format 15 0 A B G R 4 4 4 4 Little Endian"]
        #[doc = ""]
        pub const r#ABGR4444: Self = Self(842089025u32);
        #[doc = "16 Bit Rbga Format 15 0 R G B A 4 4 4 4 Little Endian"]
        #[doc = ""]
        pub const r#RGBA4444: Self = Self(842088786u32);
        #[doc = "16 Bit Bgra Format 15 0 B G R A 4 4 4 4 Little Endian"]
        #[doc = ""]
        pub const r#BGRA4444: Self = Self(842088770u32);
        #[doc = "16 Bit X Rgb Format 15 0 X R G B 1 5 5 5 Little Endian"]
        #[doc = ""]
        pub const r#XRGB1555: Self = Self(892424792u32);
        #[doc = "16 Bit X Bgr 1555 Format 15 0 X B G R 1 5 5 5 Little Endian"]
        #[doc = ""]
        pub const r#XBGR1555: Self = Self(892420696u32);
        #[doc = "16 Bit Rg Bx 5551 Format 15 0 R G B X 5 5 5 1 Little Endian"]
        #[doc = ""]
        pub const r#RGBX5551: Self = Self(892426322u32);
        #[doc = "16 Bit Bg Rx 5551 Format 15 0 B G R X 5 5 5 1 Little Endian"]
        #[doc = ""]
        pub const r#BGRX5551: Self = Self(892426306u32);
        #[doc = "16 Bit Argb 1555 Format 15 0 A R G B 1 5 5 5 Little Endian"]
        #[doc = ""]
        pub const r#ARGB1555: Self = Self(892424769u32);
        #[doc = "16 Bit Abgr 1555 Format 15 0 A B G R 1 5 5 5 Little Endian"]
        #[doc = ""]
        pub const r#ABGR1555: Self = Self(892420673u32);
        #[doc = "16 Bit Rgba 5551 Format 15 0 R G B A 5 5 5 1 Little Endian"]
        #[doc = ""]
        pub const r#RGBA5551: Self = Self(892420434u32);
        #[doc = "16 Bit Bgra 5551 Format 15 0 B G R A 5 5 5 1 Little Endian"]
        #[doc = ""]
        pub const r#BGRA5551: Self = Self(892420418u32);
        #[doc = "16 Bit Rgb 565 Format 15 0 R G B 5 6 5 Little Endian"]
        #[doc = ""]
        pub const r#RGB565: Self = Self(909199186u32);
        #[doc = "16 Bit Bgr 565 Format 15 0 B G R 5 6 5 Little Endian"]
        #[doc = ""]
        pub const r#BGR565: Self = Self(909199170u32);
        #[doc = "24 Bit Rgb Format 23 0 R G B Little Endian"]
        #[doc = ""]
        pub const r#RGB888: Self = Self(875710290u32);
        #[doc = "24 Bit Bgr Format 23 0 B G R Little Endian"]
        #[doc = ""]
        pub const r#BGR888: Self = Self(875710274u32);
        #[doc = "32 Bit X Bgr Format 31 0 X B G R 8 8 8 8 Little Endian"]
        #[doc = ""]
        pub const r#XBGR8888: Self = Self(875709016u32);
        #[doc = "32 Bit Rg Bx Format 31 0 R G B X 8 8 8 8 Little Endian"]
        #[doc = ""]
        pub const r#RGBX8888: Self = Self(875714642u32);
        #[doc = "32 Bit Bg Rx Format 31 0 B G R X 8 8 8 8 Little Endian"]
        #[doc = ""]
        pub const r#BGRX8888: Self = Self(875714626u32);
        #[doc = "32 Bit Abgr Format 31 0 A B G R 8 8 8 8 Little Endian"]
        #[doc = ""]
        pub const r#ABGR8888: Self = Self(875708993u32);
        #[doc = "32 Bit Rgba Format 31 0 R G B A 8 8 8 8 Little Endian"]
        #[doc = ""]
        pub const r#RGBA8888: Self = Self(875708754u32);
        #[doc = "32 Bit Bgra Format 31 0 B G R A 8 8 8 8 Little Endian"]
        #[doc = ""]
        pub const r#BGRA8888: Self = Self(875708738u32);
        #[doc = "32 Bit X Rgb Format 31 0 X R G B 2 10 10 10 Little Endian"]
        #[doc = ""]
        pub const r#XRGB2101010: Self = Self(808669784u32);
        #[doc = "32 Bit X Bgr Format 31 0 X B G R 2 10 10 10 Little Endian"]
        #[doc = ""]
        pub const r#XBGR2101010: Self = Self(808665688u32);
        #[doc = "32 Bit Rg Bx Format 31 0 R G B X 10 10 10 2 Little Endian"]
        #[doc = ""]
        pub const r#RGBX1010102: Self = Self(808671314u32);
        #[doc = "32 Bit Bg Rx Format 31 0 B G R X 10 10 10 2 Little Endian"]
        #[doc = ""]
        pub const r#BGRX1010102: Self = Self(808671298u32);
        #[doc = "32 Bit Argb Format 31 0 A R G B 2 10 10 10 Little Endian"]
        #[doc = ""]
        pub const r#ARGB2101010: Self = Self(808669761u32);
        #[doc = "32 Bit Abgr Format 31 0 A B G R 2 10 10 10 Little Endian"]
        #[doc = ""]
        pub const r#ABGR2101010: Self = Self(808665665u32);
        #[doc = "32 Bit Rgba Format 31 0 R G B A 10 10 10 2 Little Endian"]
        #[doc = ""]
        pub const r#RGBA1010102: Self = Self(808665426u32);
        #[doc = "32 Bit Bgra Format 31 0 B G R A 10 10 10 2 Little Endian"]
        #[doc = ""]
        pub const r#BGRA1010102: Self = Self(808665410u32);
        #[doc = "Packed Y Cb Cr Format 31 0 Cr0 Y1 Cb0 Y0 8 8 8 8 Little Endian"]
        #[doc = ""]
        pub const r#YUYV: Self = Self(1448695129u32);
        #[doc = "Packed Y Cb Cr Format 31 0 Cb0 Y1 Cr0 Y0 8 8 8 8 Little Endian"]
        #[doc = ""]
        pub const r#YVYU: Self = Self(1431918169u32);
        #[doc = "Packed Y Cb Cr Format 31 0 Y1 Cr0 Y0 Cb0 8 8 8 8 Little Endian"]
        #[doc = ""]
        pub const r#UYVY: Self = Self(1498831189u32);
        #[doc = "Packed Y Cb Cr Format 31 0 Y1 Cb0 Y0 Cr0 8 8 8 8 Little Endian"]
        #[doc = ""]
        pub const r#VYUY: Self = Self(1498765654u32);
        #[doc = "Packed Ay Cb Cr Format 31 0 A Y Cb Cr 8 8 8 8 Little Endian"]
        #[doc = ""]
        pub const r#AYUV: Self = Self(1448433985u32);
        #[doc = "2 Plane Y Cb Cr Cr Cb Format 2x2 Subsampled Cr Cb Plane"]
        #[doc = ""]
        pub const r#NV12: Self = Self(842094158u32);
        #[doc = "2 Plane Y Cb Cr Cb Cr Format 2x2 Subsampled Cb Cr Plane"]
        #[doc = ""]
        pub const r#NV21: Self = Self(825382478u32);
        #[doc = "2 Plane Y Cb Cr Cr Cb Format 2x1 Subsampled Cr Cb Plane"]
        #[doc = ""]
        pub const r#NV16: Self = Self(909203022u32);
        #[doc = "2 Plane Y Cb Cr Cb Cr Format 2x1 Subsampled Cb Cr Plane"]
        #[doc = ""]
        pub const r#NV61: Self = Self(825644622u32);
        #[doc = "3 Plane Y Cb Cr Format 4x4 Subsampled Cb 1 And Cr 2 Planes"]
        #[doc = ""]
        pub const r#YUV410: Self = Self(961959257u32);
        #[doc = "3 Plane Y Cb Cr Format 4x4 Subsampled Cr 1 And Cb 2 Planes"]
        #[doc = ""]
        pub const r#YVU410: Self = Self(961893977u32);
        #[doc = "3 Plane Y Cb Cr Format 4x1 Subsampled Cb 1 And Cr 2 Planes"]
        #[doc = ""]
        pub const r#YUV411: Self = Self(825316697u32);
        #[doc = "3 Plane Y Cb Cr Format 4x1 Subsampled Cr 1 And Cb 2 Planes"]
        #[doc = ""]
        pub const r#YVU411: Self = Self(825316953u32);
        #[doc = "3 Plane Y Cb Cr Format 2x2 Subsampled Cb 1 And Cr 2 Planes"]
        #[doc = ""]
        pub const r#YUV420: Self = Self(842093913u32);
        #[doc = "3 Plane Y Cb Cr Format 2x2 Subsampled Cr 1 And Cb 2 Planes"]
        #[doc = ""]
        pub const r#YVU420: Self = Self(842094169u32);
        #[doc = "3 Plane Y Cb Cr Format 2x1 Subsampled Cb 1 And Cr 2 Planes"]
        #[doc = ""]
        pub const r#YUV422: Self = Self(909202777u32);
        #[doc = "3 Plane Y Cb Cr Format 2x1 Subsampled Cr 1 And Cb 2 Planes"]
        #[doc = ""]
        pub const r#YVU422: Self = Self(909203033u32);
        #[doc = "3 Plane Y Cb Cr Format Non Subsampled Cb 1 And Cr 2 Planes"]
        #[doc = ""]
        pub const r#YUV444: Self = Self(875713881u32);
        #[doc = "3 Plane Y Cb Cr Format Non Subsampled Cr 1 And Cb 2 Planes"]
        #[doc = ""]
        pub const r#YVU444: Self = Self(875714137u32);
        #[doc = "7 0 R"]
        #[doc = ""]
        pub const r#R8: Self = Self(538982482u32);
        #[doc = "15 0 R Little Endian"]
        #[doc = ""]
        pub const r#R16: Self = Self(540422482u32);
        #[doc = "15 0 R G 8 8 Little Endian"]
        #[doc = ""]
        pub const r#RG88: Self = Self(943212370u32);
        #[doc = "15 0 G R 8 8 Little Endian"]
        #[doc = ""]
        pub const r#GR88: Self = Self(943215175u32);
        #[doc = "31 0 R G 16 16 Little Endian"]
        #[doc = ""]
        pub const r#RG1616: Self = Self(842221394u32);
        #[doc = "31 0 G R 16 16 Little Endian"]
        #[doc = ""]
        pub const r#GR1616: Self = Self(842224199u32);
        #[doc = "63 0 X R G B 16 16 16 16 Little Endian"]
        #[doc = ""]
        pub const r#XRGB16161616F: Self = Self(1211388504u32);
        #[doc = "63 0 X B G R 16 16 16 16 Little Endian"]
        #[doc = ""]
        pub const r#XBGR16161616F: Self = Self(1211384408u32);
        #[doc = "63 0 A R G B 16 16 16 16 Little Endian"]
        #[doc = ""]
        pub const r#ARGB16161616F: Self = Self(1211388481u32);
        #[doc = "63 0 A B G R 16 16 16 16 Little Endian"]
        #[doc = ""]
        pub const r#ABGR16161616F: Self = Self(1211384385u32);
        #[doc = "31 0 X Y Cb Cr 8 8 8 8 Little Endian"]
        #[doc = ""]
        pub const r#XYUV8888: Self = Self(1448434008u32);
        #[doc = "23 0 Cr Cb Y 8 8 8 Little Endian"]
        #[doc = ""]
        pub const r#VUY888: Self = Self(875713878u32);
        #[doc = "Y Followed By U Then V 10 10 10 Non Linear Modifier Only"]
        #[doc = ""]
        pub const r#VUY101010: Self = Self(808670550u32);
        #[doc = "63 0 Cr0 0 Y1 0 Cb0 0 Y0 0 10 6 10 6 10 6 10 6 Little Endian Per 2 Y Pixels"]
        #[doc = ""]
        pub const r#Y210: Self = Self(808530521u32);
        #[doc = "63 0 Cr0 0 Y1 0 Cb0 0 Y0 0 12 4 12 4 12 4 12 4 Little Endian Per 2 Y Pixels"]
        #[doc = ""]
        pub const r#Y212: Self = Self(842084953u32);
        #[doc = "63 0 Cr0 Y1 Cb0 Y0 16 16 16 16 Little Endian Per 2 Y Pixels"]
        #[doc = ""]
        pub const r#Y216: Self = Self(909193817u32);
        #[doc = "31 0 A Cr Y Cb 2 10 10 10 Little Endian"]
        #[doc = ""]
        pub const r#Y410: Self = Self(808531033u32);
        #[doc = "63 0 A 0 Cr 0 Y 0 Cb 0 12 4 12 4 12 4 12 4 Little Endian"]
        #[doc = ""]
        pub const r#Y412: Self = Self(842085465u32);
        #[doc = "63 0 A Cr Y Cb 16 16 16 16 Little Endian"]
        #[doc = ""]
        pub const r#Y416: Self = Self(909194329u32);
        #[doc = "31 0 X Cr Y Cb 2 10 10 10 Little Endian"]
        #[doc = ""]
        pub const r#XVYU2101010: Self = Self(808670808u32);
        #[doc = "63 0 X 0 Cr 0 Y 0 Cb 0 12 4 12 4 12 4 12 4 Little Endian"]
        #[doc = ""]
        pub const r#XVYU12_16161616: Self = Self(909334104u32);
        #[doc = "63 0 X Cr Y Cb 16 16 16 16 Little Endian"]
        #[doc = ""]
        pub const r#XVYU16161616: Self = Self(942954072u32);
        #[doc = "63 0 A3 A2 Y3 0 Cr0 0 Y2 0 A1 A0 Y1 0 Cb0 0 Y0 0 1 1 8 2 8 2 8 2 1 1 8 2 8 2 8 2 Little Endian"]
        #[doc = ""]
        pub const r#Y0L0: Self = Self(810299481u32);
        #[doc = "63 0 X3 X2 Y3 0 Cr0 0 Y2 0 X1 X0 Y1 0 Cb0 0 Y0 0 1 1 8 2 8 2 8 2 1 1 8 2 8 2 8 2 Little Endian"]
        #[doc = ""]
        pub const r#X0L0: Self = Self(810299480u32);
        #[doc = "63 0 A3 A2 Y3 Cr0 Y2 A1 A0 Y1 Cb0 Y0 1 1 10 10 10 1 1 10 10 10 Little Endian"]
        #[doc = ""]
        pub const r#Y0L2: Self = Self(843853913u32);
        #[doc = "63 0 X3 X2 Y3 Cr0 Y2 X1 X0 Y1 Cb0 Y0 1 1 10 10 10 1 1 10 10 10 Little Endian"]
        #[doc = ""]
        pub const r#X0L2: Self = Self(843853912u32);
        #[doc = ""]
        pub const r#YUV420_8BIT: Self = Self(942691673u32);
        #[doc = ""]
        pub const r#YUV420_10BIT: Self = Self(808539481u32);
        #[doc = ""]
        pub const r#XRGB8888_A8: Self = Self(943805016u32);
        #[doc = ""]
        pub const r#XBGR8888_A8: Self = Self(943800920u32);
        #[doc = ""]
        pub const r#RGBX8888_A8: Self = Self(943806546u32);
        #[doc = ""]
        pub const r#BGRX8888_A8: Self = Self(943806530u32);
        #[doc = ""]
        pub const r#RGB888_A8: Self = Self(943798354u32);
        #[doc = ""]
        pub const r#BGR888_A8: Self = Self(943798338u32);
        #[doc = ""]
        pub const r#RGB565_A8: Self = Self(943797586u32);
        #[doc = ""]
        pub const r#BGR565_A8: Self = Self(943797570u32);
        #[doc = "Non Subsampled Cr Cb Plane"]
        #[doc = ""]
        pub const r#NV24: Self = Self(875714126u32);
        #[doc = "Non Subsampled Cb Cr Plane"]
        #[doc = ""]
        pub const r#NV42: Self = Self(842290766u32);
        #[doc = "2x1 Subsampled Cr Cb Plane 10 Bit Per Channel"]
        #[doc = ""]
        pub const r#P210: Self = Self(808530512u32);
        #[doc = "2x2 Subsampled Cr Cb Plane 10 Bits Per Channel"]
        #[doc = ""]
        pub const r#P010: Self = Self(808530000u32);
        #[doc = "2x2 Subsampled Cr Cb Plane 12 Bits Per Channel"]
        #[doc = ""]
        pub const r#P012: Self = Self(842084432u32);
        #[doc = "2x2 Subsampled Cr Cb Plane 16 Bits Per Channel"]
        #[doc = ""]
        pub const r#P016: Self = Self(909193296u32);
        #[doc = "63 0 A X B X G X R X 10 6 10 6 10 6 10 6 Little Endian"]
        #[doc = ""]
        pub const r#AXBXGXRX106106106106: Self = Self(808534593u32);
        #[doc = "2x2 Subsampled Cr Cb Plane"]
        #[doc = ""]
        pub const r#NV15: Self = Self(892425806u32);
        #[doc = ""]
        pub const r#Q410: Self = Self(808531025u32);
        #[doc = ""]
        pub const r#Q401: Self = Self(825242705u32);
        #[doc = "63 0 X R G B 16 16 16 16 Little Endian"]
        #[doc = ""]
        pub const r#XRGB16161616: Self = Self(942953048u32);
        #[doc = "63 0 X B G R 16 16 16 16 Little Endian"]
        #[doc = ""]
        pub const r#XBGR16161616: Self = Self(942948952u32);
        #[doc = "63 0 A R G B 16 16 16 16 Little Endian"]
        #[doc = ""]
        pub const r#ARGB16161616: Self = Self(942953025u32);
        #[doc = "63 0 A B G R 16 16 16 16 Little Endian"]
        #[doc = ""]
        pub const r#ABGR16161616: Self = Self(942948929u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Format {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Format {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Format {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "ARGB8888", 0u32),
                1u32 => ::core::write!(f, "{}({})", "XRGB8888", 1u32),
                538982467u32 => ::core::write!(f, "{}({})", "C8", 538982467u32),
                943867730u32 => ::core::write!(f, "{}({})", "RGB332", 943867730u32),
                944916290u32 => ::core::write!(f, "{}({})", "BGR233", 944916290u32),
                842093144u32 => ::core::write!(f, "{}({})", "XRGB4444", 842093144u32),
                842089048u32 => ::core::write!(f, "{}({})", "XBGR4444", 842089048u32),
                842094674u32 => ::core::write!(f, "{}({})", "RGBX4444", 842094674u32),
                842094658u32 => ::core::write!(f, "{}({})", "BGRX4444", 842094658u32),
                842093121u32 => ::core::write!(f, "{}({})", "ARGB4444", 842093121u32),
                842089025u32 => ::core::write!(f, "{}({})", "ABGR4444", 842089025u32),
                842088786u32 => ::core::write!(f, "{}({})", "RGBA4444", 842088786u32),
                842088770u32 => ::core::write!(f, "{}({})", "BGRA4444", 842088770u32),
                892424792u32 => ::core::write!(f, "{}({})", "XRGB1555", 892424792u32),
                892420696u32 => ::core::write!(f, "{}({})", "XBGR1555", 892420696u32),
                892426322u32 => ::core::write!(f, "{}({})", "RGBX5551", 892426322u32),
                892426306u32 => ::core::write!(f, "{}({})", "BGRX5551", 892426306u32),
                892424769u32 => ::core::write!(f, "{}({})", "ARGB1555", 892424769u32),
                892420673u32 => ::core::write!(f, "{}({})", "ABGR1555", 892420673u32),
                892420434u32 => ::core::write!(f, "{}({})", "RGBA5551", 892420434u32),
                892420418u32 => ::core::write!(f, "{}({})", "BGRA5551", 892420418u32),
                909199186u32 => ::core::write!(f, "{}({})", "RGB565", 909199186u32),
                909199170u32 => ::core::write!(f, "{}({})", "BGR565", 909199170u32),
                875710290u32 => ::core::write!(f, "{}({})", "RGB888", 875710290u32),
                875710274u32 => ::core::write!(f, "{}({})", "BGR888", 875710274u32),
                875709016u32 => ::core::write!(f, "{}({})", "XBGR8888", 875709016u32),
                875714642u32 => ::core::write!(f, "{}({})", "RGBX8888", 875714642u32),
                875714626u32 => ::core::write!(f, "{}({})", "BGRX8888", 875714626u32),
                875708993u32 => ::core::write!(f, "{}({})", "ABGR8888", 875708993u32),
                875708754u32 => ::core::write!(f, "{}({})", "RGBA8888", 875708754u32),
                875708738u32 => ::core::write!(f, "{}({})", "BGRA8888", 875708738u32),
                808669784u32 => ::core::write!(f, "{}({})", "XRGB2101010", 808669784u32),
                808665688u32 => ::core::write!(f, "{}({})", "XBGR2101010", 808665688u32),
                808671314u32 => ::core::write!(f, "{}({})", "RGBX1010102", 808671314u32),
                808671298u32 => ::core::write!(f, "{}({})", "BGRX1010102", 808671298u32),
                808669761u32 => ::core::write!(f, "{}({})", "ARGB2101010", 808669761u32),
                808665665u32 => ::core::write!(f, "{}({})", "ABGR2101010", 808665665u32),
                808665426u32 => ::core::write!(f, "{}({})", "RGBA1010102", 808665426u32),
                808665410u32 => ::core::write!(f, "{}({})", "BGRA1010102", 808665410u32),
                1448695129u32 => ::core::write!(f, "{}({})", "YUYV", 1448695129u32),
                1431918169u32 => ::core::write!(f, "{}({})", "YVYU", 1431918169u32),
                1498831189u32 => ::core::write!(f, "{}({})", "UYVY", 1498831189u32),
                1498765654u32 => ::core::write!(f, "{}({})", "VYUY", 1498765654u32),
                1448433985u32 => ::core::write!(f, "{}({})", "AYUV", 1448433985u32),
                842094158u32 => ::core::write!(f, "{}({})", "NV12", 842094158u32),
                825382478u32 => ::core::write!(f, "{}({})", "NV21", 825382478u32),
                909203022u32 => ::core::write!(f, "{}({})", "NV16", 909203022u32),
                825644622u32 => ::core::write!(f, "{}({})", "NV61", 825644622u32),
                961959257u32 => ::core::write!(f, "{}({})", "YUV410", 961959257u32),
                961893977u32 => ::core::write!(f, "{}({})", "YVU410", 961893977u32),
                825316697u32 => ::core::write!(f, "{}({})", "YUV411", 825316697u32),
                825316953u32 => ::core::write!(f, "{}({})", "YVU411", 825316953u32),
                842093913u32 => ::core::write!(f, "{}({})", "YUV420", 842093913u32),
                842094169u32 => ::core::write!(f, "{}({})", "YVU420", 842094169u32),
                909202777u32 => ::core::write!(f, "{}({})", "YUV422", 909202777u32),
                909203033u32 => ::core::write!(f, "{}({})", "YVU422", 909203033u32),
                875713881u32 => ::core::write!(f, "{}({})", "YUV444", 875713881u32),
                875714137u32 => ::core::write!(f, "{}({})", "YVU444", 875714137u32),
                538982482u32 => ::core::write!(f, "{}({})", "R8", 538982482u32),
                540422482u32 => ::core::write!(f, "{}({})", "R16", 540422482u32),
                943212370u32 => ::core::write!(f, "{}({})", "RG88", 943212370u32),
                943215175u32 => ::core::write!(f, "{}({})", "GR88", 943215175u32),
                842221394u32 => ::core::write!(f, "{}({})", "RG1616", 842221394u32),
                842224199u32 => ::core::write!(f, "{}({})", "GR1616", 842224199u32),
                1211388504u32 => ::core::write!(f, "{}({})", "XRGB16161616F", 1211388504u32),
                1211384408u32 => ::core::write!(f, "{}({})", "XBGR16161616F", 1211384408u32),
                1211388481u32 => ::core::write!(f, "{}({})", "ARGB16161616F", 1211388481u32),
                1211384385u32 => ::core::write!(f, "{}({})", "ABGR16161616F", 1211384385u32),
                1448434008u32 => ::core::write!(f, "{}({})", "XYUV8888", 1448434008u32),
                875713878u32 => ::core::write!(f, "{}({})", "VUY888", 875713878u32),
                808670550u32 => ::core::write!(f, "{}({})", "VUY101010", 808670550u32),
                808530521u32 => ::core::write!(f, "{}({})", "Y210", 808530521u32),
                842084953u32 => ::core::write!(f, "{}({})", "Y212", 842084953u32),
                909193817u32 => ::core::write!(f, "{}({})", "Y216", 909193817u32),
                808531033u32 => ::core::write!(f, "{}({})", "Y410", 808531033u32),
                842085465u32 => ::core::write!(f, "{}({})", "Y412", 842085465u32),
                909194329u32 => ::core::write!(f, "{}({})", "Y416", 909194329u32),
                808670808u32 => ::core::write!(f, "{}({})", "XVYU2101010", 808670808u32),
                909334104u32 => ::core::write!(f, "{}({})", "XVYU12_16161616", 909334104u32),
                942954072u32 => ::core::write!(f, "{}({})", "XVYU16161616", 942954072u32),
                810299481u32 => ::core::write!(f, "{}({})", "Y0L0", 810299481u32),
                810299480u32 => ::core::write!(f, "{}({})", "X0L0", 810299480u32),
                843853913u32 => ::core::write!(f, "{}({})", "Y0L2", 843853913u32),
                843853912u32 => ::core::write!(f, "{}({})", "X0L2", 843853912u32),
                942691673u32 => ::core::write!(f, "{}({})", "YUV420_8BIT", 942691673u32),
                808539481u32 => ::core::write!(f, "{}({})", "YUV420_10BIT", 808539481u32),
                943805016u32 => ::core::write!(f, "{}({})", "XRGB8888_A8", 943805016u32),
                943800920u32 => ::core::write!(f, "{}({})", "XBGR8888_A8", 943800920u32),
                943806546u32 => ::core::write!(f, "{}({})", "RGBX8888_A8", 943806546u32),
                943806530u32 => ::core::write!(f, "{}({})", "BGRX8888_A8", 943806530u32),
                943798354u32 => ::core::write!(f, "{}({})", "RGB888_A8", 943798354u32),
                943798338u32 => ::core::write!(f, "{}({})", "BGR888_A8", 943798338u32),
                943797586u32 => ::core::write!(f, "{}({})", "RGB565_A8", 943797586u32),
                943797570u32 => ::core::write!(f, "{}({})", "BGR565_A8", 943797570u32),
                875714126u32 => ::core::write!(f, "{}({})", "NV24", 875714126u32),
                842290766u32 => ::core::write!(f, "{}({})", "NV42", 842290766u32),
                808530512u32 => ::core::write!(f, "{}({})", "P210", 808530512u32),
                808530000u32 => ::core::write!(f, "{}({})", "P010", 808530000u32),
                842084432u32 => ::core::write!(f, "{}({})", "P012", 842084432u32),
                909193296u32 => ::core::write!(f, "{}({})", "P016", 909193296u32),
                808534593u32 => ::core::write!(f, "{}({})", "AXBXGXRX106106106106", 808534593u32),
                892425806u32 => ::core::write!(f, "{}({})", "NV15", 892425806u32),
                808531025u32 => ::core::write!(f, "{}({})", "Q410", 808531025u32),
                825242705u32 => ::core::write!(f, "{}({})", "Q401", 825242705u32),
                942953048u32 => ::core::write!(f, "{}({})", "XRGB16161616", 942953048u32),
                942948952u32 => ::core::write!(f, "{}({})", "XBGR16161616", 942948952u32),
                942953025u32 => ::core::write!(f, "{}({})", "ARGB16161616", 942953025u32),
                942948929u32 => ::core::write!(f, "{}({})", "ABGR16161616", 942948929u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "`Version 1`"]
#[doc = ""]
#[doc = "Content For A Wl Surface"]
#[doc = ""]
#[doc = "A buffer provides the content for a wl_surface. Buffers are\ncreated through factory interfaces such as wl_shm, wp_linux_buffer_params\n(from the linux-dmabuf protocol extension) or similar. It has a width and\na height and can be attached to a wl_surface, but the mechanism by which a\nclient provides and updates the contents is defined by the buffer factory\ninterface.\n\nIf the buffer uses a format that has an alpha channel, the alpha channel\nis assumed to be premultiplied in the color channels unless otherwise\nspecified."]
pub trait r#WlBuffer<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_buffer";
    const VERSION: ::core::primitive::u32 = 1u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                Self::r#destroy(_this, _event_loop, _client)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Destroy A Buffer"]
    #[doc = ""]
    #[doc = "Destroy a buffer. If and how you need to release the backing\nstorage is defined by the buffer factory interface.\n\nFor possible side-effects to a surface, see wl_surface.attach."]
    fn r#destroy(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Compositor Releases Buffer"]
    #[doc = ""]
    #[doc = "Sent when this wl_buffer is no longer used by the compositor.\nThe client is now free to reuse or destroy this buffer and its\nbacking storage.\n\nIf a client receives a release event before the frame callback\nrequested in the same wl_surface.commit that attaches this\nwl_buffer to a surface, then the client is immediately free to\nreuse the buffer and its backing storage, and does not need a\nsecond buffer for the next surface content update. Typically\nthis is possible, when the compositor maintains a copy of the\nwl_surface contents, e.g. as a GL texture. This is an important\noptimization for GL(ES) compositors with wl_shm clients."]
    fn r#release(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 0u16);
        _stream.commit(_key)
    }
}
pub mod r#wl_buffer {}
#[doc = "`Version 3`"]
#[doc = ""]
#[doc = "Offer To Transfer Data"]
#[doc = ""]
#[doc = "A wl_data_offer represents a piece of data offered for transfer\nby another client (the source client). It is used by the\ncopy-and-paste and drag-and-drop mechanisms. The offer\ndescribes the different mime types that the data can be\nconverted to and provides the mechanism for transferring the\ndata directly from the source client."]
pub trait r#WlDataOffer<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_data_offer";
    const VERSION: ::core::primitive::u32 = 3u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                let r#serial = _stream.u32()?;
                let r#mime_type = _stream.string()?;
                Self::r#accept(_this, _event_loop, _client, r#serial, r#mime_type)
            }
            1u16 => {
                let _stream = _client.stream();
                let r#mime_type = _stream.string()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                let r#fd = _stream.file()?;
                Self::r#receive(_this, _event_loop, _client, r#mime_type, r#fd)
            }
            2u16 => {
                let _stream = _client.stream();
                Self::r#destroy(_this, _event_loop, _client)
            }
            3u16 => {
                let _stream = _client.stream();
                Self::r#finish(_this, _event_loop, _client)
            }
            4u16 => {
                let _stream = _client.stream();
                let r#dnd_actions = _stream.u32()?;
                let r#preferred_action = _stream.u32()?;
                Self::r#set_actions(
                    _this,
                    _event_loop,
                    _client,
                    r#dnd_actions,
                    r#preferred_action,
                )
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Accept One Of The Offered Mime Types"]
    #[doc = ""]
    #[doc = "Indicate that the client can accept the given mime type, or\nNULL for not accepted.\n\nFor objects of version 2 or older, this request is used by the\nclient to give feedback whether the client can receive the given\nmime type, or NULL if none is accepted; the feedback does not\ndetermine whether the drag-and-drop operation succeeds or not.\n\nFor objects of version 3 or newer, this request determines the\nfinal result of the drag-and-drop operation. If the end result\nis that no mime types were accepted, the drag-and-drop operation\nwill be cancelled and the corresponding drag source will receive\nwl_data_source.cancelled. Clients may still use this event in\nconjunction with wl_data_source.action for feedback."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: serial number of the accept request"]
    #[doc = "\n`mime_type`: mime type accepted by the client"]
    fn r#accept(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#serial: ::core::primitive::u32,
        r#mime_type: ::core::option::Option<::std::string::String>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Request That The Data Is Transferred"]
    #[doc = ""]
    #[doc = "To transfer the offered data, the client issues this request\nand indicates the mime type it wants to receive. The transfer\nhappens through the passed file descriptor (typically created\nwith the pipe system call). The source client writes the data\nin the mime type representation requested and then closes the\nfile descriptor.\n\nThe receiving client reads from the read end of the pipe until\nEOF and then closes its end, at which point the transfer is\ncomplete.\n\nThis request may happen multiple times for different mime types,\nboth before and after wl_data_device.drop. Drag-and-drop destination\nclients may preemptively fetch data or examine it more closely to\ndetermine acceptance."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`mime_type`: mime type desired by receiver"]
    #[doc = "\n`fd`: file descriptor for data transfer"]
    fn r#receive(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#mime_type: ::std::string::String,
        r#fd: ::wl::File,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Destroy Data Offer"]
    #[doc = ""]
    #[doc = "Destroy the data offer."]
    fn r#destroy(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "The Offer Will No Longer Be Used"]
    #[doc = ""]
    #[doc = "Notifies the compositor that the drag destination successfully\nfinished the drag-and-drop operation.\n\nUpon receiving this request, the compositor will emit\nwl_data_source.dnd_finished on the drag source client.\n\nIt is a client error to perform other requests than\nwl_data_offer.destroy after this one. It is also an error to perform\nthis request after a NULL mime type has been set in\nwl_data_offer.accept or no action was received through\nwl_data_offer.action.\n\nIf wl_data_offer.finish request is received for a non drag and drop\noperation, the invalid_finish protocol error is raised."]
    fn r#finish(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "Set The Available Preferred Drag And Drop Actions"]
    #[doc = ""]
    #[doc = "Sets the actions that the destination side client supports for\nthis operation. This request may trigger the emission of\nwl_data_source.action and wl_data_offer.action events if the compositor\nneeds to change the selected action.\n\nThis request can be called multiple times throughout the\ndrag-and-drop operation, typically in response to wl_data_device.enter\nor wl_data_device.motion events.\n\nThis request determines the final result of the drag-and-drop\noperation. If the end result is that no action is accepted,\nthe drag source will receive wl_data_source.cancelled.\n\nThe dnd_actions argument must contain only values expressed in the\nwl_data_device_manager.dnd_actions enum, and the preferred_action\nargument must only contain one of those values set, otherwise it\nwill result in a protocol error.\n\nWhile managing an \"ask\" action, the destination drag-and-drop client\nmay perform further wl_data_offer.receive requests, and is expected\nto perform one last wl_data_offer.set_actions request with a preferred\naction other than \"ask\" (and optionally wl_data_offer.accept) before\nrequesting wl_data_offer.finish, in order to convey the action selected\nby the user. If the preferred action is not in the\nwl_data_offer.source_actions mask, an error will be raised.\n\nIf the \"ask\" action is dismissed (e.g. user cancellation), the client\nis expected to perform wl_data_offer.destroy right away.\n\nThis request can only be made on drag-and-drop offers, a protocol error\nwill be raised otherwise."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`dnd_actions`: actions supported by the destination client"]
    #[doc = "\n`preferred_action`: action preferred by the destination client"]
    fn r#set_actions(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#dnd_actions: ::core::primitive::u32,
        r#preferred_action: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Advertise Offered Mime Type"]
    #[doc = ""]
    #[doc = "Sent immediately after creating the wl_data_offer object. One\nevent per offered mime type."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`mime_type`: offered mime type"]
    fn r#offer(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#mime_type: &'_ ::core::primitive::str,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 0u16);
        _stream.send_string(::core::option::Option::Some(r#mime_type))?;
        _stream.commit(_key)
    }
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "Notify The Source Side Available Actions"]
    #[doc = ""]
    #[doc = "This event indicates the actions offered by the data source. It\nwill be sent right after wl_data_device.enter, or anytime the source\nside changes its offered actions through wl_data_source.set_actions."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`source_actions`: actions offered by the data source"]
    fn r#source_actions(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#source_actions: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 1u16);
        _stream.send_u32(r#source_actions)?;
        _stream.commit(_key)
    }
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "Notify The Selected Action"]
    #[doc = ""]
    #[doc = "This event indicates the action selected by the compositor after\nmatching the source/destination side actions. Only one action (or\nnone) will be offered here.\n\nThis event can be emitted multiple times during the drag-and-drop\noperation in response to destination side action changes through\nwl_data_offer.set_actions.\n\nThis event will no longer be emitted after wl_data_device.drop\nhappened on the drag-and-drop destination, the client must\nhonor the last action received, or the last preferred one set\nthrough wl_data_offer.set_actions when handling an \"ask\" action.\n\nCompositors may also change the selected action on the fly, mainly\nin response to keyboard modifier changes during the drag-and-drop\noperation.\n\nThe most recent action received is always the valid one. Prior to\nreceiving wl_data_device.drop, the chosen action may change (e.g.\ndue to keyboard modifiers being pressed). At the time of receiving\nwl_data_device.drop the drag-and-drop destination must honor the\nlast action received.\n\nAction changes may still happen after wl_data_device.drop,\nespecially on \"ask\" actions, where the drag-and-drop destination\nmay choose another action afterwards. Action changes happening\nat this stage are always the result of inter-client negotiation, the\ncompositor shall no longer be able to induce a different action.\n\nUpon \"ask\" actions, it is expected that the drag-and-drop destination\nmay potentially choose a different action and/or mime type,\nbased on wl_data_offer.source_actions and finally chosen by the\nuser (e.g. popping up a menu with the available options). The\nfinal wl_data_offer.set_actions and wl_data_offer.accept requests\nmust happen before the call to wl_data_offer.finish."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`dnd_action`: action selected by the compositor"]
    fn r#action(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#dnd_action: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 2u16);
        _stream.send_u32(r#dnd_action)?;
        _stream.commit(_key)
    }
}
pub mod r#wl_data_offer {
    #[doc = ""]
    #[repr(transparent)]
    pub struct r#Error(u32);
    impl r#Error {
        #[doc = "Finish Request Was Called Untimely"]
        #[doc = ""]
        pub const r#INVALID_FINISH: Self = Self(0u32);
        #[doc = "Action Mask Contains Invalid Values"]
        #[doc = ""]
        pub const r#INVALID_ACTION_MASK: Self = Self(1u32);
        #[doc = "Action Argument Has An Invalid Value"]
        #[doc = ""]
        pub const r#INVALID_ACTION: Self = Self(2u32);
        #[doc = "Offer Doesn T Accept This Request"]
        #[doc = ""]
        pub const r#INVALID_OFFER: Self = Self(3u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Error {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Error {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "INVALID_FINISH", 0u32),
                1u32 => ::core::write!(f, "{}({})", "INVALID_ACTION_MASK", 1u32),
                2u32 => ::core::write!(f, "{}({})", "INVALID_ACTION", 2u32),
                3u32 => ::core::write!(f, "{}({})", "INVALID_OFFER", 3u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "`Version 3`"]
#[doc = ""]
#[doc = "Offer To Transfer Data"]
#[doc = ""]
#[doc = "The wl_data_source object is the source side of a wl_data_offer.\nIt is created by the source client in a data transfer and\nprovides a way to describe the offered data and a way to respond\nto requests to transfer the data."]
pub trait r#WlDataSource<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_data_source";
    const VERSION: ::core::primitive::u32 = 3u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                let r#mime_type = _stream.string()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                Self::r#offer(_this, _event_loop, _client, r#mime_type)
            }
            1u16 => {
                let _stream = _client.stream();
                Self::r#destroy(_this, _event_loop, _client)
            }
            2u16 => {
                let _stream = _client.stream();
                let r#dnd_actions = _stream.u32()?;
                Self::r#set_actions(_this, _event_loop, _client, r#dnd_actions)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Add An Offered Mime Type"]
    #[doc = ""]
    #[doc = "This request adds a mime type to the set of mime types\nadvertised to targets. Can be called several times to offer\nmultiple types."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`mime_type`: mime type offered by the data source"]
    fn r#offer(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#mime_type: ::std::string::String,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Destroy The Data Source"]
    #[doc = ""]
    #[doc = "Destroy the data source."]
    fn r#destroy(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "Set The Available Drag And Drop Actions"]
    #[doc = ""]
    #[doc = "Sets the actions that the source side client supports for this\noperation. This request may trigger wl_data_source.action and\nwl_data_offer.action events if the compositor needs to change the\nselected action.\n\nThe dnd_actions argument must contain only values expressed in the\nwl_data_device_manager.dnd_actions enum, otherwise it will result\nin a protocol error.\n\nThis request must be made once only, and can only be made on sources\nused in drag-and-drop, so it must be performed before\nwl_data_device.start_drag. Attempting to use the source other than\nfor drag-and-drop will raise a protocol error."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`dnd_actions`: actions supported by the data source"]
    fn r#set_actions(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#dnd_actions: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "A Target Accepts An Offered Mime Type"]
    #[doc = ""]
    #[doc = "Sent when a target accepts pointer_focus or motion events. If\na target does not accept any of the offered types, type is NULL.\n\nUsed for feedback during drag-and-drop."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`mime_type`: mime type accepted by the target"]
    fn r#target(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#mime_type: ::core::option::Option<&'_ ::core::primitive::str>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 0u16);
        _stream.send_string(r#mime_type)?;
        _stream.commit(_key)
    }
    #[doc = "Send The Data"]
    #[doc = ""]
    #[doc = "Request for data from the client. Send the data as the\nspecified mime type over the passed file descriptor, then\nclose it."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`mime_type`: mime type for the data"]
    #[doc = "\n`fd`: file descriptor for the data"]
    fn r#send(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#mime_type: &'_ ::core::primitive::str,
        r#fd: ::wl::Fd<'static>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 1u16);
        _stream.send_string(::core::option::Option::Some(r#mime_type))?;
        _stream.send_file(r#fd)?;
        _stream.commit(_key)
    }
    #[doc = "Selection Was Cancelled"]
    #[doc = ""]
    #[doc = "This data source is no longer valid. There are several reasons why\nthis could happen:\n\n- The data source has been replaced by another data source.\n- The drag-and-drop operation was performed, but the drop destination\ndid not accept any of the mime types offered through\nwl_data_source.target.\n- The drag-and-drop operation was performed, but the drop destination\ndid not select any of the actions present in the mask offered through\nwl_data_source.action.\n- The drag-and-drop operation was performed but didn't happen over a\nsurface.\n- The compositor cancelled the drag-and-drop operation (e.g. compositor\ndependent timeouts to avoid stale drag-and-drop transfers).\n\nThe client should clean up and destroy this data source.\n\nFor objects of version 2 or older, wl_data_source.cancelled will\nonly be emitted if the data source was replaced by another data\nsource."]
    fn r#cancelled(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 2u16);
        _stream.commit(_key)
    }
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "The Drag And Drop Operation Physically Finished"]
    #[doc = ""]
    #[doc = "The user performed the drop action. This event does not indicate\nacceptance, wl_data_source.cancelled may still be emitted afterwards\nif the drop destination does not accept any mime type.\n\nHowever, this event might however not be received if the compositor\ncancelled the drag-and-drop operation before this event could happen.\n\nNote that the data_source may still be used in the future and should\nnot be destroyed here."]
    fn r#dnd_drop_performed(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 3u16);
        _stream.commit(_key)
    }
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "The Drag And Drop Operation Concluded"]
    #[doc = ""]
    #[doc = "The drop destination finished interoperating with this data\nsource, so the client is now free to destroy this data source and\nfree all associated data.\n\nIf the action used to perform the operation was \"move\", the\nsource can now delete the transferred data."]
    fn r#dnd_finished(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 4u16);
        _stream.commit(_key)
    }
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "Notify The Selected Action"]
    #[doc = ""]
    #[doc = "This event indicates the action selected by the compositor after\nmatching the source/destination side actions. Only one action (or\nnone) will be offered here.\n\nThis event can be emitted multiple times during the drag-and-drop\noperation, mainly in response to destination side changes through\nwl_data_offer.set_actions, and as the data device enters/leaves\nsurfaces.\n\nIt is only possible to receive this event after\nwl_data_source.dnd_drop_performed if the drag-and-drop operation\nended in an \"ask\" action, in which case the final wl_data_source.action\nevent will happen immediately before wl_data_source.dnd_finished.\n\nCompositors may also change the selected action on the fly, mainly\nin response to keyboard modifier changes during the drag-and-drop\noperation.\n\nThe most recent action received is always the valid one. The chosen\naction may change alongside negotiation (e.g. an \"ask\" action can turn\ninto a \"move\" operation), so the effects of the final action must\nalways be applied in wl_data_offer.dnd_finished.\n\nClients can trigger cursor surface changes from this point, so\nthey reflect the current action."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`dnd_action`: action selected by the compositor"]
    fn r#action(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#dnd_action: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 5u16);
        _stream.send_u32(r#dnd_action)?;
        _stream.commit(_key)
    }
}
pub mod r#wl_data_source {
    #[doc = ""]
    #[repr(transparent)]
    pub struct r#Error(u32);
    impl r#Error {
        #[doc = "Action Mask Contains Invalid Values"]
        #[doc = ""]
        pub const r#INVALID_ACTION_MASK: Self = Self(0u32);
        #[doc = "Source Doesn T Accept This Request"]
        #[doc = ""]
        pub const r#INVALID_SOURCE: Self = Self(1u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Error {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Error {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "INVALID_ACTION_MASK", 0u32),
                1u32 => ::core::write!(f, "{}({})", "INVALID_SOURCE", 1u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "`Version 3`"]
#[doc = ""]
#[doc = "Data Transfer Device"]
#[doc = ""]
#[doc = "There is one wl_data_device per seat which can be obtained\nfrom the global wl_data_device_manager singleton.\n\nA wl_data_device provides access to inter-client data transfer\nmechanisms such as copy-and-paste and drag-and-drop."]
pub trait r#WlDataDevice<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_data_device";
    const VERSION: ::core::primitive::u32 = 3u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                let r#source = _stream.object()?;
                let r#origin = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                let r#icon = _stream.object()?;
                let r#serial = _stream.u32()?;
                Self::r#start_drag(
                    _this,
                    _event_loop,
                    _client,
                    r#source,
                    r#origin,
                    r#icon,
                    r#serial,
                )
            }
            1u16 => {
                let _stream = _client.stream();
                let r#source = _stream.object()?;
                let r#serial = _stream.u32()?;
                Self::r#set_selection(_this, _event_loop, _client, r#source, r#serial)
            }
            2u16 => {
                let _stream = _client.stream();
                Self::r#release(_this, _event_loop, _client)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Start Drag And Drop Operation"]
    #[doc = ""]
    #[doc = "This request asks the compositor to start a drag-and-drop\noperation on behalf of the client.\n\nThe source argument is the data source that provides the data\nfor the eventual data transfer. If source is NULL, enter, leave\nand motion events are sent only to the client that initiated the\ndrag and the client is expected to handle the data passing\ninternally. If source is destroyed, the drag-and-drop session will be\ncancelled.\n\nThe origin surface is the surface where the drag originates and\nthe client must have an active implicit grab that matches the\nserial.\n\nThe icon surface is an optional (can be NULL) surface that\nprovides an icon to be moved around with the cursor. Initially,\nthe top-left corner of the icon surface is placed at the cursor\nhotspot, but subsequent wl_surface.attach request can move the\nrelative position. Attach requests must be confirmed with\nwl_surface.commit as usual. The icon surface is given the role of\na drag-and-drop icon. If the icon surface already has another role,\nit raises a protocol error.\n\nThe current and pending input regions of the icon wl_surface are\ncleared, and wl_surface.set_input_region is ignored until the\nwl_surface is no longer used as the icon surface. When the use\nas an icon ends, the current and pending input regions become\nundefined, and the wl_surface is unmapped."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`source`: data source for the eventual transfer"]
    #[doc = "\n`origin`: surface where the drag originates"]
    #[doc = "\n`icon`: drag-and-drop icon surface"]
    #[doc = "\n`serial`: serial number of the implicit grab on the origin"]
    fn r#start_drag(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#source: ::core::option::Option<::wl::Id>,
        r#origin: ::wl::Id,
        r#icon: ::core::option::Option<::wl::Id>,
        r#serial: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Copy Data To The Selection"]
    #[doc = ""]
    #[doc = "This request asks the compositor to set the selection\nto the data from the source on behalf of the client.\n\nTo unset the selection, set the source to NULL."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`source`: data source for the selection"]
    #[doc = "\n`serial`: serial number of the event that triggered this request"]
    fn r#set_selection(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#source: ::core::option::Option<::wl::Id>,
        r#serial: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "`Since version 2`"]
    #[doc = ""]
    #[doc = "Destroy Data Device"]
    #[doc = ""]
    #[doc = "This request destroys the data device."]
    fn r#release(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Introduce A New Wl Data Offer"]
    #[doc = ""]
    #[doc = "The data_offer event introduces a new wl_data_offer object,\nwhich will subsequently be used in either the\ndata_device.enter event (for drag-and-drop) or the\ndata_device.selection event (for selections). Immediately\nfollowing the data_device.data_offer event, the new data_offer\nobject will send out data_offer.offer events to describe the\nmime types it offers."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`id`: the new data_offer object"]
    fn r#data_offer(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#id: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 0u16);
        _stream.send_object(Some(r#id))?;
        _stream.commit(_key)
    }
    #[doc = "Initiate Drag And Drop Session"]
    #[doc = ""]
    #[doc = "This event is sent when an active drag-and-drop pointer enters\na surface owned by the client. The position of the pointer at\nenter time is provided by the x and y arguments, in surface-local\ncoordinates."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: serial number of the enter event"]
    #[doc = "\n`surface`: client surface entered"]
    #[doc = "\n`x`: surface-local x coordinate"]
    #[doc = "\n`y`: surface-local y coordinate"]
    #[doc = "\n`id`: source data_offer object"]
    fn r#enter(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#serial: ::core::primitive::u32,
        r#surface: ::wl::Id,
        r#x: ::wl::Fixed,
        r#y: ::wl::Fixed,
        r#id: ::core::option::Option<::wl::Id>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 1u16);
        _stream.send_u32(r#serial)?;
        _stream.send_object(Some(r#surface))?;
        _stream.send_fixed(r#x)?;
        _stream.send_fixed(r#y)?;
        _stream.send_object(r#id)?;
        _stream.commit(_key)
    }
    #[doc = "End Drag And Drop Session"]
    #[doc = ""]
    #[doc = "This event is sent when the drag-and-drop pointer leaves the\nsurface and the session ends. The client must destroy the\nwl_data_offer introduced at enter time at this point."]
    fn r#leave(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 2u16);
        _stream.commit(_key)
    }
    #[doc = "Drag And Drop Session Motion"]
    #[doc = ""]
    #[doc = "This event is sent when the drag-and-drop pointer moves within\nthe currently focused surface. The new position of the pointer\nis provided by the x and y arguments, in surface-local\ncoordinates."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`time`: timestamp with millisecond granularity"]
    #[doc = "\n`x`: surface-local x coordinate"]
    #[doc = "\n`y`: surface-local y coordinate"]
    fn r#motion(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#time: ::core::primitive::u32,
        r#x: ::wl::Fixed,
        r#y: ::wl::Fixed,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 3u16);
        _stream.send_u32(r#time)?;
        _stream.send_fixed(r#x)?;
        _stream.send_fixed(r#y)?;
        _stream.commit(_key)
    }
    #[doc = "End Drag And Drop Session Successfully"]
    #[doc = ""]
    #[doc = "The event is sent when a drag-and-drop operation is ended\nbecause the implicit grab is removed.\n\nThe drag-and-drop destination is expected to honor the last action\nreceived through wl_data_offer.action, if the resulting action is\n\"copy\" or \"move\", the destination can still perform\nwl_data_offer.receive requests, and is expected to end all\ntransfers with a wl_data_offer.finish request.\n\nIf the resulting action is \"ask\", the action will not be considered\nfinal. The drag-and-drop destination is expected to perform one last\nwl_data_offer.set_actions request, or wl_data_offer.destroy in order\nto cancel the operation."]
    fn r#drop(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 4u16);
        _stream.commit(_key)
    }
    #[doc = "Advertise New Selection"]
    #[doc = ""]
    #[doc = "The selection event is sent out to notify the client of a new\nwl_data_offer for the selection for this device. The\ndata_device.data_offer and the data_offer.offer events are\nsent out immediately before this event to introduce the data\noffer object. The selection event is sent to a client\nimmediately before receiving keyboard focus and when a new\nselection is set while the client has keyboard focus. The\ndata_offer is valid until a new data_offer or NULL is received\nor until the client loses keyboard focus. Switching surface with\nkeyboard focus within the same client doesn't mean a new selection\nwill be sent. The client must destroy the previous selection\ndata_offer, if any, upon receiving this event."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`id`: selection data_offer object"]
    fn r#selection(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#id: ::core::option::Option<::wl::Id>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 5u16);
        _stream.send_object(r#id)?;
        _stream.commit(_key)
    }
}
pub mod r#wl_data_device {
    #[doc = ""]
    #[repr(transparent)]
    pub struct r#Error(u32);
    impl r#Error {
        #[doc = "Given Wl Surface Has Another Role"]
        #[doc = ""]
        pub const r#ROLE: Self = Self(0u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Error {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Error {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "ROLE", 0u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "`Version 3`"]
#[doc = ""]
#[doc = "Data Transfer Interface"]
#[doc = ""]
#[doc = "The wl_data_device_manager is a singleton global object that\nprovides access to inter-client data transfer mechanisms such as\ncopy-and-paste and drag-and-drop. These mechanisms are tied to\na wl_seat and this interface lets a client get a wl_data_device\ncorresponding to a wl_seat.\n\nDepending on the version bound, the objects created from the bound\nwl_data_device_manager object will have different requirements for\nfunctioning properly. See wl_data_source.set_actions,\nwl_data_offer.accept and wl_data_offer.finish for details."]
pub trait r#WlDataDeviceManager<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_data_device_manager";
    const VERSION: ::core::primitive::u32 = 3u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                let r#id = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                Self::r#create_data_source(_this, _event_loop, _client, r#id)
            }
            1u16 => {
                let _stream = _client.stream();
                let r#id = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                let r#seat = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                Self::r#get_data_device(_this, _event_loop, _client, r#id, r#seat)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Create A New Data Source"]
    #[doc = ""]
    #[doc = "Create a new data source."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`id`: data source to create"]
    fn r#create_data_source(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#id: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Create A New Data Device"]
    #[doc = ""]
    #[doc = "Create a new data device for a given seat."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`id`: data device to create"]
    #[doc = "\n`seat`: seat associated with the data device"]
    fn r#get_data_device(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#id: ::wl::Id,
        r#seat: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
}
pub mod r#wl_data_device_manager {
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "Drag And Drop Actions"]
    #[doc = ""]
    #[doc = "This is a bitmask of the available/preferred actions in a\ndrag-and-drop operation.\n\nIn the compositor, the selected action is a result of matching the\nactions offered by the source and destination sides. \"action\" events\nwith a \"none\" action will be sent to both source and destination if\nthere is no match. All further checks will effectively happen on\n(source actions ∩ destination actions).\n\nIn addition, compositors may also pick different actions in\nreaction to key modifiers being pressed. One common design that\nis used in major toolkits (and the behavior recommended for\ncompositors) is:\n\n- If no modifiers are pressed, the first match (in bit order)\nwill be used.\n- Pressing Shift selects \"move\", if enabled in the mask.\n- Pressing Control selects \"copy\", if enabled in the mask.\n\nBehavior beyond that is considered implementation-dependent.\nCompositors may for example bind other modifiers (like Alt/Meta)\nor drags initiated with other buttons than BTN_LEFT to specific\nactions (e.g. \"ask\")."]
    #[repr(transparent)]
    pub struct r#DndAction(u32);
    impl r#DndAction {
        #[doc = "No Action"]
        #[doc = ""]
        pub const r#NONE: Self = Self(0u32);
        #[doc = "Copy Action"]
        #[doc = ""]
        pub const r#COPY: Self = Self(1u32);
        #[doc = "Move Action"]
        #[doc = ""]
        pub const r#MOVE: Self = Self(2u32);
        #[doc = "Ask Action"]
        #[doc = ""]
        pub const r#ASK: Self = Self(4u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#DndAction {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#DndAction {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#DndAction {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "NONE", 0u32),
                1u32 => ::core::write!(f, "{}({})", "COPY", 1u32),
                2u32 => ::core::write!(f, "{}({})", "MOVE", 2u32),
                4u32 => ::core::write!(f, "{}({})", "ASK", 4u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "`Version 1`"]
#[doc = ""]
#[doc = "Create Desktop Style Surfaces"]
#[doc = ""]
#[doc = "This interface is implemented by servers that provide\ndesktop-style user interfaces.\n\nIt allows clients to associate a wl_shell_surface with\na basic surface.\n\nNote! This protocol is deprecated and not intended for production use.\nFor desktop-style user interfaces, use xdg_shell. Compositors and clients\nshould not implement this interface."]
pub trait r#WlShell<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_shell";
    const VERSION: ::core::primitive::u32 = 1u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                let r#id = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                let r#surface = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                Self::r#get_shell_surface(_this, _event_loop, _client, r#id, r#surface)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Create A Shell Surface From A Surface"]
    #[doc = ""]
    #[doc = "Create a shell surface for an existing surface. This gives\nthe wl_surface the role of a shell surface. If the wl_surface\nalready has another role, it raises a protocol error.\n\nOnly one shell surface can be associated with a given surface."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`id`: shell surface to create"]
    #[doc = "\n`surface`: surface to be given the shell surface role"]
    fn r#get_shell_surface(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#id: ::wl::Id,
        r#surface: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
}
pub mod r#wl_shell {
    #[doc = ""]
    #[repr(transparent)]
    pub struct r#Error(u32);
    impl r#Error {
        #[doc = "Given Wl Surface Has Another Role"]
        #[doc = ""]
        pub const r#ROLE: Self = Self(0u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Error {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Error {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "ROLE", 0u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "`Version 1`"]
#[doc = ""]
#[doc = "Desktop Style Metadata Interface"]
#[doc = ""]
#[doc = "An interface that may be implemented by a wl_surface, for\nimplementations that provide a desktop-style user interface.\n\nIt provides requests to treat surfaces like toplevel, fullscreen\nor popup windows, move, resize or maximize them, associate\nmetadata like title and class, etc.\n\nOn the server side the object is automatically destroyed when\nthe related wl_surface is destroyed. On the client side,\nwl_shell_surface_destroy() must be called before destroying\nthe wl_surface object."]
pub trait r#WlShellSurface<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_shell_surface";
    const VERSION: ::core::primitive::u32 = 1u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                let r#serial = _stream.u32()?;
                Self::r#pong(_this, _event_loop, _client, r#serial)
            }
            1u16 => {
                let _stream = _client.stream();
                let r#seat = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                let r#serial = _stream.u32()?;
                Self::r#move(_this, _event_loop, _client, r#seat, r#serial)
            }
            2u16 => {
                let _stream = _client.stream();
                let r#seat = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                let r#serial = _stream.u32()?;
                let r#edges = _stream.u32()?;
                Self::r#resize(_this, _event_loop, _client, r#seat, r#serial, r#edges)
            }
            3u16 => {
                let _stream = _client.stream();
                Self::r#set_toplevel(_this, _event_loop, _client)
            }
            4u16 => {
                let _stream = _client.stream();
                let r#parent = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                let r#x = _stream.i32()?;
                let r#y = _stream.i32()?;
                let r#flags = _stream.u32()?;
                Self::r#set_transient(_this, _event_loop, _client, r#parent, r#x, r#y, r#flags)
            }
            5u16 => {
                let _stream = _client.stream();
                let r#method = _stream.u32()?;
                let r#framerate = _stream.u32()?;
                let r#output = _stream.object()?;
                Self::r#set_fullscreen(_this, _event_loop, _client, r#method, r#framerate, r#output)
            }
            6u16 => {
                let _stream = _client.stream();
                let r#seat = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                let r#serial = _stream.u32()?;
                let r#parent = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                let r#x = _stream.i32()?;
                let r#y = _stream.i32()?;
                let r#flags = _stream.u32()?;
                Self::r#set_popup(
                    _this,
                    _event_loop,
                    _client,
                    r#seat,
                    r#serial,
                    r#parent,
                    r#x,
                    r#y,
                    r#flags,
                )
            }
            7u16 => {
                let _stream = _client.stream();
                let r#output = _stream.object()?;
                Self::r#set_maximized(_this, _event_loop, _client, r#output)
            }
            8u16 => {
                let _stream = _client.stream();
                let r#title = _stream.string()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                Self::r#set_title(_this, _event_loop, _client, r#title)
            }
            9u16 => {
                let _stream = _client.stream();
                let r#class = _stream.string()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                Self::r#set_class(_this, _event_loop, _client, r#class)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Respond To A Ping Event"]
    #[doc = ""]
    #[doc = "A client must respond to a ping event with a pong request or\nthe client may be deemed unresponsive."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: serial number of the ping event"]
    fn r#pong(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#serial: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Start An Interactive Move"]
    #[doc = ""]
    #[doc = "Start a pointer-driven move of the surface.\n\nThis request must be used in response to a button press event.\nThe server may ignore move requests depending on the state of\nthe surface (e.g. fullscreen or maximized)."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`seat`: seat whose pointer is used"]
    #[doc = "\n`serial`: serial number of the implicit grab on the pointer"]
    fn r#move(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#seat: ::wl::Id,
        r#serial: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Start An Interactive Resize"]
    #[doc = ""]
    #[doc = "Start a pointer-driven resizing of the surface.\n\nThis request must be used in response to a button press event.\nThe server may ignore resize requests depending on the state of\nthe surface (e.g. fullscreen or maximized)."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`seat`: seat whose pointer is used"]
    #[doc = "\n`serial`: serial number of the implicit grab on the pointer"]
    #[doc = "\n`edges`: which edge or corner is being dragged"]
    fn r#resize(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#seat: ::wl::Id,
        r#serial: ::core::primitive::u32,
        r#edges: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Make The Surface A Toplevel Surface"]
    #[doc = ""]
    #[doc = "Map the surface as a toplevel surface.\n\nA toplevel surface is not fullscreen, maximized or transient."]
    fn r#set_toplevel(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Make The Surface A Transient Surface"]
    #[doc = ""]
    #[doc = "Map the surface relative to an existing surface.\n\nThe x and y arguments specify the location of the upper left\ncorner of the surface relative to the upper left corner of the\nparent surface, in surface-local coordinates.\n\nThe flags argument controls details of the transient behaviour."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`parent`: parent surface"]
    #[doc = "\n`x`: surface-local x coordinate"]
    #[doc = "\n`y`: surface-local y coordinate"]
    #[doc = "\n`flags`: transient surface behavior"]
    fn r#set_transient(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#parent: ::wl::Id,
        r#x: ::core::primitive::i32,
        r#y: ::core::primitive::i32,
        r#flags: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Make The Surface A Fullscreen Surface"]
    #[doc = ""]
    #[doc = "Map the surface as a fullscreen surface.\n\nIf an output parameter is given then the surface will be made\nfullscreen on that output. If the client does not specify the\noutput then the compositor will apply its policy - usually\nchoosing the output on which the surface has the biggest surface\narea.\n\nThe client may specify a method to resolve a size conflict\nbetween the output size and the surface size - this is provided\nthrough the method parameter.\n\nThe framerate parameter is used only when the method is set\nto \"driver\", to indicate the preferred framerate. A value of 0\nindicates that the client does not care about framerate. The\nframerate is specified in mHz, that is framerate of 60000 is 60Hz.\n\nA method of \"scale\" or \"driver\" implies a scaling operation of\nthe surface, either via a direct scaling operation or a change of\nthe output mode. This will override any kind of output scaling, so\nthat mapping a surface with a buffer size equal to the mode can\nfill the screen independent of buffer_scale.\n\nA method of \"fill\" means we don't scale up the buffer, however\nany output scale is applied. This means that you may run into\nan edge case where the application maps a buffer with the same\nsize of the output mode but buffer_scale 1 (thus making a\nsurface larger than the output). In this case it is allowed to\ndownscale the results to fit the screen.\n\nThe compositor must reply to this request with a configure event\nwith the dimensions for the output on which the surface will\nbe made fullscreen."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`method`: method for resolving size conflict"]
    #[doc = "\n`framerate`: framerate in mHz"]
    #[doc = "\n`output`: output on which the surface is to be fullscreen"]
    fn r#set_fullscreen(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#method: ::core::primitive::u32,
        r#framerate: ::core::primitive::u32,
        r#output: ::core::option::Option<::wl::Id>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Make The Surface A Popup Surface"]
    #[doc = ""]
    #[doc = "Map the surface as a popup.\n\nA popup surface is a transient surface with an added pointer\ngrab.\n\nAn existing implicit grab will be changed to owner-events mode,\nand the popup grab will continue after the implicit grab ends\n(i.e. releasing the mouse button does not cause the popup to\nbe unmapped).\n\nThe popup grab continues until the window is destroyed or a\nmouse button is pressed in any other client's window. A click\nin any of the client's surfaces is reported as normal, however,\nclicks in other clients' surfaces will be discarded and trigger\nthe callback.\n\nThe x and y arguments specify the location of the upper left\ncorner of the surface relative to the upper left corner of the\nparent surface, in surface-local coordinates."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`seat`: seat whose pointer is used"]
    #[doc = "\n`serial`: serial number of the implicit grab on the pointer"]
    #[doc = "\n`parent`: parent surface"]
    #[doc = "\n`x`: surface-local x coordinate"]
    #[doc = "\n`y`: surface-local y coordinate"]
    #[doc = "\n`flags`: transient surface behavior"]
    fn r#set_popup(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#seat: ::wl::Id,
        r#serial: ::core::primitive::u32,
        r#parent: ::wl::Id,
        r#x: ::core::primitive::i32,
        r#y: ::core::primitive::i32,
        r#flags: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Make The Surface A Maximized Surface"]
    #[doc = ""]
    #[doc = "Map the surface as a maximized surface.\n\nIf an output parameter is given then the surface will be\nmaximized on that output. If the client does not specify the\noutput then the compositor will apply its policy - usually\nchoosing the output on which the surface has the biggest surface\narea.\n\nThe compositor will reply with a configure event telling\nthe expected new surface size. The operation is completed\non the next buffer attach to this surface.\n\nA maximized surface typically fills the entire output it is\nbound to, except for desktop elements such as panels. This is\nthe main difference between a maximized shell surface and a\nfullscreen shell surface.\n\nThe details depend on the compositor implementation."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`output`: output on which the surface is to be maximized"]
    fn r#set_maximized(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#output: ::core::option::Option<::wl::Id>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Set Surface Title"]
    #[doc = ""]
    #[doc = "Set a short title for the surface.\n\nThis string may be used to identify the surface in a task bar,\nwindow list, or other user interface elements provided by the\ncompositor.\n\nThe string must be encoded in UTF-8."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`title`: surface title"]
    fn r#set_title(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#title: ::std::string::String,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Set Surface Class"]
    #[doc = ""]
    #[doc = "Set a class for the surface.\n\nThe surface class identifies the general class of applications\nto which the surface belongs. A common convention is to use the\nfile name (or the full path if it is a non-standard location) of\nthe application's .desktop file as the class."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`class_`: surface class"]
    fn r#set_class(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#class: ::std::string::String,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Ping Client"]
    #[doc = ""]
    #[doc = "Ping a client to check if it is receiving events and sending\nrequests. A client is expected to reply with a pong request."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: serial number of the ping"]
    fn r#ping(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#serial: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 0u16);
        _stream.send_u32(r#serial)?;
        _stream.commit(_key)
    }
    #[doc = "Suggest Resize"]
    #[doc = ""]
    #[doc = "The configure event asks the client to resize its surface.\n\nThe size is a hint, in the sense that the client is free to\nignore it if it doesn't resize, pick a smaller size (to\nsatisfy aspect ratio or resize in steps of NxM pixels).\n\nThe edges parameter provides a hint about how the surface\nwas resized. The client may use this information to decide\nhow to adjust its content to the new size (e.g. a scrolling\narea might adjust its content position to leave the viewable\ncontent unmoved).\n\nThe client is free to dismiss all but the last configure\nevent it received.\n\nThe width and height arguments specify the size of the window\nin surface-local coordinates."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`edges`: how the surface was resized"]
    #[doc = "\n`width`: new width of the surface"]
    #[doc = "\n`height`: new height of the surface"]
    fn r#configure(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#edges: ::core::primitive::u32,
        r#width: ::core::primitive::i32,
        r#height: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 1u16);
        _stream.send_u32(r#edges)?;
        _stream.send_i32(r#width)?;
        _stream.send_i32(r#height)?;
        _stream.commit(_key)
    }
    #[doc = "Popup Interaction Is Done"]
    #[doc = ""]
    #[doc = "The popup_done event is sent out when a popup grab is broken,\nthat is, when the user clicks a surface that doesn't belong\nto the client owning the popup surface."]
    fn r#popup_done(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 2u16);
        _stream.commit(_key)
    }
}
pub mod r#wl_shell_surface {
    #[doc = "Edge Values For Resizing"]
    #[doc = ""]
    #[doc = "These values are used to indicate which edge of a surface\nis being dragged in a resize operation. The server may\nuse this information to adapt its behavior, e.g. choose\nan appropriate cursor image."]
    #[repr(transparent)]
    pub struct r#Resize(u32);
    impl r#Resize {
        #[doc = "No Edge"]
        #[doc = ""]
        pub const r#NONE: Self = Self(0u32);
        #[doc = "Top Edge"]
        #[doc = ""]
        pub const r#TOP: Self = Self(1u32);
        #[doc = "Bottom Edge"]
        #[doc = ""]
        pub const r#BOTTOM: Self = Self(2u32);
        #[doc = "Left Edge"]
        #[doc = ""]
        pub const r#LEFT: Self = Self(4u32);
        #[doc = "Top And Left Edges"]
        #[doc = ""]
        pub const r#TOP_LEFT: Self = Self(5u32);
        #[doc = "Bottom And Left Edges"]
        #[doc = ""]
        pub const r#BOTTOM_LEFT: Self = Self(6u32);
        #[doc = "Right Edge"]
        #[doc = ""]
        pub const r#RIGHT: Self = Self(8u32);
        #[doc = "Top And Right Edges"]
        #[doc = ""]
        pub const r#TOP_RIGHT: Self = Self(9u32);
        #[doc = "Bottom And Right Edges"]
        #[doc = ""]
        pub const r#BOTTOM_RIGHT: Self = Self(10u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Resize {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Resize {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Resize {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "NONE", 0u32),
                1u32 => ::core::write!(f, "{}({})", "TOP", 1u32),
                2u32 => ::core::write!(f, "{}({})", "BOTTOM", 2u32),
                4u32 => ::core::write!(f, "{}({})", "LEFT", 4u32),
                5u32 => ::core::write!(f, "{}({})", "TOP_LEFT", 5u32),
                6u32 => ::core::write!(f, "{}({})", "BOTTOM_LEFT", 6u32),
                8u32 => ::core::write!(f, "{}({})", "RIGHT", 8u32),
                9u32 => ::core::write!(f, "{}({})", "TOP_RIGHT", 9u32),
                10u32 => ::core::write!(f, "{}({})", "BOTTOM_RIGHT", 10u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
    #[doc = "Details Of Transient Behaviour"]
    #[doc = ""]
    #[doc = "These flags specify details of the expected behaviour\nof transient surfaces. Used in the set_transient request."]
    #[repr(transparent)]
    pub struct r#Transient(u32);
    impl r#Transient {
        #[doc = "Do Not Set Keyboard Focus"]
        #[doc = ""]
        pub const r#INACTIVE: Self = Self(1u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Transient {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Transient {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Transient {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                1u32 => ::core::write!(f, "{}({})", "INACTIVE", 1u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
    #[doc = "Different Method To Set The Surface Fullscreen"]
    #[doc = ""]
    #[doc = "Hints to indicate to the compositor how to deal with a conflict\nbetween the dimensions of the surface and the dimensions of the\noutput. The compositor is free to ignore this parameter."]
    #[repr(transparent)]
    pub struct r#FullscreenMethod(u32);
    impl r#FullscreenMethod {
        #[doc = "No Preference Apply Default Policy"]
        #[doc = ""]
        pub const r#DEFAULT: Self = Self(0u32);
        #[doc = "Scale Preserve The Surface S Aspect Ratio And Center On Output"]
        #[doc = ""]
        pub const r#SCALE: Self = Self(1u32);
        #[doc = "Switch Output Mode To The Smallest Mode That Can Fit The Surface Add Black Borders To Compensate Size Mismatch"]
        #[doc = ""]
        pub const r#DRIVER: Self = Self(2u32);
        #[doc = "No Upscaling Center On Output And Add Black Borders To Compensate Size Mismatch"]
        #[doc = ""]
        pub const r#FILL: Self = Self(3u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#FullscreenMethod {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#FullscreenMethod {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#FullscreenMethod {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "DEFAULT", 0u32),
                1u32 => ::core::write!(f, "{}({})", "SCALE", 1u32),
                2u32 => ::core::write!(f, "{}({})", "DRIVER", 2u32),
                3u32 => ::core::write!(f, "{}({})", "FILL", 3u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "`Version 5`"]
#[doc = ""]
#[doc = "An Onscreen Surface"]
#[doc = ""]
#[doc = "A surface is a rectangular area that may be displayed on zero\nor more outputs, and shown any number of times at the compositor's\ndiscretion. They can present wl_buffers, receive user input, and\ndefine a local coordinate system.\n\nThe size of a surface (and relative positions on it) is described\nin surface-local coordinates, which may differ from the buffer\ncoordinates of the pixel content, in case a buffer_transform\nor a buffer_scale is used.\n\nA surface without a \"role\" is fairly useless: a compositor does\nnot know where, when or how to present it. The role is the\npurpose of a wl_surface. Examples of roles are a cursor for a\npointer (as set by wl_pointer.set_cursor), a drag icon\n(wl_data_device.start_drag), a sub-surface\n(wl_subcompositor.get_subsurface), and a window as defined by a\nshell protocol (e.g. wl_shell.get_shell_surface).\n\nA surface can have only one role at a time. Initially a\nwl_surface does not have a role. Once a wl_surface is given a\nrole, it is set permanently for the whole lifetime of the\nwl_surface object. Giving the current role again is allowed,\nunless explicitly forbidden by the relevant interface\nspecification.\n\nSurface roles are given by requests in other interfaces such as\nwl_pointer.set_cursor. The request should explicitly mention\nthat this request gives a role to a wl_surface. Often, this\nrequest also creates a new protocol object that represents the\nrole and adds additional functionality to wl_surface. When a\nclient wants to destroy a wl_surface, they must destroy this 'role\nobject' before the wl_surface.\n\nDestroying the role object does not remove the role from the\nwl_surface, but it may stop the wl_surface from \"playing the role\".\nFor instance, if a wl_subsurface object is destroyed, the wl_surface\nit was created for will be unmapped and forget its position and\nz-order. It is allowed to create a wl_subsurface for the same\nwl_surface again, but it is not allowed to use the wl_surface as\na cursor (cursor is a different role than sub-surface, and role\nswitching is not allowed)."]
pub trait r#WlSurface<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_surface";
    const VERSION: ::core::primitive::u32 = 5u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                Self::r#destroy(_this, _event_loop, _client)
            }
            1u16 => {
                let _stream = _client.stream();
                let r#buffer = _stream.object()?;
                let r#x = _stream.i32()?;
                let r#y = _stream.i32()?;
                Self::r#attach(_this, _event_loop, _client, r#buffer, r#x, r#y)
            }
            2u16 => {
                let _stream = _client.stream();
                let r#x = _stream.i32()?;
                let r#y = _stream.i32()?;
                let r#width = _stream.i32()?;
                let r#height = _stream.i32()?;
                Self::r#damage(_this, _event_loop, _client, r#x, r#y, r#width, r#height)
            }
            3u16 => {
                let _stream = _client.stream();
                let r#callback = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                Self::r#frame(_this, _event_loop, _client, r#callback)
            }
            4u16 => {
                let _stream = _client.stream();
                let r#region = _stream.object()?;
                Self::r#set_opaque_region(_this, _event_loop, _client, r#region)
            }
            5u16 => {
                let _stream = _client.stream();
                let r#region = _stream.object()?;
                Self::r#set_input_region(_this, _event_loop, _client, r#region)
            }
            6u16 => {
                let _stream = _client.stream();
                Self::r#commit(_this, _event_loop, _client)
            }
            7u16 => {
                let _stream = _client.stream();
                let r#transform = _stream.i32()?;
                Self::r#set_buffer_transform(_this, _event_loop, _client, r#transform)
            }
            8u16 => {
                let _stream = _client.stream();
                let r#scale = _stream.i32()?;
                Self::r#set_buffer_scale(_this, _event_loop, _client, r#scale)
            }
            9u16 => {
                let _stream = _client.stream();
                let r#x = _stream.i32()?;
                let r#y = _stream.i32()?;
                let r#width = _stream.i32()?;
                let r#height = _stream.i32()?;
                Self::r#damage_buffer(_this, _event_loop, _client, r#x, r#y, r#width, r#height)
            }
            10u16 => {
                let _stream = _client.stream();
                let r#x = _stream.i32()?;
                let r#y = _stream.i32()?;
                Self::r#offset(_this, _event_loop, _client, r#x, r#y)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Delete Surface"]
    #[doc = ""]
    #[doc = "Deletes the surface and invalidates its object ID."]
    fn r#destroy(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Set The Surface Contents"]
    #[doc = ""]
    #[doc = "Set a buffer as the content of this surface.\n\nThe new size of the surface is calculated based on the buffer\nsize transformed by the inverse buffer_transform and the\ninverse buffer_scale. This means that at commit time the supplied\nbuffer size must be an integer multiple of the buffer_scale. If\nthat's not the case, an invalid_size error is sent.\n\nThe x and y arguments specify the location of the new pending\nbuffer's upper left corner, relative to the current buffer's upper\nleft corner, in surface-local coordinates. In other words, the\nx and y, combined with the new surface size define in which\ndirections the surface's size changes. Setting anything other than 0\nas x and y arguments is discouraged, and should instead be replaced\nwith using the separate wl_surface.offset request.\n\nWhen the bound wl_surface version is 5 or higher, passing any\nnon-zero x or y is a protocol violation, and will result in an\n'invalid_offset' error being raised. To achieve equivalent semantics,\nuse wl_surface.offset.\n\nSurface contents are double-buffered state, see wl_surface.commit.\n\nThe initial surface contents are void; there is no content.\nwl_surface.attach assigns the given wl_buffer as the pending\nwl_buffer. wl_surface.commit makes the pending wl_buffer the new\nsurface contents, and the size of the surface becomes the size\ncalculated from the wl_buffer, as described above. After commit,\nthere is no pending buffer until the next attach.\n\nCommitting a pending wl_buffer allows the compositor to read the\npixels in the wl_buffer. The compositor may access the pixels at\nany time after the wl_surface.commit request. When the compositor\nwill not access the pixels anymore, it will send the\nwl_buffer.release event. Only after receiving wl_buffer.release,\nthe client may reuse the wl_buffer. A wl_buffer that has been\nattached and then replaced by another attach instead of committed\nwill not receive a release event, and is not used by the\ncompositor.\n\nIf a pending wl_buffer has been committed to more than one wl_surface,\nthe delivery of wl_buffer.release events becomes undefined. A well\nbehaved client should not rely on wl_buffer.release events in this\ncase. Alternatively, a client could create multiple wl_buffer objects\nfrom the same backing storage or use wp_linux_buffer_release.\n\nDestroying the wl_buffer after wl_buffer.release does not change\nthe surface contents. Destroying the wl_buffer before wl_buffer.release\nis allowed as long as the underlying buffer storage isn't re-used (this\ncan happen e.g. on client process termination). However, if the client\ndestroys the wl_buffer before receiving the wl_buffer.release event and\nmutates the underlying buffer storage, the surface contents become\nundefined immediately.\n\nIf wl_surface.attach is sent with a NULL wl_buffer, the\nfollowing wl_surface.commit will remove the surface content."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`buffer`: buffer of surface contents"]
    #[doc = "\n`x`: surface-local x coordinate"]
    #[doc = "\n`y`: surface-local y coordinate"]
    fn r#attach(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#buffer: ::core::option::Option<::wl::Id>,
        r#x: ::core::primitive::i32,
        r#y: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Mark Part Of The Surface Damaged"]
    #[doc = ""]
    #[doc = "This request is used to describe the regions where the pending\nbuffer is different from the current surface contents, and where\nthe surface therefore needs to be repainted. The compositor\nignores the parts of the damage that fall outside of the surface.\n\nDamage is double-buffered state, see wl_surface.commit.\n\nThe damage rectangle is specified in surface-local coordinates,\nwhere x and y specify the upper left corner of the damage rectangle.\n\nThe initial value for pending damage is empty: no damage.\nwl_surface.damage adds pending damage: the new pending damage\nis the union of old pending damage and the given rectangle.\n\nwl_surface.commit assigns pending damage as the current damage,\nand clears pending damage. The server will clear the current\ndamage as it repaints the surface.\n\nNote! New clients should not use this request. Instead damage can be\nposted with wl_surface.damage_buffer which uses buffer coordinates\ninstead of surface coordinates."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`x`: surface-local x coordinate"]
    #[doc = "\n`y`: surface-local y coordinate"]
    #[doc = "\n`width`: width of damage rectangle"]
    #[doc = "\n`height`: height of damage rectangle"]
    fn r#damage(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#x: ::core::primitive::i32,
        r#y: ::core::primitive::i32,
        r#width: ::core::primitive::i32,
        r#height: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Request A Frame Throttling Hint"]
    #[doc = ""]
    #[doc = "Request a notification when it is a good time to start drawing a new\nframe, by creating a frame callback. This is useful for throttling\nredrawing operations, and driving animations.\n\nWhen a client is animating on a wl_surface, it can use the 'frame'\nrequest to get notified when it is a good time to draw and commit the\nnext frame of animation. If the client commits an update earlier than\nthat, it is likely that some updates will not make it to the display,\nand the client is wasting resources by drawing too often.\n\nThe frame request will take effect on the next wl_surface.commit.\nThe notification will only be posted for one frame unless\nrequested again. For a wl_surface, the notifications are posted in\nthe order the frame requests were committed.\n\nThe server must send the notifications so that a client\nwill not send excessive updates, while still allowing\nthe highest possible update rate for clients that wait for the reply\nbefore drawing again. The server should give some time for the client\nto draw and commit after sending the frame callback events to let it\nhit the next output refresh.\n\nA server should avoid signaling the frame callbacks if the\nsurface is not visible in any way, e.g. the surface is off-screen,\nor completely obscured by other opaque surfaces.\n\nThe object returned by this request will be destroyed by the\ncompositor after the callback is fired and as such the client must not\nattempt to use it after that point.\n\nThe callback_data passed in the callback is the current time, in\nmilliseconds, with an undefined base."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`callback`: callback object for the frame request"]
    fn r#frame(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#callback: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Set Opaque Region"]
    #[doc = ""]
    #[doc = "This request sets the region of the surface that contains\nopaque content.\n\nThe opaque region is an optimization hint for the compositor\nthat lets it optimize the redrawing of content behind opaque\nregions. Setting an opaque region is not required for correct\nbehaviour, but marking transparent content as opaque will result\nin repaint artifacts.\n\nThe opaque region is specified in surface-local coordinates.\n\nThe compositor ignores the parts of the opaque region that fall\noutside of the surface.\n\nOpaque region is double-buffered state, see wl_surface.commit.\n\nwl_surface.set_opaque_region changes the pending opaque region.\nwl_surface.commit copies the pending region to the current region.\nOtherwise, the pending and current regions are never changed.\n\nThe initial value for an opaque region is empty. Setting the pending\nopaque region has copy semantics, and the wl_region object can be\ndestroyed immediately. A NULL wl_region causes the pending opaque\nregion to be set to empty."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`region`: opaque region of the surface"]
    fn r#set_opaque_region(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#region: ::core::option::Option<::wl::Id>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Set Input Region"]
    #[doc = ""]
    #[doc = "This request sets the region of the surface that can receive\npointer and touch events.\n\nInput events happening outside of this region will try the next\nsurface in the server surface stack. The compositor ignores the\nparts of the input region that fall outside of the surface.\n\nThe input region is specified in surface-local coordinates.\n\nInput region is double-buffered state, see wl_surface.commit.\n\nwl_surface.set_input_region changes the pending input region.\nwl_surface.commit copies the pending region to the current region.\nOtherwise the pending and current regions are never changed,\nexcept cursor and icon surfaces are special cases, see\nwl_pointer.set_cursor and wl_data_device.start_drag.\n\nThe initial value for an input region is infinite. That means the\nwhole surface will accept input. Setting the pending input region\nhas copy semantics, and the wl_region object can be destroyed\nimmediately. A NULL wl_region causes the input region to be set\nto infinite."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`region`: input region of the surface"]
    fn r#set_input_region(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#region: ::core::option::Option<::wl::Id>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Commit Pending Surface State"]
    #[doc = ""]
    #[doc = "Surface state (input, opaque, and damage regions, attached buffers,\netc.) is double-buffered. Protocol requests modify the pending state,\nas opposed to the current state in use by the compositor. A commit\nrequest atomically applies all pending state, replacing the current\nstate. After commit, the new pending state is as documented for each\nrelated request.\n\nOn commit, a pending wl_buffer is applied first, and all other state\nsecond. This means that all coordinates in double-buffered state are\nrelative to the new wl_buffer coming into use, except for\nwl_surface.attach itself. If there is no pending wl_buffer, the\ncoordinates are relative to the current surface contents.\n\nAll requests that need a commit to become effective are documented\nto affect double-buffered state.\n\nOther interfaces may add further double-buffered surface state."]
    fn r#commit(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "`Since version 2`"]
    #[doc = ""]
    #[doc = "Sets The Buffer Transformation"]
    #[doc = ""]
    #[doc = "This request sets an optional transformation on how the compositor\ninterprets the contents of the buffer attached to the surface. The\naccepted values for the transform parameter are the values for\nwl_output.transform.\n\nBuffer transform is double-buffered state, see wl_surface.commit.\n\nA newly created surface has its buffer transformation set to normal.\n\nwl_surface.set_buffer_transform changes the pending buffer\ntransformation. wl_surface.commit copies the pending buffer\ntransformation to the current one. Otherwise, the pending and current\nvalues are never changed.\n\nThe purpose of this request is to allow clients to render content\naccording to the output transform, thus permitting the compositor to\nuse certain optimizations even if the display is rotated. Using\nhardware overlays and scanning out a client buffer for fullscreen\nsurfaces are examples of such optimizations. Those optimizations are\nhighly dependent on the compositor implementation, so the use of this\nrequest should be considered on a case-by-case basis.\n\nNote that if the transform value includes 90 or 270 degree rotation,\nthe width of the buffer will become the surface height and the height\nof the buffer will become the surface width.\n\nIf transform is not one of the values from the\nwl_output.transform enum the invalid_transform protocol error\nis raised."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`transform`: transform for interpreting buffer contents"]
    fn r#set_buffer_transform(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#transform: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "Sets The Buffer Scaling Factor"]
    #[doc = ""]
    #[doc = "This request sets an optional scaling factor on how the compositor\ninterprets the contents of the buffer attached to the window.\n\nBuffer scale is double-buffered state, see wl_surface.commit.\n\nA newly created surface has its buffer scale set to 1.\n\nwl_surface.set_buffer_scale changes the pending buffer scale.\nwl_surface.commit copies the pending buffer scale to the current one.\nOtherwise, the pending and current values are never changed.\n\nThe purpose of this request is to allow clients to supply higher\nresolution buffer data for use on high resolution outputs. It is\nintended that you pick the same buffer scale as the scale of the\noutput that the surface is displayed on. This means the compositor\ncan avoid scaling when rendering the surface on that output.\n\nNote that if the scale is larger than 1, then you have to attach\na buffer that is larger (by a factor of scale in each dimension)\nthan the desired surface size.\n\nIf scale is not positive the invalid_scale protocol error is\nraised."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`scale`: positive scale for interpreting buffer contents"]
    fn r#set_buffer_scale(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#scale: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "`Since version 4`"]
    #[doc = ""]
    #[doc = "Mark Part Of The Surface Damaged Using Buffer Coordinates"]
    #[doc = ""]
    #[doc = "This request is used to describe the regions where the pending\nbuffer is different from the current surface contents, and where\nthe surface therefore needs to be repainted. The compositor\nignores the parts of the damage that fall outside of the surface.\n\nDamage is double-buffered state, see wl_surface.commit.\n\nThe damage rectangle is specified in buffer coordinates,\nwhere x and y specify the upper left corner of the damage rectangle.\n\nThe initial value for pending damage is empty: no damage.\nwl_surface.damage_buffer adds pending damage: the new pending\ndamage is the union of old pending damage and the given rectangle.\n\nwl_surface.commit assigns pending damage as the current damage,\nand clears pending damage. The server will clear the current\ndamage as it repaints the surface.\n\nThis request differs from wl_surface.damage in only one way - it\ntakes damage in buffer coordinates instead of surface-local\ncoordinates. While this generally is more intuitive than surface\ncoordinates, it is especially desirable when using wp_viewport\nor when a drawing library (like EGL) is unaware of buffer scale\nand buffer transform.\n\nNote: Because buffer transformation changes and damage requests may\nbe interleaved in the protocol stream, it is impossible to determine\nthe actual mapping between surface and buffer damage until\nwl_surface.commit time. Therefore, compositors wishing to take both\nkinds of damage into account will have to accumulate damage from the\ntwo requests separately and only transform from one to the other\nafter receiving the wl_surface.commit."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`x`: buffer-local x coordinate"]
    #[doc = "\n`y`: buffer-local y coordinate"]
    #[doc = "\n`width`: width of damage rectangle"]
    #[doc = "\n`height`: height of damage rectangle"]
    fn r#damage_buffer(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#x: ::core::primitive::i32,
        r#y: ::core::primitive::i32,
        r#width: ::core::primitive::i32,
        r#height: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "`Since version 5`"]
    #[doc = ""]
    #[doc = "Set The Surface Contents Offset"]
    #[doc = ""]
    #[doc = "The x and y arguments specify the location of the new pending\nbuffer's upper left corner, relative to the current buffer's upper\nleft corner, in surface-local coordinates. In other words, the\nx and y, combined with the new surface size define in which\ndirections the surface's size changes.\n\nSurface location offset is double-buffered state, see\nwl_surface.commit.\n\nThis request is semantically equivalent to and the replaces the x and y\narguments in the wl_surface.attach request in wl_surface versions prior\nto 5. See wl_surface.attach for details."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`x`: surface-local x coordinate"]
    #[doc = "\n`y`: surface-local y coordinate"]
    fn r#offset(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#x: ::core::primitive::i32,
        r#y: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Surface Enters An Output"]
    #[doc = ""]
    #[doc = "This is emitted whenever a surface's creation, movement, or resizing\nresults in some part of it being within the scanout region of an\noutput.\n\nNote that a surface may be overlapping with zero or more outputs."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`output`: output entered by the surface"]
    fn r#enter(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#output: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 0u16);
        _stream.send_object(Some(r#output))?;
        _stream.commit(_key)
    }
    #[doc = "Surface Leaves An Output"]
    #[doc = ""]
    #[doc = "This is emitted whenever a surface's creation, movement, or resizing\nresults in it no longer having any part of it within the scanout region\nof an output.\n\nClients should not use the number of outputs the surface is on for frame\nthrottling purposes. The surface might be hidden even if no leave event\nhas been sent, and the compositor might expect new surface content\nupdates even if no enter event has been sent. The frame event should be\nused instead."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`output`: output left by the surface"]
    fn r#leave(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#output: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 1u16);
        _stream.send_object(Some(r#output))?;
        _stream.commit(_key)
    }
}
pub mod r#wl_surface {
    #[doc = "Wl Surface Error Values"]
    #[doc = ""]
    #[doc = "These errors can be emitted in response to wl_surface requests."]
    #[repr(transparent)]
    pub struct r#Error(u32);
    impl r#Error {
        #[doc = "Buffer Scale Value Is Invalid"]
        #[doc = ""]
        pub const r#INVALID_SCALE: Self = Self(0u32);
        #[doc = "Buffer Transform Value Is Invalid"]
        #[doc = ""]
        pub const r#INVALID_TRANSFORM: Self = Self(1u32);
        #[doc = "Buffer Size Is Invalid"]
        #[doc = ""]
        pub const r#INVALID_SIZE: Self = Self(2u32);
        #[doc = "Buffer Offset Is Invalid"]
        #[doc = ""]
        pub const r#INVALID_OFFSET: Self = Self(3u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Error {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Error {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "INVALID_SCALE", 0u32),
                1u32 => ::core::write!(f, "{}({})", "INVALID_TRANSFORM", 1u32),
                2u32 => ::core::write!(f, "{}({})", "INVALID_SIZE", 2u32),
                3u32 => ::core::write!(f, "{}({})", "INVALID_OFFSET", 3u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "`Version 8`"]
#[doc = ""]
#[doc = "Group Of Input Devices"]
#[doc = ""]
#[doc = "A seat is a group of keyboards, pointer and touch devices. This\nobject is published as a global during start up, or when such a\ndevice is hot plugged. A seat typically has a pointer and\nmaintains a keyboard focus and a pointer focus."]
pub trait r#WlSeat<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_seat";
    const VERSION: ::core::primitive::u32 = 8u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                let r#id = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                Self::r#get_pointer(_this, _event_loop, _client, r#id)
            }
            1u16 => {
                let _stream = _client.stream();
                let r#id = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                Self::r#get_keyboard(_this, _event_loop, _client, r#id)
            }
            2u16 => {
                let _stream = _client.stream();
                let r#id = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                Self::r#get_touch(_this, _event_loop, _client, r#id)
            }
            3u16 => {
                let _stream = _client.stream();
                Self::r#release(_this, _event_loop, _client)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Return Pointer Object"]
    #[doc = ""]
    #[doc = "The ID provided will be initialized to the wl_pointer interface\nfor this seat.\n\nThis request only takes effect if the seat has the pointer\ncapability, or has had the pointer capability in the past.\nIt is a protocol violation to issue this request on a seat that has\nnever had the pointer capability. The missing_capability error will\nbe sent in this case."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`id`: seat pointer"]
    fn r#get_pointer(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#id: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Return Keyboard Object"]
    #[doc = ""]
    #[doc = "The ID provided will be initialized to the wl_keyboard interface\nfor this seat.\n\nThis request only takes effect if the seat has the keyboard\ncapability, or has had the keyboard capability in the past.\nIt is a protocol violation to issue this request on a seat that has\nnever had the keyboard capability. The missing_capability error will\nbe sent in this case."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`id`: seat keyboard"]
    fn r#get_keyboard(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#id: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Return Touch Object"]
    #[doc = ""]
    #[doc = "The ID provided will be initialized to the wl_touch interface\nfor this seat.\n\nThis request only takes effect if the seat has the touch\ncapability, or has had the touch capability in the past.\nIt is a protocol violation to issue this request on a seat that has\nnever had the touch capability. The missing_capability error will\nbe sent in this case."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`id`: seat touch interface"]
    fn r#get_touch(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#id: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "`Since version 5`"]
    #[doc = ""]
    #[doc = "Release The Seat Object"]
    #[doc = ""]
    #[doc = "Using this request a client can tell the server that it is not going to\nuse the seat object anymore."]
    fn r#release(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Seat Capabilities Changed"]
    #[doc = ""]
    #[doc = "This is emitted whenever a seat gains or loses the pointer,\nkeyboard or touch capabilities. The argument is a capability\nenum containing the complete set of capabilities this seat has.\n\nWhen the pointer capability is added, a client may create a\nwl_pointer object using the wl_seat.get_pointer request. This object\nwill receive pointer events until the capability is removed in the\nfuture.\n\nWhen the pointer capability is removed, a client should destroy the\nwl_pointer objects associated with the seat where the capability was\nremoved, using the wl_pointer.release request. No further pointer\nevents will be received on these objects.\n\nIn some compositors, if a seat regains the pointer capability and a\nclient has a previously obtained wl_pointer object of version 4 or\nless, that object may start sending pointer events again. This\nbehavior is considered a misinterpretation of the intended behavior\nand must not be relied upon by the client. wl_pointer objects of\nversion 5 or later must not send events if created before the most\nrecent event notifying the client of an added pointer capability.\n\nThe above behavior also applies to wl_keyboard and wl_touch with the\nkeyboard and touch capabilities, respectively."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`capabilities`: capabilities of the seat"]
    fn r#capabilities(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#capabilities: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 0u16);
        _stream.send_u32(r#capabilities)?;
        _stream.commit(_key)
    }
    #[doc = "`Since version 2`"]
    #[doc = ""]
    #[doc = "Unique Identifier For This Seat"]
    #[doc = ""]
    #[doc = "In a multi-seat configuration the seat name can be used by clients to\nhelp identify which physical devices the seat represents.\n\nThe seat name is a UTF-8 string with no convention defined for its\ncontents. Each name is unique among all wl_seat globals. The name is\nonly guaranteed to be unique for the current compositor instance.\n\nThe same seat names are used for all clients. Thus, the name can be\nshared across processes to refer to a specific wl_seat global.\n\nThe name event is sent after binding to the seat global. This event is\nonly sent once per seat object, and the name does not change over the\nlifetime of the wl_seat global.\n\nCompositors may re-use the same seat name if the wl_seat global is\ndestroyed and re-created later."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`name`: seat identifier"]
    fn r#name(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#name: &'_ ::core::primitive::str,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 1u16);
        _stream.send_string(::core::option::Option::Some(r#name))?;
        _stream.commit(_key)
    }
}
pub mod r#wl_seat {
    #[doc = "Seat Capability Bitmask"]
    #[doc = ""]
    #[doc = "This is a bitmask of capabilities this seat has; if a member is\nset, then it is present on the seat."]
    #[repr(transparent)]
    pub struct r#Capability(u32);
    impl r#Capability {
        #[doc = "The Seat Has Pointer Devices"]
        #[doc = ""]
        pub const r#POINTER: Self = Self(1u32);
        #[doc = "The Seat Has One Or More Keyboards"]
        #[doc = ""]
        pub const r#KEYBOARD: Self = Self(2u32);
        #[doc = "The Seat Has Touch Devices"]
        #[doc = ""]
        pub const r#TOUCH: Self = Self(4u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Capability {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Capability {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Capability {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                1u32 => ::core::write!(f, "{}({})", "POINTER", 1u32),
                2u32 => ::core::write!(f, "{}({})", "KEYBOARD", 2u32),
                4u32 => ::core::write!(f, "{}({})", "TOUCH", 4u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
    #[doc = "Wl Seat Error Values"]
    #[doc = ""]
    #[doc = "These errors can be emitted in response to wl_seat requests."]
    #[repr(transparent)]
    pub struct r#Error(u32);
    impl r#Error {
        #[doc = "Get Pointer Get Keyboard Or Get Touch Called On Seat Without The Matching Capability"]
        #[doc = ""]
        pub const r#MISSING_CAPABILITY: Self = Self(0u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Error {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Error {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "MISSING_CAPABILITY", 0u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "`Version 8`"]
#[doc = ""]
#[doc = "Pointer Input Device"]
#[doc = ""]
#[doc = "The wl_pointer interface represents one or more input devices,\nsuch as mice, which control the pointer location and pointer_focus\nof a seat.\n\nThe wl_pointer interface generates motion, enter and leave\nevents for the surfaces that the pointer is located over,\nand button and axis events for button presses, button releases\nand scrolling."]
pub trait r#WlPointer<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_pointer";
    const VERSION: ::core::primitive::u32 = 8u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                let r#serial = _stream.u32()?;
                let r#surface = _stream.object()?;
                let r#hotspot_x = _stream.i32()?;
                let r#hotspot_y = _stream.i32()?;
                Self::r#set_cursor(
                    _this,
                    _event_loop,
                    _client,
                    r#serial,
                    r#surface,
                    r#hotspot_x,
                    r#hotspot_y,
                )
            }
            1u16 => {
                let _stream = _client.stream();
                Self::r#release(_this, _event_loop, _client)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Set The Pointer Surface"]
    #[doc = ""]
    #[doc = "Set the pointer surface, i.e., the surface that contains the\npointer image (cursor). This request gives the surface the role\nof a cursor. If the surface already has another role, it raises\na protocol error.\n\nThe cursor actually changes only if the pointer\nfocus for this device is one of the requesting client's surfaces\nor the surface parameter is the current pointer surface. If\nthere was a previous surface set with this request it is\nreplaced. If surface is NULL, the pointer image is hidden.\n\nThe parameters hotspot_x and hotspot_y define the position of\nthe pointer surface relative to the pointer location. Its\ntop-left corner is always at (x, y) - (hotspot_x, hotspot_y),\nwhere (x, y) are the coordinates of the pointer location, in\nsurface-local coordinates.\n\nOn surface.attach requests to the pointer surface, hotspot_x\nand hotspot_y are decremented by the x and y parameters\npassed to the request. Attach must be confirmed by\nwl_surface.commit as usual.\n\nThe hotspot can also be updated by passing the currently set\npointer surface to this request with new values for hotspot_x\nand hotspot_y.\n\nThe current and pending input regions of the wl_surface are\ncleared, and wl_surface.set_input_region is ignored until the\nwl_surface is no longer used as the cursor. When the use as a\ncursor ends, the current and pending input regions become\nundefined, and the wl_surface is unmapped.\n\nThe serial parameter must match the latest wl_pointer.enter\nserial number sent to the client. Otherwise the request will be\nignored."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: serial number of the enter event"]
    #[doc = "\n`surface`: pointer surface"]
    #[doc = "\n`hotspot_x`: surface-local x coordinate"]
    #[doc = "\n`hotspot_y`: surface-local y coordinate"]
    fn r#set_cursor(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#serial: ::core::primitive::u32,
        r#surface: ::core::option::Option<::wl::Id>,
        r#hotspot_x: ::core::primitive::i32,
        r#hotspot_y: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "Release The Pointer Object"]
    #[doc = ""]
    #[doc = "Using this request a client can tell the server that it is not going to\nuse the pointer object anymore.\n\nThis request destroys the pointer proxy object, so clients must not call\nwl_pointer_destroy() after using this request."]
    fn r#release(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Enter Event"]
    #[doc = ""]
    #[doc = "Notification that this seat's pointer is focused on a certain\nsurface.\n\nWhen a seat's focus enters a surface, the pointer image\nis undefined and a client should respond to this event by setting\nan appropriate pointer image with the set_cursor request."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: serial number of the enter event"]
    #[doc = "\n`surface`: surface entered by the pointer"]
    #[doc = "\n`surface_x`: surface-local x coordinate"]
    #[doc = "\n`surface_y`: surface-local y coordinate"]
    fn r#enter(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#serial: ::core::primitive::u32,
        r#surface: ::wl::Id,
        r#surface_x: ::wl::Fixed,
        r#surface_y: ::wl::Fixed,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 0u16);
        _stream.send_u32(r#serial)?;
        _stream.send_object(Some(r#surface))?;
        _stream.send_fixed(r#surface_x)?;
        _stream.send_fixed(r#surface_y)?;
        _stream.commit(_key)
    }
    #[doc = "Leave Event"]
    #[doc = ""]
    #[doc = "Notification that this seat's pointer is no longer focused on\na certain surface.\n\nThe leave notification is sent before the enter notification\nfor the new focus."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: serial number of the leave event"]
    #[doc = "\n`surface`: surface left by the pointer"]
    fn r#leave(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#serial: ::core::primitive::u32,
        r#surface: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 1u16);
        _stream.send_u32(r#serial)?;
        _stream.send_object(Some(r#surface))?;
        _stream.commit(_key)
    }
    #[doc = "Pointer Motion Event"]
    #[doc = ""]
    #[doc = "Notification of pointer location change. The arguments\nsurface_x and surface_y are the location relative to the\nfocused surface."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`time`: timestamp with millisecond granularity"]
    #[doc = "\n`surface_x`: surface-local x coordinate"]
    #[doc = "\n`surface_y`: surface-local y coordinate"]
    fn r#motion(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#time: ::core::primitive::u32,
        r#surface_x: ::wl::Fixed,
        r#surface_y: ::wl::Fixed,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 2u16);
        _stream.send_u32(r#time)?;
        _stream.send_fixed(r#surface_x)?;
        _stream.send_fixed(r#surface_y)?;
        _stream.commit(_key)
    }
    #[doc = "Pointer Button Event"]
    #[doc = ""]
    #[doc = "Mouse button click and release notifications.\n\nThe location of the click is given by the last motion or\nenter event.\nThe time argument is a timestamp with millisecond\ngranularity, with an undefined base.\n\nThe button is a button code as defined in the Linux kernel's\nlinux/input-event-codes.h header file, e.g. BTN_LEFT.\n\nAny 16-bit button code value is reserved for future additions to the\nkernel's event code list. All other button codes above 0xFFFF are\ncurrently undefined but may be used in future versions of this\nprotocol."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: serial number of the button event"]
    #[doc = "\n`time`: timestamp with millisecond granularity"]
    #[doc = "\n`button`: button that produced the event"]
    #[doc = "\n`state`: physical state of the button"]
    fn r#button(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#serial: ::core::primitive::u32,
        r#time: ::core::primitive::u32,
        r#button: ::core::primitive::u32,
        r#state: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 3u16);
        _stream.send_u32(r#serial)?;
        _stream.send_u32(r#time)?;
        _stream.send_u32(r#button)?;
        _stream.send_u32(r#state)?;
        _stream.commit(_key)
    }
    #[doc = "Axis Event"]
    #[doc = ""]
    #[doc = "Scroll and other axis notifications.\n\nFor scroll events (vertical and horizontal scroll axes), the\nvalue parameter is the length of a vector along the specified\naxis in a coordinate space identical to those of motion events,\nrepresenting a relative movement along the specified axis.\n\nFor devices that support movements non-parallel to axes multiple\naxis events will be emitted.\n\nWhen applicable, for example for touch pads, the server can\nchoose to emit scroll events where the motion vector is\nequivalent to a motion event vector.\n\nWhen applicable, a client can transform its content relative to the\nscroll distance."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`time`: timestamp with millisecond granularity"]
    #[doc = "\n`axis`: axis type"]
    #[doc = "\n`value`: length of vector in surface-local coordinate space"]
    fn r#axis(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#time: ::core::primitive::u32,
        r#axis: ::core::primitive::u32,
        r#value: ::wl::Fixed,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 4u16);
        _stream.send_u32(r#time)?;
        _stream.send_u32(r#axis)?;
        _stream.send_fixed(r#value)?;
        _stream.commit(_key)
    }
    #[doc = "`Since version 5`"]
    #[doc = ""]
    #[doc = "End Of A Pointer Event Sequence"]
    #[doc = ""]
    #[doc = "Indicates the end of a set of events that logically belong together.\nA client is expected to accumulate the data in all events within the\nframe before proceeding.\n\nAll wl_pointer events before a wl_pointer.frame event belong\nlogically together. For example, in a diagonal scroll motion the\ncompositor will send an optional wl_pointer.axis_source event, two\nwl_pointer.axis events (horizontal and vertical) and finally a\nwl_pointer.frame event. The client may use this information to\ncalculate a diagonal vector for scrolling.\n\nWhen multiple wl_pointer.axis events occur within the same frame,\nthe motion vector is the combined motion of all events.\nWhen a wl_pointer.axis and a wl_pointer.axis_stop event occur within\nthe same frame, this indicates that axis movement in one axis has\nstopped but continues in the other axis.\nWhen multiple wl_pointer.axis_stop events occur within the same\nframe, this indicates that these axes stopped in the same instance.\n\nA wl_pointer.frame event is sent for every logical event group,\neven if the group only contains a single wl_pointer event.\nSpecifically, a client may get a sequence: motion, frame, button,\nframe, axis, frame, axis_stop, frame.\n\nThe wl_pointer.enter and wl_pointer.leave events are logical events\ngenerated by the compositor and not the hardware. These events are\nalso grouped by a wl_pointer.frame. When a pointer moves from one\nsurface to another, a compositor should group the\nwl_pointer.leave event within the same wl_pointer.frame.\nHowever, a client must not rely on wl_pointer.leave and\nwl_pointer.enter being in the same wl_pointer.frame.\nCompositor-specific policies may require the wl_pointer.leave and\nwl_pointer.enter event being split across multiple wl_pointer.frame\ngroups."]
    fn r#frame(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 5u16);
        _stream.commit(_key)
    }
    #[doc = "`Since version 5`"]
    #[doc = ""]
    #[doc = "Axis Source Event"]
    #[doc = ""]
    #[doc = "Source information for scroll and other axes.\n\nThis event does not occur on its own. It is sent before a\nwl_pointer.frame event and carries the source information for\nall events within that frame.\n\nThe source specifies how this event was generated. If the source is\nwl_pointer.axis_source.finger, a wl_pointer.axis_stop event will be\nsent when the user lifts the finger off the device.\n\nIf the source is wl_pointer.axis_source.wheel,\nwl_pointer.axis_source.wheel_tilt or\nwl_pointer.axis_source.continuous, a wl_pointer.axis_stop event may\nor may not be sent. Whether a compositor sends an axis_stop event\nfor these sources is hardware-specific and implementation-dependent;\nclients must not rely on receiving an axis_stop event for these\nscroll sources and should treat scroll sequences from these scroll\nsources as unterminated by default.\n\nThis event is optional. If the source is unknown for a particular\naxis event sequence, no event is sent.\nOnly one wl_pointer.axis_source event is permitted per frame.\n\nThe order of wl_pointer.axis_discrete and wl_pointer.axis_source is\nnot guaranteed."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`axis_source`: source of the axis event"]
    fn r#axis_source(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#axis_source: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 6u16);
        _stream.send_u32(r#axis_source)?;
        _stream.commit(_key)
    }
    #[doc = "`Since version 5`"]
    #[doc = ""]
    #[doc = "Axis Stop Event"]
    #[doc = ""]
    #[doc = "Stop notification for scroll and other axes.\n\nFor some wl_pointer.axis_source types, a wl_pointer.axis_stop event\nis sent to notify a client that the axis sequence has terminated.\nThis enables the client to implement kinetic scrolling.\nSee the wl_pointer.axis_source documentation for information on when\nthis event may be generated.\n\nAny wl_pointer.axis events with the same axis_source after this\nevent should be considered as the start of a new axis motion.\n\nThe timestamp is to be interpreted identical to the timestamp in the\nwl_pointer.axis event. The timestamp value may be the same as a\npreceding wl_pointer.axis event."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`time`: timestamp with millisecond granularity"]
    #[doc = "\n`axis`: the axis stopped with this event"]
    fn r#axis_stop(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#time: ::core::primitive::u32,
        r#axis: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 7u16);
        _stream.send_u32(r#time)?;
        _stream.send_u32(r#axis)?;
        _stream.commit(_key)
    }
    #[doc = "`Since version 5`"]
    #[doc = ""]
    #[doc = "Axis Click Event"]
    #[doc = ""]
    #[doc = "Discrete step information for scroll and other axes.\n\nThis event carries the axis value of the wl_pointer.axis event in\ndiscrete steps (e.g. mouse wheel clicks).\n\nThis event is deprecated with wl_pointer version 8 - this event is not\nsent to clients supporting version 8 or later.\n\nThis event does not occur on its own, it is coupled with a\nwl_pointer.axis event that represents this axis value on a\ncontinuous scale. The protocol guarantees that each axis_discrete\nevent is always followed by exactly one axis event with the same\naxis number within the same wl_pointer.frame. Note that the protocol\nallows for other events to occur between the axis_discrete and\nits coupled axis event, including other axis_discrete or axis\nevents. A wl_pointer.frame must not contain more than one axis_discrete\nevent per axis type.\n\nThis event is optional; continuous scrolling devices\nlike two-finger scrolling on touchpads do not have discrete\nsteps and do not generate this event.\n\nThe discrete value carries the directional information. e.g. a value\nof -2 is two steps towards the negative direction of this axis.\n\nThe axis number is identical to the axis number in the associated\naxis event.\n\nThe order of wl_pointer.axis_discrete and wl_pointer.axis_source is\nnot guaranteed."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`axis`: axis type"]
    #[doc = "\n`discrete`: number of steps"]
    fn r#axis_discrete(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#axis: ::core::primitive::u32,
        r#discrete: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 8u16);
        _stream.send_u32(r#axis)?;
        _stream.send_i32(r#discrete)?;
        _stream.commit(_key)
    }
    #[doc = "`Since version 8`"]
    #[doc = ""]
    #[doc = "Axis High Resolution Scroll Event"]
    #[doc = ""]
    #[doc = "Discrete high-resolution scroll information.\n\nThis event carries high-resolution wheel scroll information,\nwith each multiple of 120 representing one logical scroll step\n(a wheel detent). For example, an axis_value120 of 30 is one quarter of\na logical scroll step in the positive direction, a value120 of\n-240 are two logical scroll steps in the negative direction within the\nsame hardware event.\nClients that rely on discrete scrolling should accumulate the\nvalue120 to multiples of 120 before processing the event.\n\nThe value120 must not be zero.\n\nThis event replaces the wl_pointer.axis_discrete event in clients\nsupporting wl_pointer version 8 or later.\n\nWhere a wl_pointer.axis_source event occurs in the same\nwl_pointer.frame, the axis source applies to this event.\n\nThe order of wl_pointer.axis_value120 and wl_pointer.axis_source is\nnot guaranteed."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`axis`: axis type"]
    #[doc = "\n`value120`: scroll distance as fraction of 120"]
    fn r#axis_value120(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#axis: ::core::primitive::u32,
        r#value120: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 9u16);
        _stream.send_u32(r#axis)?;
        _stream.send_i32(r#value120)?;
        _stream.commit(_key)
    }
}
pub mod r#wl_pointer {
    #[doc = ""]
    #[repr(transparent)]
    pub struct r#Error(u32);
    impl r#Error {
        #[doc = "Given Wl Surface Has Another Role"]
        #[doc = ""]
        pub const r#ROLE: Self = Self(0u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Error {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Error {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "ROLE", 0u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
    #[doc = "Physical Button State"]
    #[doc = ""]
    #[doc = "Describes the physical state of a button that produced the button\nevent."]
    #[repr(transparent)]
    pub struct r#ButtonState(u32);
    impl r#ButtonState {
        #[doc = "The Button Is Not Pressed"]
        #[doc = ""]
        pub const r#RELEASED: Self = Self(0u32);
        #[doc = "The Button Is Pressed"]
        #[doc = ""]
        pub const r#PRESSED: Self = Self(1u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#ButtonState {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#ButtonState {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#ButtonState {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "RELEASED", 0u32),
                1u32 => ::core::write!(f, "{}({})", "PRESSED", 1u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
    #[doc = "Axis Types"]
    #[doc = ""]
    #[doc = "Describes the axis types of scroll events."]
    #[repr(transparent)]
    pub struct r#Axis(u32);
    impl r#Axis {
        #[doc = "Vertical Axis"]
        #[doc = ""]
        pub const r#VERTICAL_SCROLL: Self = Self(0u32);
        #[doc = "Horizontal Axis"]
        #[doc = ""]
        pub const r#HORIZONTAL_SCROLL: Self = Self(1u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Axis {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Axis {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Axis {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "VERTICAL_SCROLL", 0u32),
                1u32 => ::core::write!(f, "{}({})", "HORIZONTAL_SCROLL", 1u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
    #[doc = "Axis Source Types"]
    #[doc = ""]
    #[doc = "Describes the source types for axis events. This indicates to the\nclient how an axis event was physically generated; a client may\nadjust the user interface accordingly. For example, scroll events\nfrom a \"finger\" source may be in a smooth coordinate space with\nkinetic scrolling whereas a \"wheel\" source may be in discrete steps\nof a number of lines.\n\nThe \"continuous\" axis source is a device generating events in a\ncontinuous coordinate space, but using something other than a\nfinger. One example for this source is button-based scrolling where\nthe vertical motion of a device is converted to scroll events while\na button is held down.\n\nThe \"wheel tilt\" axis source indicates that the actual device is a\nwheel but the scroll event is not caused by a rotation but a\n(usually sideways) tilt of the wheel."]
    #[repr(transparent)]
    pub struct r#AxisSource(u32);
    impl r#AxisSource {
        #[doc = "A Physical Wheel Rotation"]
        #[doc = ""]
        pub const r#WHEEL: Self = Self(0u32);
        #[doc = "Finger On A Touch Surface"]
        #[doc = ""]
        pub const r#FINGER: Self = Self(1u32);
        #[doc = "Continuous Coordinate Space"]
        #[doc = ""]
        pub const r#CONTINUOUS: Self = Self(2u32);
        #[doc = "`Since version 6`"]
        #[doc = ""]
        #[doc = "A Physical Wheel Tilt"]
        #[doc = ""]
        pub const r#WHEEL_TILT: Self = Self(3u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#AxisSource {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#AxisSource {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#AxisSource {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "WHEEL", 0u32),
                1u32 => ::core::write!(f, "{}({})", "FINGER", 1u32),
                2u32 => ::core::write!(f, "{}({})", "CONTINUOUS", 2u32),
                3u32 => ::core::write!(f, "{}({})", "WHEEL_TILT", 3u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "`Version 8`"]
#[doc = ""]
#[doc = "Keyboard Input Device"]
#[doc = ""]
#[doc = "The wl_keyboard interface represents one or more keyboards\nassociated with a seat."]
pub trait r#WlKeyboard<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_keyboard";
    const VERSION: ::core::primitive::u32 = 8u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                Self::r#release(_this, _event_loop, _client)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "Release The Keyboard Object"]
    #[doc = ""]
    fn r#release(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Keyboard Mapping"]
    #[doc = ""]
    #[doc = "This event provides a file descriptor to the client which can be\nmemory-mapped in read-only mode to provide a keyboard mapping\ndescription.\n\nFrom version 7 onwards, the fd must be mapped with MAP_PRIVATE by\nthe recipient, as MAP_SHARED may fail."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`format`: keymap format"]
    #[doc = "\n`fd`: keymap file descriptor"]
    #[doc = "\n`size`: keymap size, in bytes"]
    fn r#keymap(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#format: ::core::primitive::u32,
        r#fd: ::wl::Fd<'static>,
        r#size: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 0u16);
        _stream.send_u32(r#format)?;
        _stream.send_file(r#fd)?;
        _stream.send_u32(r#size)?;
        _stream.commit(_key)
    }
    #[doc = "Enter Event"]
    #[doc = ""]
    #[doc = "Notification that this seat's keyboard focus is on a certain\nsurface.\n\nThe compositor must send the wl_keyboard.modifiers event after this\nevent."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: serial number of the enter event"]
    #[doc = "\n`surface`: surface gaining keyboard focus"]
    #[doc = "\n`keys`: the currently pressed keys"]
    fn r#enter(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#serial: ::core::primitive::u32,
        r#surface: ::wl::Id,
        r#keys: &'_ [::core::primitive::u8],
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 1u16);
        _stream.send_u32(r#serial)?;
        _stream.send_object(Some(r#surface))?;
        _stream.send_bytes(r#keys)?;
        _stream.commit(_key)
    }
    #[doc = "Leave Event"]
    #[doc = ""]
    #[doc = "Notification that this seat's keyboard focus is no longer on\na certain surface.\n\nThe leave notification is sent before the enter notification\nfor the new focus.\n\nAfter this event client must assume that all keys, including modifiers,\nare lifted and also it must stop key repeating if there's some going on."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: serial number of the leave event"]
    #[doc = "\n`surface`: surface that lost keyboard focus"]
    fn r#leave(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#serial: ::core::primitive::u32,
        r#surface: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 2u16);
        _stream.send_u32(r#serial)?;
        _stream.send_object(Some(r#surface))?;
        _stream.commit(_key)
    }
    #[doc = "Key Event"]
    #[doc = ""]
    #[doc = "A key was pressed or released.\nThe time argument is a timestamp with millisecond\ngranularity, with an undefined base.\n\nThe key is a platform-specific key code that can be interpreted\nby feeding it to the keyboard mapping (see the keymap event).\n\nIf this event produces a change in modifiers, then the resulting\nwl_keyboard.modifiers event must be sent after this event."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: serial number of the key event"]
    #[doc = "\n`time`: timestamp with millisecond granularity"]
    #[doc = "\n`key`: key that produced the event"]
    #[doc = "\n`state`: physical state of the key"]
    fn r#key(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#serial: ::core::primitive::u32,
        r#time: ::core::primitive::u32,
        r#key: ::core::primitive::u32,
        r#state: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 3u16);
        _stream.send_u32(r#serial)?;
        _stream.send_u32(r#time)?;
        _stream.send_u32(r#key)?;
        _stream.send_u32(r#state)?;
        _stream.commit(_key)
    }
    #[doc = "Modifier And Group State"]
    #[doc = ""]
    #[doc = "Notifies clients that the modifier and/or group state has\nchanged, and it should update its local state."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: serial number of the modifiers event"]
    #[doc = "\n`mods_depressed`: depressed modifiers"]
    #[doc = "\n`mods_latched`: latched modifiers"]
    #[doc = "\n`mods_locked`: locked modifiers"]
    #[doc = "\n`group`: keyboard layout"]
    fn r#modifiers(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#serial: ::core::primitive::u32,
        r#mods_depressed: ::core::primitive::u32,
        r#mods_latched: ::core::primitive::u32,
        r#mods_locked: ::core::primitive::u32,
        r#group: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 4u16);
        _stream.send_u32(r#serial)?;
        _stream.send_u32(r#mods_depressed)?;
        _stream.send_u32(r#mods_latched)?;
        _stream.send_u32(r#mods_locked)?;
        _stream.send_u32(r#group)?;
        _stream.commit(_key)
    }
    #[doc = "`Since version 4`"]
    #[doc = ""]
    #[doc = "Repeat Rate And Delay"]
    #[doc = ""]
    #[doc = "Informs the client about the keyboard's repeat rate and delay.\n\nThis event is sent as soon as the wl_keyboard object has been created,\nand is guaranteed to be received by the client before any key press\nevent.\n\nNegative values for either rate or delay are illegal. A rate of zero\nwill disable any repeating (regardless of the value of delay).\n\nThis event can be sent later on as well with a new value if necessary,\nso clients should continue listening for the event past the creation\nof wl_keyboard."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`rate`: the rate of repeating keys in characters per second"]
    #[doc = "\n`delay`: delay in milliseconds since key down until repeating starts"]
    fn r#repeat_info(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#rate: ::core::primitive::i32,
        r#delay: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 5u16);
        _stream.send_i32(r#rate)?;
        _stream.send_i32(r#delay)?;
        _stream.commit(_key)
    }
}
pub mod r#wl_keyboard {
    #[doc = "Keyboard Mapping Format"]
    #[doc = ""]
    #[doc = "This specifies the format of the keymap provided to the\nclient with the wl_keyboard.keymap event."]
    #[repr(transparent)]
    pub struct r#KeymapFormat(u32);
    impl r#KeymapFormat {
        #[doc = "No Keymap Client Must Understand How To Interpret The Raw Keycode"]
        #[doc = ""]
        pub const r#NO_KEYMAP: Self = Self(0u32);
        #[doc = "Libxkbcommon Compatible Null Terminated String To Determine The Xkb Keycode Clients Must Add 8 To The Key Event Keycode"]
        #[doc = ""]
        pub const r#XKB_V1: Self = Self(1u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#KeymapFormat {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#KeymapFormat {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#KeymapFormat {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "NO_KEYMAP", 0u32),
                1u32 => ::core::write!(f, "{}({})", "XKB_V1", 1u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
    #[doc = "Physical Key State"]
    #[doc = ""]
    #[doc = "Describes the physical state of a key that produced the key event."]
    #[repr(transparent)]
    pub struct r#KeyState(u32);
    impl r#KeyState {
        #[doc = "Key Is Not Pressed"]
        #[doc = ""]
        pub const r#RELEASED: Self = Self(0u32);
        #[doc = "Key Is Pressed"]
        #[doc = ""]
        pub const r#PRESSED: Self = Self(1u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#KeyState {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#KeyState {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#KeyState {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "RELEASED", 0u32),
                1u32 => ::core::write!(f, "{}({})", "PRESSED", 1u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "`Version 8`"]
#[doc = ""]
#[doc = "Touchscreen Input Device"]
#[doc = ""]
#[doc = "The wl_touch interface represents a touchscreen\nassociated with a seat.\n\nTouch interactions can consist of one or more contacts.\nFor each contact, a series of events is generated, starting\nwith a down event, followed by zero or more motion events,\nand ending with an up event. Events relating to the same\ncontact point can be identified by the ID of the sequence."]
pub trait r#WlTouch<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_touch";
    const VERSION: ::core::primitive::u32 = 8u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                Self::r#release(_this, _event_loop, _client)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "Release The Touch Object"]
    #[doc = ""]
    fn r#release(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Touch Down Event And Beginning Of A Touch Sequence"]
    #[doc = ""]
    #[doc = "A new touch point has appeared on the surface. This touch point is\nassigned a unique ID. Future events from this touch point reference\nthis ID. The ID ceases to be valid after a touch up event and may be\nreused in the future."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: serial number of the touch down event"]
    #[doc = "\n`time`: timestamp with millisecond granularity"]
    #[doc = "\n`surface`: surface touched"]
    #[doc = "\n`id`: the unique ID of this touch point"]
    #[doc = "\n`x`: surface-local x coordinate"]
    #[doc = "\n`y`: surface-local y coordinate"]
    fn r#down(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#serial: ::core::primitive::u32,
        r#time: ::core::primitive::u32,
        r#surface: ::wl::Id,
        r#id: ::core::primitive::i32,
        r#x: ::wl::Fixed,
        r#y: ::wl::Fixed,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 0u16);
        _stream.send_u32(r#serial)?;
        _stream.send_u32(r#time)?;
        _stream.send_object(Some(r#surface))?;
        _stream.send_i32(r#id)?;
        _stream.send_fixed(r#x)?;
        _stream.send_fixed(r#y)?;
        _stream.commit(_key)
    }
    #[doc = "End Of A Touch Event Sequence"]
    #[doc = ""]
    #[doc = "The touch point has disappeared. No further events will be sent for\nthis touch point and the touch point's ID is released and may be\nreused in a future touch down event."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: serial number of the touch up event"]
    #[doc = "\n`time`: timestamp with millisecond granularity"]
    #[doc = "\n`id`: the unique ID of this touch point"]
    fn r#up(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#serial: ::core::primitive::u32,
        r#time: ::core::primitive::u32,
        r#id: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 1u16);
        _stream.send_u32(r#serial)?;
        _stream.send_u32(r#time)?;
        _stream.send_i32(r#id)?;
        _stream.commit(_key)
    }
    #[doc = "Update Of Touch Point Coordinates"]
    #[doc = ""]
    #[doc = "A touch point has changed coordinates."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`time`: timestamp with millisecond granularity"]
    #[doc = "\n`id`: the unique ID of this touch point"]
    #[doc = "\n`x`: surface-local x coordinate"]
    #[doc = "\n`y`: surface-local y coordinate"]
    fn r#motion(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#time: ::core::primitive::u32,
        r#id: ::core::primitive::i32,
        r#x: ::wl::Fixed,
        r#y: ::wl::Fixed,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 2u16);
        _stream.send_u32(r#time)?;
        _stream.send_i32(r#id)?;
        _stream.send_fixed(r#x)?;
        _stream.send_fixed(r#y)?;
        _stream.commit(_key)
    }
    #[doc = "End Of Touch Frame Event"]
    #[doc = ""]
    #[doc = "Indicates the end of a set of events that logically belong together.\nA client is expected to accumulate the data in all events within the\nframe before proceeding.\n\nA wl_touch.frame terminates at least one event but otherwise no\nguarantee is provided about the set of events within a frame. A client\nmust assume that any state not updated in a frame is unchanged from the\npreviously known state."]
    fn r#frame(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 3u16);
        _stream.commit(_key)
    }
    #[doc = "Touch Session Cancelled"]
    #[doc = ""]
    #[doc = "Sent if the compositor decides the touch stream is a global\ngesture. No further events are sent to the clients from that\nparticular gesture. Touch cancellation applies to all touch points\ncurrently active on this client's surface. The client is\nresponsible for finalizing the touch points, future touch points on\nthis surface may reuse the touch point ID."]
    fn r#cancel(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 4u16);
        _stream.commit(_key)
    }
    #[doc = "`Since version 6`"]
    #[doc = ""]
    #[doc = "Update Shape Of Touch Point"]
    #[doc = ""]
    #[doc = "Sent when a touchpoint has changed its shape.\n\nThis event does not occur on its own. It is sent before a\nwl_touch.frame event and carries the new shape information for\nany previously reported, or new touch points of that frame.\n\nOther events describing the touch point such as wl_touch.down,\nwl_touch.motion or wl_touch.orientation may be sent within the\nsame wl_touch.frame. A client should treat these events as a single\nlogical touch point update. The order of wl_touch.shape,\nwl_touch.orientation and wl_touch.motion is not guaranteed.\nA wl_touch.down event is guaranteed to occur before the first\nwl_touch.shape event for this touch ID but both events may occur within\nthe same wl_touch.frame.\n\nA touchpoint shape is approximated by an ellipse through the major and\nminor axis length. The major axis length describes the longer diameter\nof the ellipse, while the minor axis length describes the shorter\ndiameter. Major and minor are orthogonal and both are specified in\nsurface-local coordinates. The center of the ellipse is always at the\ntouchpoint location as reported by wl_touch.down or wl_touch.move.\n\nThis event is only sent by the compositor if the touch device supports\nshape reports. The client has to make reasonable assumptions about the\nshape if it did not receive this event."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`id`: the unique ID of this touch point"]
    #[doc = "\n`major`: length of the major axis in surface-local coordinates"]
    #[doc = "\n`minor`: length of the minor axis in surface-local coordinates"]
    fn r#shape(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#id: ::core::primitive::i32,
        r#major: ::wl::Fixed,
        r#minor: ::wl::Fixed,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 5u16);
        _stream.send_i32(r#id)?;
        _stream.send_fixed(r#major)?;
        _stream.send_fixed(r#minor)?;
        _stream.commit(_key)
    }
    #[doc = "`Since version 6`"]
    #[doc = ""]
    #[doc = "Update Orientation Of Touch Point"]
    #[doc = ""]
    #[doc = "Sent when a touchpoint has changed its orientation.\n\nThis event does not occur on its own. It is sent before a\nwl_touch.frame event and carries the new shape information for\nany previously reported, or new touch points of that frame.\n\nOther events describing the touch point such as wl_touch.down,\nwl_touch.motion or wl_touch.shape may be sent within the\nsame wl_touch.frame. A client should treat these events as a single\nlogical touch point update. The order of wl_touch.shape,\nwl_touch.orientation and wl_touch.motion is not guaranteed.\nA wl_touch.down event is guaranteed to occur before the first\nwl_touch.orientation event for this touch ID but both events may occur\nwithin the same wl_touch.frame.\n\nThe orientation describes the clockwise angle of a touchpoint's major\naxis to the positive surface y-axis and is normalized to the -180 to\n+180 degree range. The granularity of orientation depends on the touch\ndevice, some devices only support binary rotation values between 0 and\n90 degrees.\n\nThis event is only sent by the compositor if the touch device supports\norientation reports."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`id`: the unique ID of this touch point"]
    #[doc = "\n`orientation`: angle between major axis and positive surface y-axis in degrees"]
    fn r#orientation(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#id: ::core::primitive::i32,
        r#orientation: ::wl::Fixed,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 6u16);
        _stream.send_i32(r#id)?;
        _stream.send_fixed(r#orientation)?;
        _stream.commit(_key)
    }
}
pub mod r#wl_touch {}
#[doc = "`Version 4`"]
#[doc = ""]
#[doc = "Compositor Output Region"]
#[doc = ""]
#[doc = "An output describes part of the compositor geometry. The\ncompositor works in the 'compositor coordinate system' and an\noutput corresponds to a rectangular area in that space that is\nactually visible. This typically corresponds to a monitor that\ndisplays part of the compositor space. This object is published\nas global during start up, or when a monitor is hotplugged."]
pub trait r#WlOutput<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_output";
    const VERSION: ::core::primitive::u32 = 4u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                Self::r#release(_this, _event_loop, _client)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "Release The Output Object"]
    #[doc = ""]
    #[doc = "Using this request a client can tell the server that it is not going to\nuse the output object anymore."]
    fn r#release(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Properties Of The Output"]
    #[doc = ""]
    #[doc = "The geometry event describes geometric properties of the output.\nThe event is sent when binding to the output object and whenever\nany of the properties change.\n\nThe physical size can be set to zero if it doesn't make sense for this\noutput (e.g. for projectors or virtual outputs).\n\nThe geometry event will be followed by a done event (starting from\nversion 2).\n\nNote: wl_output only advertises partial information about the output\nposition and identification. Some compositors, for instance those not\nimplementing a desktop-style output layout or those exposing virtual\noutputs, might fake this information. Instead of using x and y, clients\nshould use xdg_output.logical_position. Instead of using make and model,\nclients should use name and description."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`x`: x position within the global compositor space"]
    #[doc = "\n`y`: y position within the global compositor space"]
    #[doc = "\n`physical_width`: width in millimeters of the output"]
    #[doc = "\n`physical_height`: height in millimeters of the output"]
    #[doc = "\n`subpixel`: subpixel orientation of the output"]
    #[doc = "\n`make`: textual description of the manufacturer"]
    #[doc = "\n`model`: textual description of the model"]
    #[doc = "\n`transform`: transform that maps framebuffer to output"]
    fn r#geometry(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#x: ::core::primitive::i32,
        r#y: ::core::primitive::i32,
        r#physical_width: ::core::primitive::i32,
        r#physical_height: ::core::primitive::i32,
        r#subpixel: ::core::primitive::i32,
        r#make: &'_ ::core::primitive::str,
        r#model: &'_ ::core::primitive::str,
        r#transform: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 0u16);
        _stream.send_i32(r#x)?;
        _stream.send_i32(r#y)?;
        _stream.send_i32(r#physical_width)?;
        _stream.send_i32(r#physical_height)?;
        _stream.send_i32(r#subpixel)?;
        _stream.send_string(::core::option::Option::Some(r#make))?;
        _stream.send_string(::core::option::Option::Some(r#model))?;
        _stream.send_i32(r#transform)?;
        _stream.commit(_key)
    }
    #[doc = "Advertise Available Modes For The Output"]
    #[doc = ""]
    #[doc = "The mode event describes an available mode for the output.\n\nThe event is sent when binding to the output object and there\nwill always be one mode, the current mode. The event is sent\nagain if an output changes mode, for the mode that is now\ncurrent. In other words, the current mode is always the last\nmode that was received with the current flag set.\n\nNon-current modes are deprecated. A compositor can decide to only\nadvertise the current mode and never send other modes. Clients\nshould not rely on non-current modes.\n\nThe size of a mode is given in physical hardware units of\nthe output device. This is not necessarily the same as\nthe output size in the global compositor space. For instance,\nthe output may be scaled, as described in wl_output.scale,\nor transformed, as described in wl_output.transform. Clients\nwilling to retrieve the output size in the global compositor\nspace should use xdg_output.logical_size instead.\n\nThe vertical refresh rate can be set to zero if it doesn't make\nsense for this output (e.g. for virtual outputs).\n\nThe mode event will be followed by a done event (starting from\nversion 2).\n\nClients should not use the refresh rate to schedule frames. Instead,\nthey should use the wl_surface.frame event or the presentation-time\nprotocol.\n\nNote: this information is not always meaningful for all outputs. Some\ncompositors, such as those exposing virtual outputs, might fake the\nrefresh rate or the size."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`flags`: bitfield of mode flags"]
    #[doc = "\n`width`: width of the mode in hardware units"]
    #[doc = "\n`height`: height of the mode in hardware units"]
    #[doc = "\n`refresh`: vertical refresh rate in mHz"]
    fn r#mode(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#flags: ::core::primitive::u32,
        r#width: ::core::primitive::i32,
        r#height: ::core::primitive::i32,
        r#refresh: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 1u16);
        _stream.send_u32(r#flags)?;
        _stream.send_i32(r#width)?;
        _stream.send_i32(r#height)?;
        _stream.send_i32(r#refresh)?;
        _stream.commit(_key)
    }
    #[doc = "`Since version 2`"]
    #[doc = ""]
    #[doc = "Sent All Information About Output"]
    #[doc = ""]
    #[doc = "This event is sent after all other properties have been\nsent after binding to the output object and after any\nother property changes done after that. This allows\nchanges to the output properties to be seen as\natomic, even if they happen via multiple events."]
    fn r#done(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 2u16);
        _stream.commit(_key)
    }
    #[doc = "`Since version 2`"]
    #[doc = ""]
    #[doc = "Output Scaling Properties"]
    #[doc = ""]
    #[doc = "This event contains scaling geometry information\nthat is not in the geometry event. It may be sent after\nbinding the output object or if the output scale changes\nlater. If it is not sent, the client should assume a\nscale of 1.\n\nA scale larger than 1 means that the compositor will\nautomatically scale surface buffers by this amount\nwhen rendering. This is used for very high resolution\ndisplays where applications rendering at the native\nresolution would be too small to be legible.\n\nIt is intended that scaling aware clients track the\ncurrent output of a surface, and if it is on a scaled\noutput it should use wl_surface.set_buffer_scale with\nthe scale of the output. That way the compositor can\navoid scaling the surface, and the client can supply\na higher detail image.\n\nThe scale event will be followed by a done event."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`factor`: scaling factor of output"]
    fn r#scale(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#factor: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 3u16);
        _stream.send_i32(r#factor)?;
        _stream.commit(_key)
    }
    #[doc = "`Since version 4`"]
    #[doc = ""]
    #[doc = "Name Of This Output"]
    #[doc = ""]
    #[doc = "Many compositors will assign user-friendly names to their outputs, show\nthem to the user, allow the user to refer to an output, etc. The client\nmay wish to know this name as well to offer the user similar behaviors.\n\nThe name is a UTF-8 string with no convention defined for its contents.\nEach name is unique among all wl_output globals. The name is only\nguaranteed to be unique for the compositor instance.\n\nThe same output name is used for all clients for a given wl_output\nglobal. Thus, the name can be shared across processes to refer to a\nspecific wl_output global.\n\nThe name is not guaranteed to be persistent across sessions, thus cannot\nbe used to reliably identify an output in e.g. configuration files.\n\nExamples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do\nnot assume that the name is a reflection of an underlying DRM connector,\nX11 connection, etc.\n\nThe name event is sent after binding the output object. This event is\nonly sent once per output object, and the name does not change over the\nlifetime of the wl_output global.\n\nCompositors may re-use the same output name if the wl_output global is\ndestroyed and re-created later. Compositors should avoid re-using the\nsame name if possible.\n\nThe name event will be followed by a done event."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`name`: output name"]
    fn r#name(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#name: &'_ ::core::primitive::str,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 4u16);
        _stream.send_string(::core::option::Option::Some(r#name))?;
        _stream.commit(_key)
    }
    #[doc = "`Since version 4`"]
    #[doc = ""]
    #[doc = "Human Readable Description Of This Output"]
    #[doc = ""]
    #[doc = "Many compositors can produce human-readable descriptions of their\noutputs. The client may wish to know this description as well, e.g. for\noutput selection purposes.\n\nThe description is a UTF-8 string with no convention defined for its\ncontents. The description is not guaranteed to be unique among all\nwl_output globals. Examples might include 'Foocorp 11\" Display' or\n'Virtual X11 output via :1'.\n\nThe description event is sent after binding the output object and\nwhenever the description changes. The description is optional, and may\nnot be sent at all.\n\nThe description event will be followed by a done event."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`description`: output description"]
    fn r#description(
        this: ::wl::lease::Lease<Self>,
        client: &mut ::wl::server::Client<T>,
        r#description: &'_ ::core::primitive::str,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _stream = client.stream();
        let _key = _stream.start_message(this.id(), 5u16);
        _stream.send_string(::core::option::Option::Some(r#description))?;
        _stream.commit(_key)
    }
}
pub mod r#wl_output {
    #[doc = "Subpixel Geometry Information"]
    #[doc = ""]
    #[doc = "This enumeration describes how the physical\npixels on an output are laid out."]
    #[repr(transparent)]
    pub struct r#Subpixel(u32);
    impl r#Subpixel {
        #[doc = "Unknown Geometry"]
        #[doc = ""]
        pub const r#UNKNOWN: Self = Self(0u32);
        #[doc = "No Geometry"]
        #[doc = ""]
        pub const r#NONE: Self = Self(1u32);
        #[doc = "Horizontal Rgb"]
        #[doc = ""]
        pub const r#HORIZONTAL_RGB: Self = Self(2u32);
        #[doc = "Horizontal Bgr"]
        #[doc = ""]
        pub const r#HORIZONTAL_BGR: Self = Self(3u32);
        #[doc = "Vertical Rgb"]
        #[doc = ""]
        pub const r#VERTICAL_RGB: Self = Self(4u32);
        #[doc = "Vertical Bgr"]
        #[doc = ""]
        pub const r#VERTICAL_BGR: Self = Self(5u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Subpixel {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Subpixel {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Subpixel {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "UNKNOWN", 0u32),
                1u32 => ::core::write!(f, "{}({})", "NONE", 1u32),
                2u32 => ::core::write!(f, "{}({})", "HORIZONTAL_RGB", 2u32),
                3u32 => ::core::write!(f, "{}({})", "HORIZONTAL_BGR", 3u32),
                4u32 => ::core::write!(f, "{}({})", "VERTICAL_RGB", 4u32),
                5u32 => ::core::write!(f, "{}({})", "VERTICAL_BGR", 5u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
    #[doc = "Transform From Framebuffer To Output"]
    #[doc = ""]
    #[doc = "This describes the transform that a compositor will apply to a\nsurface to compensate for the rotation or mirroring of an\noutput device.\n\nThe flipped values correspond to an initial flip around a\nvertical axis followed by rotation.\n\nThe purpose is mainly to allow clients to render accordingly and\ntell the compositor, so that for fullscreen surfaces, the\ncompositor will still be able to scan out directly from client\nsurfaces."]
    #[repr(transparent)]
    pub struct r#Transform(u32);
    impl r#Transform {
        #[doc = "No Transform"]
        #[doc = ""]
        pub const r#NORMAL: Self = Self(0u32);
        #[doc = "90 Degrees Counter Clockwise"]
        #[doc = ""]
        pub const r#TRANSFORM_90: Self = Self(1u32);
        #[doc = "180 Degrees Counter Clockwise"]
        #[doc = ""]
        pub const r#TRANSFORM_180: Self = Self(2u32);
        #[doc = "270 Degrees Counter Clockwise"]
        #[doc = ""]
        pub const r#TRANSFORM_270: Self = Self(3u32);
        #[doc = "180 Degree Flip Around A Vertical Axis"]
        #[doc = ""]
        pub const r#FLIPPED: Self = Self(4u32);
        #[doc = "Flip And Rotate 90 Degrees Counter Clockwise"]
        #[doc = ""]
        pub const r#FLIPPED_90: Self = Self(5u32);
        #[doc = "Flip And Rotate 180 Degrees Counter Clockwise"]
        #[doc = ""]
        pub const r#FLIPPED_180: Self = Self(6u32);
        #[doc = "Flip And Rotate 270 Degrees Counter Clockwise"]
        #[doc = ""]
        pub const r#FLIPPED_270: Self = Self(7u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Transform {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Transform {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Transform {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "NORMAL", 0u32),
                1u32 => ::core::write!(f, "{}({})", "TRANSFORM_90", 1u32),
                2u32 => ::core::write!(f, "{}({})", "TRANSFORM_180", 2u32),
                3u32 => ::core::write!(f, "{}({})", "TRANSFORM_270", 3u32),
                4u32 => ::core::write!(f, "{}({})", "FLIPPED", 4u32),
                5u32 => ::core::write!(f, "{}({})", "FLIPPED_90", 5u32),
                6u32 => ::core::write!(f, "{}({})", "FLIPPED_180", 6u32),
                7u32 => ::core::write!(f, "{}({})", "FLIPPED_270", 7u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
    #[doc = "Mode Information"]
    #[doc = ""]
    #[doc = "These flags describe properties of an output mode.\nThey are used in the flags bitfield of the mode event."]
    #[repr(transparent)]
    pub struct r#Mode(u32);
    impl r#Mode {
        #[doc = "Indicates This Is The Current Mode"]
        #[doc = ""]
        pub const r#CURRENT: Self = Self(1u32);
        #[doc = "Indicates This Is The Preferred Mode"]
        #[doc = ""]
        pub const r#PREFERRED: Self = Self(2u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Mode {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Mode {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Mode {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                1u32 => ::core::write!(f, "{}({})", "CURRENT", 1u32),
                2u32 => ::core::write!(f, "{}({})", "PREFERRED", 2u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "`Version 1`"]
#[doc = ""]
#[doc = "Region Interface"]
#[doc = ""]
#[doc = "A region object describes an area.\n\nRegion objects are used to describe the opaque and input\nregions of a surface."]
pub trait r#WlRegion<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_region";
    const VERSION: ::core::primitive::u32 = 1u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                Self::r#destroy(_this, _event_loop, _client)
            }
            1u16 => {
                let _stream = _client.stream();
                let r#x = _stream.i32()?;
                let r#y = _stream.i32()?;
                let r#width = _stream.i32()?;
                let r#height = _stream.i32()?;
                Self::r#add(_this, _event_loop, _client, r#x, r#y, r#width, r#height)
            }
            2u16 => {
                let _stream = _client.stream();
                let r#x = _stream.i32()?;
                let r#y = _stream.i32()?;
                let r#width = _stream.i32()?;
                let r#height = _stream.i32()?;
                Self::r#subtract(_this, _event_loop, _client, r#x, r#y, r#width, r#height)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Destroy Region"]
    #[doc = ""]
    #[doc = "Destroy the region. This will invalidate the object ID."]
    fn r#destroy(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Add Rectangle To Region"]
    #[doc = ""]
    #[doc = "Add the specified rectangle to the region."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`x`: region-local x coordinate"]
    #[doc = "\n`y`: region-local y coordinate"]
    #[doc = "\n`width`: rectangle width"]
    #[doc = "\n`height`: rectangle height"]
    fn r#add(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#x: ::core::primitive::i32,
        r#y: ::core::primitive::i32,
        r#width: ::core::primitive::i32,
        r#height: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Subtract Rectangle From Region"]
    #[doc = ""]
    #[doc = "Subtract the specified rectangle from the region."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`x`: region-local x coordinate"]
    #[doc = "\n`y`: region-local y coordinate"]
    #[doc = "\n`width`: rectangle width"]
    #[doc = "\n`height`: rectangle height"]
    fn r#subtract(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#x: ::core::primitive::i32,
        r#y: ::core::primitive::i32,
        r#width: ::core::primitive::i32,
        r#height: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
}
pub mod r#wl_region {}
#[doc = "`Version 1`"]
#[doc = ""]
#[doc = "Sub Surface Compositing"]
#[doc = ""]
#[doc = "The global interface exposing sub-surface compositing capabilities.\nA wl_surface, that has sub-surfaces associated, is called the\nparent surface. Sub-surfaces can be arbitrarily nested and create\na tree of sub-surfaces.\n\nThe root surface in a tree of sub-surfaces is the main\nsurface. The main surface cannot be a sub-surface, because\nsub-surfaces must always have a parent.\n\nA main surface with its sub-surfaces forms a (compound) window.\nFor window management purposes, this set of wl_surface objects is\nto be considered as a single window, and it should also behave as\nsuch.\n\nThe aim of sub-surfaces is to offload some of the compositing work\nwithin a window from clients to the compositor. A prime example is\na video player with decorations and video in separate wl_surface\nobjects. This should allow the compositor to pass YUV video buffer\nprocessing to dedicated overlay hardware when possible."]
pub trait r#WlSubcompositor<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_subcompositor";
    const VERSION: ::core::primitive::u32 = 1u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                Self::r#destroy(_this, _event_loop, _client)
            }
            1u16 => {
                let _stream = _client.stream();
                let r#id = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                let r#surface = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                let r#parent = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                Self::r#get_subsurface(_this, _event_loop, _client, r#id, r#surface, r#parent)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Unbind From The Subcompositor Interface"]
    #[doc = ""]
    #[doc = "Informs the server that the client will not be using this\nprotocol object anymore. This does not affect any other\nobjects, wl_subsurface objects included."]
    fn r#destroy(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Give A Surface The Role Sub Surface"]
    #[doc = ""]
    #[doc = "Create a sub-surface interface for the given surface, and\nassociate it with the given parent surface. This turns a\nplain wl_surface into a sub-surface.\n\nThe to-be sub-surface must not already have another role, and it\nmust not have an existing wl_subsurface object. Otherwise a protocol\nerror is raised.\n\nAdding sub-surfaces to a parent is a double-buffered operation on the\nparent (see wl_surface.commit). The effect of adding a sub-surface\nbecomes visible on the next time the state of the parent surface is\napplied.\n\nThis request modifies the behaviour of wl_surface.commit request on\nthe sub-surface, see the documentation on wl_subsurface interface."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`id`: the new sub-surface object ID"]
    #[doc = "\n`surface`: the surface to be turned into a sub-surface"]
    #[doc = "\n`parent`: the parent surface"]
    fn r#get_subsurface(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#id: ::wl::Id,
        r#surface: ::wl::Id,
        r#parent: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
}
pub mod r#wl_subcompositor {
    #[doc = ""]
    #[repr(transparent)]
    pub struct r#Error(u32);
    impl r#Error {
        #[doc = "The To Be Sub Surface Is Invalid"]
        #[doc = ""]
        pub const r#BAD_SURFACE: Self = Self(0u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Error {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Error {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "BAD_SURFACE", 0u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "`Version 1`"]
#[doc = ""]
#[doc = "Sub Surface Interface To A Wl Surface"]
#[doc = ""]
#[doc = "An additional interface to a wl_surface object, which has been\nmade a sub-surface. A sub-surface has one parent surface. A\nsub-surface's size and position are not limited to that of the parent.\nParticularly, a sub-surface is not automatically clipped to its\nparent's area.\n\nA sub-surface becomes mapped, when a non-NULL wl_buffer is applied\nand the parent surface is mapped. The order of which one happens\nfirst is irrelevant. A sub-surface is hidden if the parent becomes\nhidden, or if a NULL wl_buffer is applied. These rules apply\nrecursively through the tree of surfaces.\n\nThe behaviour of a wl_surface.commit request on a sub-surface\ndepends on the sub-surface's mode. The possible modes are\nsynchronized and desynchronized, see methods\nwl_subsurface.set_sync and wl_subsurface.set_desync. Synchronized\nmode caches the wl_surface state to be applied when the parent's\nstate gets applied, and desynchronized mode applies the pending\nwl_surface state directly. A sub-surface is initially in the\nsynchronized mode.\n\nSub-surfaces also have another kind of state, which is managed by\nwl_subsurface requests, as opposed to wl_surface requests. This\nstate includes the sub-surface position relative to the parent\nsurface (wl_subsurface.set_position), and the stacking order of\nthe parent and its sub-surfaces (wl_subsurface.place_above and\n.place_below). This state is applied when the parent surface's\nwl_surface state is applied, regardless of the sub-surface's mode.\nAs the exception, set_sync and set_desync are effective immediately.\n\nThe main surface can be thought to be always in desynchronized mode,\nsince it does not have a parent in the sub-surfaces sense.\n\nEven if a sub-surface is in desynchronized mode, it will behave as\nin synchronized mode, if its parent surface behaves as in\nsynchronized mode. This rule is applied recursively throughout the\ntree of surfaces. This means, that one can set a sub-surface into\nsynchronized mode, and then assume that all its child and grand-child\nsub-surfaces are synchronized, too, without explicitly setting them.\n\nIf the wl_surface associated with the wl_subsurface is destroyed, the\nwl_subsurface object becomes inert. Note, that destroying either object\ntakes effect immediately. If you need to synchronize the removal\nof a sub-surface to the parent surface update, unmap the sub-surface\nfirst by attaching a NULL wl_buffer, update parent, and then destroy\nthe sub-surface.\n\nIf the parent wl_surface object is destroyed, the sub-surface is\nunmapped."]
pub trait r#WlSubsurface<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "wl_subsurface";
    const VERSION: ::core::primitive::u32 = 1u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::wl::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::wl::wire::EventLoop<T>,
        _client: &mut ::wl::server::Client<T>,
        _message: ::wl::wire::Message,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>> {
        let _this: ::wl::lease::Lease<Self> =
            _this.downcast().ok_or(::wl::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                Self::r#destroy(_this, _event_loop, _client)
            }
            1u16 => {
                let _stream = _client.stream();
                let r#x = _stream.i32()?;
                let r#y = _stream.i32()?;
                Self::r#set_position(_this, _event_loop, _client, r#x, r#y)
            }
            2u16 => {
                let _stream = _client.stream();
                let r#sibling = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                Self::r#place_above(_this, _event_loop, _client, r#sibling)
            }
            3u16 => {
                let _stream = _client.stream();
                let r#sibling = _stream.object()?.ok_or(::wl::wire::WlError::NON_NULLABLE)?;
                Self::r#place_below(_this, _event_loop, _client, r#sibling)
            }
            4u16 => {
                let _stream = _client.stream();
                Self::r#set_sync(_this, _event_loop, _client)
            }
            5u16 => {
                let _stream = _client.stream();
                Self::r#set_desync(_this, _event_loop, _client)
            }
            _ => ::core::result::Result::Err(::wl::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `wl`"]
    fn into_object(self, id: ::wl::Id) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `wl`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::wl::Id,
        version: u32,
    ) -> ::wl::lease::Resident<Self, T, ::wl::server::Client<T>> {
        ::wl::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, version, self)
    }
    #[doc = "Remove Sub Surface Interface"]
    #[doc = ""]
    #[doc = "The sub-surface interface is removed from the wl_surface object\nthat was turned into a sub-surface with a\nwl_subcompositor.get_subsurface request. The wl_surface's association\nto the parent is deleted, and the wl_surface loses its role as\na sub-surface. The wl_surface is unmapped immediately."]
    fn r#destroy(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Reposition The Sub Surface"]
    #[doc = ""]
    #[doc = "This schedules a sub-surface position change.\nThe sub-surface will be moved so that its origin (top left\ncorner pixel) will be at the location x, y of the parent surface\ncoordinate system. The coordinates are not restricted to the parent\nsurface area. Negative values are allowed.\n\nThe scheduled coordinates will take effect whenever the state of the\nparent surface is applied. When this happens depends on whether the\nparent surface is in synchronized mode or not. See\nwl_subsurface.set_sync and wl_subsurface.set_desync for details.\n\nIf more than one set_position request is invoked by the client before\nthe commit of the parent surface, the position of a new request always\nreplaces the scheduled position from any previous request.\n\nThe initial position is 0, 0."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`x`: x coordinate in the parent surface"]
    #[doc = "\n`y`: y coordinate in the parent surface"]
    fn r#set_position(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#x: ::core::primitive::i32,
        r#y: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Restack The Sub Surface"]
    #[doc = ""]
    #[doc = "This sub-surface is taken from the stack, and put back just\nabove the reference surface, changing the z-order of the sub-surfaces.\nThe reference surface must be one of the sibling surfaces, or the\nparent surface. Using any other surface, including this sub-surface,\nwill cause a protocol error.\n\nThe z-order is double-buffered. Requests are handled in order and\napplied immediately to a pending state. The final pending state is\ncopied to the active state the next time the state of the parent\nsurface is applied. When this happens depends on whether the parent\nsurface is in synchronized mode or not. See wl_subsurface.set_sync and\nwl_subsurface.set_desync for details.\n\nA new sub-surface is initially added as the top-most in the stack\nof its siblings and parent."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`sibling`: the reference surface"]
    fn r#place_above(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#sibling: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Restack The Sub Surface"]
    #[doc = ""]
    #[doc = "The sub-surface is placed just below the reference surface.\nSee wl_subsurface.place_above."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`sibling`: the reference surface"]
    fn r#place_below(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
        r#sibling: ::wl::Id,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Set Sub Surface To Synchronized Mode"]
    #[doc = ""]
    #[doc = "Change the commit behaviour of the sub-surface to synchronized\nmode, also described as the parent dependent mode.\n\nIn synchronized mode, wl_surface.commit on a sub-surface will\naccumulate the committed state in a cache, but the state will\nnot be applied and hence will not change the compositor output.\nThe cached state is applied to the sub-surface immediately after\nthe parent surface's state is applied. This ensures atomic\nupdates of the parent and all its synchronized sub-surfaces.\nApplying the cached state will invalidate the cache, so further\nparent surface commits do not (re-)apply old state.\n\nSee wl_subsurface for the recursive effect of this mode."]
    fn r#set_sync(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
    #[doc = "Set Sub Surface To Desynchronized Mode"]
    #[doc = ""]
    #[doc = "Change the commit behaviour of the sub-surface to desynchronized\nmode, also described as independent or freely running mode.\n\nIn desynchronized mode, wl_surface.commit on a sub-surface will\napply the pending state directly, without caching, as happens\nnormally with a wl_surface. Calling wl_surface.commit on the\nparent surface has no effect on the sub-surface's wl_surface\nstate. This mode allows a sub-surface to be updated on its own.\n\nIf cached state exists when wl_surface.commit is called in\ndesynchronized mode, the pending state is added to the cached\nstate, and applied as a whole. This invalidates the cache.\n\nNote: even if a sub-surface is set to desynchronized, a parent\nsub-surface may override it to behave as synchronized. For details,\nsee wl_subsurface.\n\nIf a surface's parent surface behaves as desynchronized, then\nthe cached state is applied on set_desync."]
    fn r#set_desync(
        this: ::wl::lease::Lease<Self>,
        event_loop: &mut ::wl::wire::EventLoop<T>,
        client: &mut ::wl::server::Client<T>,
    ) -> ::core::result::Result<(), ::wl::wire::WlError<'static>>;
}
pub mod r#wl_subsurface {
    #[doc = ""]
    #[repr(transparent)]
    pub struct r#Error(u32);
    impl r#Error {
        #[doc = "Wl Surface Is Not A Sibling Or The Parent"]
        #[doc = ""]
        pub const r#BAD_SURFACE: Self = Self(0u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Error {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Error {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "BAD_SURFACE", 0u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}

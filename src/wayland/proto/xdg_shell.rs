// Auto-Generated file. Do not edit.
#![allow(dead_code)]
#![doc = "# xdg_shell"]
#![doc = ""]
#![doc = "## Copyright"]
#![doc = "Copyright © 2008-2013 Kristian Høgsberg\nCopyright © 2013   Rafael Antognolli\nCopyright © 2013   Jasper St. Pierre\nCopyright © 2010-2013 Intel Corporation\nCopyright © 2015-2017 Samsung Electronics Co., Ltd\nCopyright © 2015-2017 Red Hat Inc.\n\nPermission is hereby granted, free of charge, to any person obtaining a\ncopy of this software and associated documentation files (the \"Software\"),\nto deal in the Software without restriction, including without limitation\nthe rights to use, copy, modify, merge, publish, distribute, sublicense,\nand/or sell copies of the Software, and to permit persons to whom the\nSoftware is furnished to do so, subject to the following conditions:\n\nThe above copyright notice and this permission notice (including the next\nparagraph) shall be included in all copies or substantial portions of the\nSoftware.\n\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\nIMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\nFITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL\nTHE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\nLIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING\nFROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER\nDEALINGS IN THE SOFTWARE."]
#[doc = "create desktop-style surfaces"]
#[doc = ""]
#[doc = "`Version 5`"]
#[doc = ""]
#[doc = "The xdg_wm_base interface is exposed as a global object enabling clients\nto turn their wl_surfaces into windows in a desktop environment. It\ndefines the basic functionality needed for clients and the compositor to\ncreate windows that can be dragged, resized, maximized, etc, as well as\ncreating transient windows such as popup menus."]
pub trait r#XdgWmBase<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "xdg_wm_base";
    const VERSION: ::core::primitive::u32 = 5u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::yutani::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::yutani::wire::EventLoop<T>,
        _client: &mut ::yutani::server::Client<T>,
        _message: ::yutani::wire::Message,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        let _this: ::yutani::lease::Lease<Self> =
            _this.downcast().ok_or(::yutani::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_wm_base", "@{}.", "destroy", "(", ")"),
                        _this.id(),
                    );
                }
                Self::r#destroy(_this, _event_loop, _client)
            }
            1u16 => {
                let _stream = _client.stream();
                let r#id = _stream
                    .object()?
                    .ok_or(::yutani::wire::WlError::NON_NULLABLE)?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "xdg_wm_base",
                            "@{}.",
                            "create_positioner",
                            "(",
                            "{:?}",
                            ")"
                        ),
                        _this.id(),
                        r#id,
                    );
                }
                Self::r#create_positioner(_this, _event_loop, _client, r#id)
            }
            2u16 => {
                let _stream = _client.stream();
                let r#id = _stream
                    .object()?
                    .ok_or(::yutani::wire::WlError::NON_NULLABLE)?;
                let r#surface = _stream
                    .object()?
                    .ok_or(::yutani::wire::WlError::NON_NULLABLE)?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "xdg_wm_base",
                            "@{}.",
                            "get_xdg_surface",
                            "(",
                            "{:?}",
                            ", {:?}",
                            ")"
                        ),
                        _this.id(),
                        r#id,
                        r#surface,
                    );
                }
                Self::r#get_xdg_surface(_this, _event_loop, _client, r#id, r#surface)
            }
            3u16 => {
                let _stream = _client.stream();
                let r#serial = _stream.u32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_wm_base", "@{}.", "pong", "(", "{:?}", ")"),
                        _this.id(),
                        r#serial,
                    );
                }
                Self::r#pong(_this, _event_loop, _client, r#serial)
            }
            _ => ::core::result::Result::Err(::yutani::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `yutani`"]
    fn into_object(
        self,
        id: ::yutani::Id,
    ) -> ::yutani::lease::Resident<Self, T, ::yutani::server::Client<T>> {
        ::yutani::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `yutani`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::yutani::Id,
        version: u32,
    ) -> ::core::result::Result<
        ::yutani::lease::Resident<Self, T, ::yutani::server::Client<T>>,
        ::yutani::wire::WlError<'static>,
    > {
        if version > Self::VERSION {
            ::core::result::Result::Err(::yutani::wire::WlError::UNSUPPORTED_VERSION)
        } else {
            ::core::result::Result::Ok(::yutani::lease::Resident::new(
                id,
                Self::dispatch,
                Self::INTERFACE,
                version,
                self,
            ))
        }
    }
    #[doc = "destroy xdg_wm_base"]
    #[doc = ""]
    #[doc = "Destroy this xdg_wm_base object.\n\nDestroying a bound xdg_wm_base object while there are surfaces\nstill alive created by this xdg_wm_base object instance is illegal\nand will result in a protocol error."]
    fn r#destroy(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "create a positioner object"]
    #[doc = ""]
    #[doc = "Create a positioner object. A positioner object is used to position\nsurfaces relative to some parent surface. See the interface description\nand xdg_surface.get_popup for details."]
    fn r#create_positioner(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#id: ::yutani::Id,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "create a shell surface from a surface"]
    #[doc = ""]
    #[doc = "This creates an xdg_surface for the given surface. While xdg_surface\nitself is not a role, the corresponding surface may only be assigned\na role extending xdg_surface, such as xdg_toplevel or xdg_popup. It is\nillegal to create an xdg_surface for a wl_surface which already has an\nassigned role and this will result in a protocol error.\n\nThis creates an xdg_surface for the given surface. An xdg_surface is\nused as basis to define a role to a given surface, such as xdg_toplevel\nor xdg_popup. It also manages functionality shared between xdg_surface\nbased surface roles.\n\nSee the documentation of xdg_surface for more details about what an\nxdg_surface is and how it is used."]
    fn r#get_xdg_surface(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#id: ::yutani::Id,
        r#surface: ::yutani::Id,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "respond to a ping event"]
    #[doc = ""]
    #[doc = "A client must respond to a ping event with a pong request or\nthe client may be deemed unresponsive. See xdg_wm_base.ping."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: serial of the ping event"]
    fn r#pong(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#serial: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "check if the client is alive"]
    #[doc = ""]
    #[doc = "The ping event asks the client if it's still alive. Pass the\nserial specified in the event back to the compositor by sending\na \"pong\" request back with the specified serial. See xdg_wm_base.pong.\n\nCompositors can use this to determine if the client is still\nalive. It's unspecified what will happen if the client doesn't\nrespond to the ping request, or in what timeframe. Clients should\ntry to respond in a reasonable amount of time.\n\nA compositor is free to ping in any way it wants, but a client must\nalways respond to any xdg_wm_base object it created."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: pass this to the pong request"]
    fn r#ping(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
        r#serial: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(" -> ", "xdg_wm_base", "@{}.", "ping", "(", "{:?}", ")"),
                _this.id(),
                r#serial,
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 0u16);
        _stream.send_u32(r#serial)?;
        _stream.commit(_key)
    }
}
pub mod r#xdg_wm_base {
    #[doc = ""]
    #[repr(transparent)]
    pub struct r#Error(u32);
    impl r#Error {
        #[doc = "given wl_surface has another role"]
        #[doc = ""]
        pub const r#ROLE: Self = Self(0u32);
        #[doc = "xdg_wm_base was destroyed before children"]
        #[doc = ""]
        pub const r#DEFUNCT_SURFACES: Self = Self(1u32);
        #[doc = "the client tried to map or destroy a non-topmost popup"]
        #[doc = ""]
        pub const r#NOT_THE_TOPMOST_POPUP: Self = Self(2u32);
        #[doc = "the client specified an invalid popup parent surface"]
        #[doc = ""]
        pub const r#INVALID_POPUP_PARENT: Self = Self(3u32);
        #[doc = "the client provided an invalid surface state"]
        #[doc = ""]
        pub const r#INVALID_SURFACE_STATE: Self = Self(4u32);
        #[doc = "the client provided an invalid positioner"]
        #[doc = ""]
        pub const r#INVALID_POSITIONER: Self = Self(5u32);
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
                1u32 => ::core::write!(f, "{}({})", "DEFUNCT_SURFACES", 1u32),
                2u32 => ::core::write!(f, "{}({})", "NOT_THE_TOPMOST_POPUP", 2u32),
                3u32 => ::core::write!(f, "{}({})", "INVALID_POPUP_PARENT", 3u32),
                4u32 => ::core::write!(f, "{}({})", "INVALID_SURFACE_STATE", 4u32),
                5u32 => ::core::write!(f, "{}({})", "INVALID_POSITIONER", 5u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "child surface positioner"]
#[doc = ""]
#[doc = "`Version 5`"]
#[doc = ""]
#[doc = "The xdg_positioner provides a collection of rules for the placement of a\nchild surface relative to a parent surface. Rules can be defined to ensure\nthe child surface remains within the visible area's borders, and to\nspecify how the child surface changes its position, such as sliding along\nan axis, or flipping around a rectangle. These positioner-created rules are\nconstrained by the requirement that a child surface must intersect with or\nbe at least partially adjacent to its parent surface.\n\nSee the various requests for details about possible rules.\n\nAt the time of the request, the compositor makes a copy of the rules\nspecified by the xdg_positioner. Thus, after the request is complete the\nxdg_positioner object can be destroyed or reused; further changes to the\nobject will have no effect on previous usages.\n\nFor an xdg_positioner object to be considered complete, it must have a\nnon-zero size set by set_size, and a non-zero anchor rectangle set by\nset_anchor_rect. Passing an incomplete xdg_positioner object when\npositioning a surface raises an error."]
pub trait r#XdgPositioner<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "xdg_positioner";
    const VERSION: ::core::primitive::u32 = 5u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::yutani::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::yutani::wire::EventLoop<T>,
        _client: &mut ::yutani::server::Client<T>,
        _message: ::yutani::wire::Message,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        let _this: ::yutani::lease::Lease<Self> =
            _this.downcast().ok_or(::yutani::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_positioner", "@{}.", "destroy", "(", ")"),
                        _this.id(),
                    );
                }
                Self::r#destroy(_this, _event_loop, _client)
            }
            1u16 => {
                let _stream = _client.stream();
                let r#width = _stream.i32()?;
                let r#height = _stream.i32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "xdg_positioner",
                            "@{}.",
                            "set_size",
                            "(",
                            "{:?}",
                            ", {:?}",
                            ")"
                        ),
                        _this.id(),
                        r#width,
                        r#height,
                    );
                }
                Self::r#set_size(_this, _event_loop, _client, r#width, r#height)
            }
            2u16 => {
                let _stream = _client.stream();
                let r#x = _stream.i32()?;
                let r#y = _stream.i32()?;
                let r#width = _stream.i32()?;
                let r#height = _stream.i32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "xdg_positioner",
                            "@{}.",
                            "set_anchor_rect",
                            "(",
                            "{:?}",
                            ", {:?}",
                            ", {:?}",
                            ", {:?}",
                            ")"
                        ),
                        _this.id(),
                        r#x,
                        r#y,
                        r#width,
                        r#height,
                    );
                }
                Self::r#set_anchor_rect(_this, _event_loop, _client, r#x, r#y, r#width, r#height)
            }
            3u16 => {
                let _stream = _client.stream();
                let r#anchor = _stream.u32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_positioner", "@{}.", "set_anchor", "(", "{:?}", ")"),
                        _this.id(),
                        r#anchor,
                    );
                }
                Self::r#set_anchor(_this, _event_loop, _client, r#anchor)
            }
            4u16 => {
                let _stream = _client.stream();
                let r#gravity = _stream.u32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_positioner", "@{}.", "set_gravity", "(", "{:?}", ")"),
                        _this.id(),
                        r#gravity,
                    );
                }
                Self::r#set_gravity(_this, _event_loop, _client, r#gravity)
            }
            5u16 => {
                let _stream = _client.stream();
                let r#constraint_adjustment = _stream.u32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "xdg_positioner",
                            "@{}.",
                            "set_constraint_adjustment",
                            "(",
                            "{:?}",
                            ")"
                        ),
                        _this.id(),
                        r#constraint_adjustment,
                    );
                }
                Self::r#set_constraint_adjustment(
                    _this,
                    _event_loop,
                    _client,
                    r#constraint_adjustment,
                )
            }
            6u16 => {
                let _stream = _client.stream();
                let r#x = _stream.i32()?;
                let r#y = _stream.i32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "xdg_positioner",
                            "@{}.",
                            "set_offset",
                            "(",
                            "{:?}",
                            ", {:?}",
                            ")"
                        ),
                        _this.id(),
                        r#x,
                        r#y,
                    );
                }
                Self::r#set_offset(_this, _event_loop, _client, r#x, r#y)
            }
            7u16 => {
                let _stream = _client.stream();
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_positioner", "@{}.", "set_reactive", "(", ")"),
                        _this.id(),
                    );
                }
                Self::r#set_reactive(_this, _event_loop, _client)
            }
            8u16 => {
                let _stream = _client.stream();
                let r#parent_width = _stream.i32()?;
                let r#parent_height = _stream.i32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "xdg_positioner",
                            "@{}.",
                            "set_parent_size",
                            "(",
                            "{:?}",
                            ", {:?}",
                            ")"
                        ),
                        _this.id(),
                        r#parent_width,
                        r#parent_height,
                    );
                }
                Self::r#set_parent_size(
                    _this,
                    _event_loop,
                    _client,
                    r#parent_width,
                    r#parent_height,
                )
            }
            9u16 => {
                let _stream = _client.stream();
                let r#serial = _stream.u32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "xdg_positioner",
                            "@{}.",
                            "set_parent_configure",
                            "(",
                            "{:?}",
                            ")"
                        ),
                        _this.id(),
                        r#serial,
                    );
                }
                Self::r#set_parent_configure(_this, _event_loop, _client, r#serial)
            }
            _ => ::core::result::Result::Err(::yutani::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `yutani`"]
    fn into_object(
        self,
        id: ::yutani::Id,
    ) -> ::yutani::lease::Resident<Self, T, ::yutani::server::Client<T>> {
        ::yutani::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `yutani`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::yutani::Id,
        version: u32,
    ) -> ::core::result::Result<
        ::yutani::lease::Resident<Self, T, ::yutani::server::Client<T>>,
        ::yutani::wire::WlError<'static>,
    > {
        if version > Self::VERSION {
            ::core::result::Result::Err(::yutani::wire::WlError::UNSUPPORTED_VERSION)
        } else {
            ::core::result::Result::Ok(::yutani::lease::Resident::new(
                id,
                Self::dispatch,
                Self::INTERFACE,
                version,
                self,
            ))
        }
    }
    #[doc = "destroy the xdg_positioner object"]
    #[doc = ""]
    #[doc = "Notify the compositor that the xdg_positioner will no longer be used."]
    fn r#destroy(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "set the size of the to-be positioned rectangle"]
    #[doc = ""]
    #[doc = "Set the size of the surface that is to be positioned with the positioner\nobject. The size is in surface-local coordinates and corresponds to the\nwindow geometry. See xdg_surface.set_window_geometry.\n\nIf a zero or negative size is set the invalid_input error is raised."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`width`: width of positioned rectangle"]
    #[doc = "\n`height`: height of positioned rectangle"]
    fn r#set_size(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#width: ::core::primitive::i32,
        r#height: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "set the anchor rectangle within the parent surface"]
    #[doc = ""]
    #[doc = "Specify the anchor rectangle within the parent surface that the child\nsurface will be placed relative to. The rectangle is relative to the\nwindow geometry as defined by xdg_surface.set_window_geometry of the\nparent surface.\n\nWhen the xdg_positioner object is used to position a child surface, the\nanchor rectangle may not extend outside the window geometry of the\npositioned child's parent surface.\n\nIf a negative size is set the invalid_input error is raised."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`x`: x position of anchor rectangle"]
    #[doc = "\n`y`: y position of anchor rectangle"]
    #[doc = "\n`width`: width of anchor rectangle"]
    #[doc = "\n`height`: height of anchor rectangle"]
    fn r#set_anchor_rect(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#x: ::core::primitive::i32,
        r#y: ::core::primitive::i32,
        r#width: ::core::primitive::i32,
        r#height: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "set anchor rectangle anchor"]
    #[doc = ""]
    #[doc = "Defines the anchor point for the anchor rectangle. The specified anchor\nis used derive an anchor point that the child surface will be\npositioned relative to. If a corner anchor is set (e.g. 'top_left' or\n'bottom_right'), the anchor point will be at the specified corner;\notherwise, the derived anchor point will be centered on the specified\nedge, or in the center of the anchor rectangle if no edge is specified."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`anchor`: anchor"]
    fn r#set_anchor(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#anchor: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "set child surface gravity"]
    #[doc = ""]
    #[doc = "Defines in what direction a surface should be positioned, relative to\nthe anchor point of the parent surface. If a corner gravity is\nspecified (e.g. 'bottom_right' or 'top_left'), then the child surface\nwill be placed towards the specified gravity; otherwise, the child\nsurface will be centered over the anchor point on any axis that had no\ngravity specified."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`gravity`: gravity direction"]
    fn r#set_gravity(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#gravity: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "set the adjustment to be done when constrained"]
    #[doc = ""]
    #[doc = "Specify how the window should be positioned if the originally intended\nposition caused the surface to be constrained, meaning at least\npartially outside positioning boundaries set by the compositor. The\nadjustment is set by constructing a bitmask describing the adjustment to\nbe made when the surface is constrained on that axis.\n\nIf no bit for one axis is set, the compositor will assume that the child\nsurface should not change its position on that axis when constrained.\n\nIf more than one bit for one axis is set, the order of how adjustments\nare applied is specified in the corresponding adjustment descriptions.\n\nThe default adjustment is none."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`constraint_adjustment`: bit mask of constraint adjustments"]
    fn r#set_constraint_adjustment(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#constraint_adjustment: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "set surface position offset"]
    #[doc = ""]
    #[doc = "Specify the surface position offset relative to the position of the\nanchor on the anchor rectangle and the anchor on the surface. For\nexample if the anchor of the anchor rectangle is at (x, y), the surface\nhas the gravity bottom|right, and the offset is (ox, oy), the calculated\nsurface position will be (x + ox, y + oy). The offset position of the\nsurface is the one used for constraint testing. See\nset_constraint_adjustment.\n\nAn example use case is placing a popup menu on top of a user interface\nelement, while aligning the user interface element of the parent surface\nwith some user interface element placed somewhere in the popup surface."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`x`: surface position x offset"]
    #[doc = "\n`y`: surface position y offset"]
    fn r#set_offset(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#x: ::core::primitive::i32,
        r#y: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "continuously reconstrain the surface"]
    #[doc = ""]
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "When set reactive, the surface is reconstrained if the conditions used\nfor constraining changed, e.g. the parent window moved.\n\nIf the conditions changed and the popup was reconstrained, an\nxdg_popup.configure event is sent with updated geometry, followed by an\nxdg_surface.configure event."]
    fn r#set_reactive(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = ""]
    #[doc = ""]
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "Set the parent window geometry the compositor should use when\npositioning the popup. The compositor may use this information to\ndetermine the future state the popup should be constrained using. If\nthis doesn't match the dimension of the parent the popup is eventually\npositioned against, the behavior is undefined.\n\nThe arguments are given in the surface-local coordinate space."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`parent_width`: future window geometry width of parent"]
    #[doc = "\n`parent_height`: future window geometry height of parent"]
    fn r#set_parent_size(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#parent_width: ::core::primitive::i32,
        r#parent_height: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "set parent configure this is a response to"]
    #[doc = ""]
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "Set the serial of an xdg_surface.configure event this positioner will be\nused in response to. The compositor may use this information together\nwith set_parent_size to determine what future state the popup should be\nconstrained using."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: serial of parent configure event"]
    fn r#set_parent_configure(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#serial: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
}
pub mod r#xdg_positioner {
    #[doc = ""]
    #[repr(transparent)]
    pub struct r#Error(u32);
    impl r#Error {
        #[doc = "invalid input provided"]
        #[doc = ""]
        pub const r#INVALID_INPUT: Self = Self(0u32);
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
                0u32 => ::core::write!(f, "{}({})", "INVALID_INPUT", 0u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
    #[doc = ""]
    #[repr(transparent)]
    pub struct r#Anchor(u32);
    impl r#Anchor {
        #[doc = ""]
        pub const r#NONE: Self = Self(0u32);
        #[doc = ""]
        pub const r#TOP: Self = Self(1u32);
        #[doc = ""]
        pub const r#BOTTOM: Self = Self(2u32);
        #[doc = ""]
        pub const r#LEFT: Self = Self(3u32);
        #[doc = ""]
        pub const r#RIGHT: Self = Self(4u32);
        #[doc = ""]
        pub const r#TOP_LEFT: Self = Self(5u32);
        #[doc = ""]
        pub const r#BOTTOM_LEFT: Self = Self(6u32);
        #[doc = ""]
        pub const r#TOP_RIGHT: Self = Self(7u32);
        #[doc = ""]
        pub const r#BOTTOM_RIGHT: Self = Self(8u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Anchor {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Anchor {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Anchor {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "NONE", 0u32),
                1u32 => ::core::write!(f, "{}({})", "TOP", 1u32),
                2u32 => ::core::write!(f, "{}({})", "BOTTOM", 2u32),
                3u32 => ::core::write!(f, "{}({})", "LEFT", 3u32),
                4u32 => ::core::write!(f, "{}({})", "RIGHT", 4u32),
                5u32 => ::core::write!(f, "{}({})", "TOP_LEFT", 5u32),
                6u32 => ::core::write!(f, "{}({})", "BOTTOM_LEFT", 6u32),
                7u32 => ::core::write!(f, "{}({})", "TOP_RIGHT", 7u32),
                8u32 => ::core::write!(f, "{}({})", "BOTTOM_RIGHT", 8u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
    #[doc = ""]
    #[repr(transparent)]
    pub struct r#Gravity(u32);
    impl r#Gravity {
        #[doc = ""]
        pub const r#NONE: Self = Self(0u32);
        #[doc = ""]
        pub const r#TOP: Self = Self(1u32);
        #[doc = ""]
        pub const r#BOTTOM: Self = Self(2u32);
        #[doc = ""]
        pub const r#LEFT: Self = Self(3u32);
        #[doc = ""]
        pub const r#RIGHT: Self = Self(4u32);
        #[doc = ""]
        pub const r#TOP_LEFT: Self = Self(5u32);
        #[doc = ""]
        pub const r#BOTTOM_LEFT: Self = Self(6u32);
        #[doc = ""]
        pub const r#TOP_RIGHT: Self = Self(7u32);
        #[doc = ""]
        pub const r#BOTTOM_RIGHT: Self = Self(8u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Gravity {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Gravity {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Gravity {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "NONE", 0u32),
                1u32 => ::core::write!(f, "{}({})", "TOP", 1u32),
                2u32 => ::core::write!(f, "{}({})", "BOTTOM", 2u32),
                3u32 => ::core::write!(f, "{}({})", "LEFT", 3u32),
                4u32 => ::core::write!(f, "{}({})", "RIGHT", 4u32),
                5u32 => ::core::write!(f, "{}({})", "TOP_LEFT", 5u32),
                6u32 => ::core::write!(f, "{}({})", "BOTTOM_LEFT", 6u32),
                7u32 => ::core::write!(f, "{}({})", "TOP_RIGHT", 7u32),
                8u32 => ::core::write!(f, "{}({})", "BOTTOM_RIGHT", 8u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
    #[doc = "constraint adjustments"]
    #[doc = ""]
    #[doc = "The constraint adjustment value define ways the compositor will adjust\nthe position of the surface, if the unadjusted position would result\nin the surface being partly constrained.\n\nWhether a surface is considered 'constrained' is left to the compositor\nto determine. For example, the surface may be partly outside the\ncompositor's defined 'work area', thus necessitating the child surface's\nposition be adjusted until it is entirely inside the work area.\n\nThe adjustments can be combined, according to a defined precedence: 1)\nFlip, 2) Slide, 3) Resize."]
    #[repr(transparent)]
    pub struct r#ConstraintAdjustment(u32);
    impl r#ConstraintAdjustment {
        #[doc = "don't move the child surface when constrained"]
        #[doc = ""]
        #[doc = "Don't alter the surface position even if it is constrained on some\naxis, for example partially outside the edge of an output."]
        pub const r#NONE: Self = Self(0u32);
        #[doc = "move along the x axis until unconstrained"]
        #[doc = ""]
        #[doc = "Slide the surface along the x axis until it is no longer constrained.\n\nFirst try to slide towards the direction of the gravity on the x axis\nuntil either the edge in the opposite direction of the gravity is\nunconstrained or the edge in the direction of the gravity is\nconstrained.\n\nThen try to slide towards the opposite direction of the gravity on the\nx axis until either the edge in the direction of the gravity is\nunconstrained or the edge in the opposite direction of the gravity is\nconstrained."]
        pub const r#SLIDE_X: Self = Self(1u32);
        #[doc = "move along the y axis until unconstrained"]
        #[doc = ""]
        #[doc = "Slide the surface along the y axis until it is no longer constrained.\n\nFirst try to slide towards the direction of the gravity on the y axis\nuntil either the edge in the opposite direction of the gravity is\nunconstrained or the edge in the direction of the gravity is\nconstrained.\n\nThen try to slide towards the opposite direction of the gravity on the\ny axis until either the edge in the direction of the gravity is\nunconstrained or the edge in the opposite direction of the gravity is\nconstrained."]
        pub const r#SLIDE_Y: Self = Self(2u32);
        #[doc = "invert the anchor and gravity on the x axis"]
        #[doc = ""]
        #[doc = "Invert the anchor and gravity on the x axis if the surface is\nconstrained on the x axis. For example, if the left edge of the\nsurface is constrained, the gravity is 'left' and the anchor is\n'left', change the gravity to 'right' and the anchor to 'right'.\n\nIf the adjusted position also ends up being constrained, the resulting\nposition of the flip_x adjustment will be the one before the\nadjustment."]
        pub const r#FLIP_X: Self = Self(4u32);
        #[doc = "invert the anchor and gravity on the y axis"]
        #[doc = ""]
        #[doc = "Invert the anchor and gravity on the y axis if the surface is\nconstrained on the y axis. For example, if the bottom edge of the\nsurface is constrained, the gravity is 'bottom' and the anchor is\n'bottom', change the gravity to 'top' and the anchor to 'top'.\n\nThe adjusted position is calculated given the original anchor\nrectangle and offset, but with the new flipped anchor and gravity\nvalues.\n\nIf the adjusted position also ends up being constrained, the resulting\nposition of the flip_y adjustment will be the one before the\nadjustment."]
        pub const r#FLIP_Y: Self = Self(8u32);
        #[doc = "horizontally resize the surface"]
        #[doc = ""]
        #[doc = "Resize the surface horizontally so that it is completely\nunconstrained."]
        pub const r#RESIZE_X: Self = Self(16u32);
        #[doc = "vertically resize the surface"]
        #[doc = ""]
        #[doc = "Resize the surface vertically so that it is completely unconstrained."]
        pub const r#RESIZE_Y: Self = Self(32u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#ConstraintAdjustment {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#ConstraintAdjustment {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#ConstraintAdjustment {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                0u32 => ::core::write!(f, "{}({})", "NONE", 0u32),
                1u32 => ::core::write!(f, "{}({})", "SLIDE_X", 1u32),
                2u32 => ::core::write!(f, "{}({})", "SLIDE_Y", 2u32),
                4u32 => ::core::write!(f, "{}({})", "FLIP_X", 4u32),
                8u32 => ::core::write!(f, "{}({})", "FLIP_Y", 8u32),
                16u32 => ::core::write!(f, "{}({})", "RESIZE_X", 16u32),
                32u32 => ::core::write!(f, "{}({})", "RESIZE_Y", 32u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "desktop user interface surface base interface"]
#[doc = ""]
#[doc = "`Version 5`"]
#[doc = ""]
#[doc = "An interface that may be implemented by a wl_surface, for\nimplementations that provide a desktop-style user interface.\n\nIt provides a base set of functionality required to construct user\ninterface elements requiring management by the compositor, such as\ntoplevel windows, menus, etc. The types of functionality are split into\nxdg_surface roles.\n\nCreating an xdg_surface does not set the role for a wl_surface. In order\nto map an xdg_surface, the client must create a role-specific object\nusing, e.g., get_toplevel, get_popup. The wl_surface for any given\nxdg_surface can have at most one role, and may not be assigned any role\nnot based on xdg_surface.\n\nA role must be assigned before any other requests are made to the\nxdg_surface object.\n\nThe client must call wl_surface.commit on the corresponding wl_surface\nfor the xdg_surface state to take effect.\n\nCreating an xdg_surface from a wl_surface which has a buffer attached or\ncommitted is a client error, and any attempts by a client to attach or\nmanipulate a buffer prior to the first xdg_surface.configure call must\nalso be treated as errors.\n\nAfter creating a role-specific object and setting it up, the client must\nperform an initial commit without any buffer attached. The compositor\nwill reply with an xdg_surface.configure event. The client must\nacknowledge it and is then allowed to attach a buffer to map the surface.\n\nMapping an xdg_surface-based role surface is defined as making it\npossible for the surface to be shown by the compositor. Note that\na mapped surface is not guaranteed to be visible once it is mapped.\n\nFor an xdg_surface to be mapped by the compositor, the following\nconditions must be met:\n(1) the client has assigned an xdg_surface-based role to the surface\n(2) the client has set and committed the xdg_surface state and the\nrole-dependent state to the surface\n(3) the client has committed a buffer to the surface\n\nA newly-unmapped surface is considered to have met condition (1) out\nof the 3 required conditions for mapping a surface if its role surface\nhas not been destroyed, i.e. the client must perform the initial commit\nagain before attaching a buffer."]
pub trait r#XdgSurface<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "xdg_surface";
    const VERSION: ::core::primitive::u32 = 5u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::yutani::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::yutani::wire::EventLoop<T>,
        _client: &mut ::yutani::server::Client<T>,
        _message: ::yutani::wire::Message,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        let _this: ::yutani::lease::Lease<Self> =
            _this.downcast().ok_or(::yutani::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_surface", "@{}.", "destroy", "(", ")"),
                        _this.id(),
                    );
                }
                Self::r#destroy(_this, _event_loop, _client)
            }
            1u16 => {
                let _stream = _client.stream();
                let r#id = _stream
                    .object()?
                    .ok_or(::yutani::wire::WlError::NON_NULLABLE)?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_surface", "@{}.", "get_toplevel", "(", "{:?}", ")"),
                        _this.id(),
                        r#id,
                    );
                }
                Self::r#get_toplevel(_this, _event_loop, _client, r#id)
            }
            2u16 => {
                let _stream = _client.stream();
                let r#id = _stream
                    .object()?
                    .ok_or(::yutani::wire::WlError::NON_NULLABLE)?;
                let r#parent = _stream.object()?;
                let r#positioner = _stream
                    .object()?
                    .ok_or(::yutani::wire::WlError::NON_NULLABLE)?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "xdg_surface",
                            "@{}.",
                            "get_popup",
                            "(",
                            "{:?}",
                            ", {:?}",
                            ", {:?}",
                            ")"
                        ),
                        _this.id(),
                        r#id,
                        r#parent,
                        r#positioner,
                    );
                }
                Self::r#get_popup(_this, _event_loop, _client, r#id, r#parent, r#positioner)
            }
            3u16 => {
                let _stream = _client.stream();
                let r#x = _stream.i32()?;
                let r#y = _stream.i32()?;
                let r#width = _stream.i32()?;
                let r#height = _stream.i32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "xdg_surface",
                            "@{}.",
                            "set_window_geometry",
                            "(",
                            "{:?}",
                            ", {:?}",
                            ", {:?}",
                            ", {:?}",
                            ")"
                        ),
                        _this.id(),
                        r#x,
                        r#y,
                        r#width,
                        r#height,
                    );
                }
                Self::r#set_window_geometry(
                    _this,
                    _event_loop,
                    _client,
                    r#x,
                    r#y,
                    r#width,
                    r#height,
                )
            }
            4u16 => {
                let _stream = _client.stream();
                let r#serial = _stream.u32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_surface", "@{}.", "ack_configure", "(", "{:?}", ")"),
                        _this.id(),
                        r#serial,
                    );
                }
                Self::r#ack_configure(_this, _event_loop, _client, r#serial)
            }
            _ => ::core::result::Result::Err(::yutani::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `yutani`"]
    fn into_object(
        self,
        id: ::yutani::Id,
    ) -> ::yutani::lease::Resident<Self, T, ::yutani::server::Client<T>> {
        ::yutani::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `yutani`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::yutani::Id,
        version: u32,
    ) -> ::core::result::Result<
        ::yutani::lease::Resident<Self, T, ::yutani::server::Client<T>>,
        ::yutani::wire::WlError<'static>,
    > {
        if version > Self::VERSION {
            ::core::result::Result::Err(::yutani::wire::WlError::UNSUPPORTED_VERSION)
        } else {
            ::core::result::Result::Ok(::yutani::lease::Resident::new(
                id,
                Self::dispatch,
                Self::INTERFACE,
                version,
                self,
            ))
        }
    }
    #[doc = "destroy the xdg_surface"]
    #[doc = ""]
    #[doc = "Destroy the xdg_surface object. An xdg_surface must only be destroyed\nafter its role object has been destroyed."]
    fn r#destroy(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "assign the xdg_toplevel surface role"]
    #[doc = ""]
    #[doc = "This creates an xdg_toplevel object for the given xdg_surface and gives\nthe associated wl_surface the xdg_toplevel role.\n\nSee the documentation of xdg_toplevel for more details about what an\nxdg_toplevel is and how it is used."]
    fn r#get_toplevel(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#id: ::yutani::Id,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "assign the xdg_popup surface role"]
    #[doc = ""]
    #[doc = "This creates an xdg_popup object for the given xdg_surface and gives\nthe associated wl_surface the xdg_popup role.\n\nIf null is passed as a parent, a parent surface must be specified using\nsome other protocol, before committing the initial state.\n\nSee the documentation of xdg_popup for more details about what an\nxdg_popup is and how it is used."]
    fn r#get_popup(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#id: ::yutani::Id,
        r#parent: ::core::option::Option<::yutani::Id>,
        r#positioner: ::yutani::Id,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "set the new window geometry"]
    #[doc = ""]
    #[doc = "The window geometry of a surface is its \"visible bounds\" from the\nuser's perspective. Client-side decorations often have invisible\nportions like drop-shadows which should be ignored for the\npurposes of aligning, placing and constraining windows.\n\nThe window geometry is double buffered, and will be applied at the\ntime wl_surface.commit of the corresponding wl_surface is called.\n\nWhen maintaining a position, the compositor should treat the (x, y)\ncoordinate of the window geometry as the top left corner of the window.\nA client changing the (x, y) window geometry coordinate should in\ngeneral not alter the position of the window.\n\nOnce the window geometry of the surface is set, it is not possible to\nunset it, and it will remain the same until set_window_geometry is\ncalled again, even if a new subsurface or buffer is attached.\n\nIf never set, the value is the full bounds of the surface,\nincluding any subsurfaces. This updates dynamically on every\ncommit. This unset is meant for extremely simple clients.\n\nThe arguments are given in the surface-local coordinate space of\nthe wl_surface associated with this xdg_surface.\n\nThe width and height must be greater than zero. Setting an invalid size\nwill raise an error. When applied, the effective window geometry will be\nthe set window geometry clamped to the bounding rectangle of the\ncombined geometry of the surface of the xdg_surface and the associated\nsubsurfaces."]
    fn r#set_window_geometry(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#x: ::core::primitive::i32,
        r#y: ::core::primitive::i32,
        r#width: ::core::primitive::i32,
        r#height: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "ack a configure event"]
    #[doc = ""]
    #[doc = "When a configure event is received, if a client commits the\nsurface in response to the configure event, then the client\nmust make an ack_configure request sometime before the commit\nrequest, passing along the serial of the configure event.\n\nFor instance, for toplevel surfaces the compositor might use this\ninformation to move a surface to the top left only when the client has\ndrawn itself for the maximized or fullscreen state.\n\nIf the client receives multiple configure events before it\ncan respond to one, it only has to ack the last configure event.\n\nA client is not required to commit immediately after sending\nan ack_configure request - it may even ack_configure several times\nbefore its next surface commit.\n\nA client may send multiple ack_configure requests before committing, but\nonly the last request sent before a commit indicates which configure\nevent the client really is responding to.\n\nSending an ack_configure request consumes the serial number sent with\nthe request, as well as serial numbers sent by all configure events\nsent on this xdg_surface prior to the configure event referenced by\nthe committed serial.\n\nIt is an error to issue multiple ack_configure requests referencing a\nserial from the same configure event, or to issue an ack_configure\nrequest referencing a serial from a configure event issued before the\nevent identified by the last ack_configure request for the same\nxdg_surface. Doing so will raise an invalid_serial error."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: the serial from the configure event"]
    fn r#ack_configure(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#serial: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "suggest a surface change"]
    #[doc = ""]
    #[doc = "The configure event marks the end of a configure sequence. A configure\nsequence is a set of one or more events configuring the state of the\nxdg_surface, including the final xdg_surface.configure event.\n\nWhere applicable, xdg_surface surface roles will during a configure\nsequence extend this event as a latched state sent as events before the\nxdg_surface.configure event. Such events should be considered to make up\na set of atomically applied configuration states, where the\nxdg_surface.configure commits the accumulated state.\n\nClients should arrange their surface for the new states, and then send\nan ack_configure request with the serial sent in this configure event at\nsome point before committing the new surface.\n\nIf the client receives multiple configure events before it can respond\nto one, it is free to discard all but the last event it received."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`serial`: serial of the configure event"]
    fn r#configure(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
        r#serial: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(" -> ", "xdg_surface", "@{}.", "configure", "(", "{:?}", ")"),
                _this.id(),
                r#serial,
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 0u16);
        _stream.send_u32(r#serial)?;
        _stream.commit(_key)
    }
}
pub mod r#xdg_surface {
    #[doc = ""]
    #[repr(transparent)]
    pub struct r#Error(u32);
    impl r#Error {
        #[doc = ""]
        pub const r#NOT_CONSTRUCTED: Self = Self(1u32);
        #[doc = ""]
        pub const r#ALREADY_CONSTRUCTED: Self = Self(2u32);
        #[doc = ""]
        pub const r#UNCONFIGURED_BUFFER: Self = Self(3u32);
        #[doc = ""]
        pub const r#INVALID_SERIAL: Self = Self(4u32);
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
                1u32 => ::core::write!(f, "{}({})", "NOT_CONSTRUCTED", 1u32),
                2u32 => ::core::write!(f, "{}({})", "ALREADY_CONSTRUCTED", 2u32),
                3u32 => ::core::write!(f, "{}({})", "UNCONFIGURED_BUFFER", 3u32),
                4u32 => ::core::write!(f, "{}({})", "INVALID_SERIAL", 4u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "toplevel surface"]
#[doc = ""]
#[doc = "`Version 5`"]
#[doc = ""]
#[doc = "This interface defines an xdg_surface role which allows a surface to,\namong other things, set window-like properties such as maximize,\nfullscreen, and minimize, set application-specific metadata like title and\nid, and well as trigger user interactive operations such as interactive\nresize and move.\n\nUnmapping an xdg_toplevel means that the surface cannot be shown\nby the compositor until it is explicitly mapped again.\nAll active operations (e.g., move, resize) are canceled and all\nattributes (e.g. title, state, stacking, ...) are discarded for\nan xdg_toplevel surface when it is unmapped. The xdg_toplevel returns to\nthe state it had right after xdg_surface.get_toplevel. The client\ncan re-map the toplevel by perfoming a commit without any buffer\nattached, waiting for a configure event and handling it as usual (see\nxdg_surface description).\n\nAttaching a null buffer to a toplevel unmaps the surface."]
pub trait r#XdgToplevel<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "xdg_toplevel";
    const VERSION: ::core::primitive::u32 = 5u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::yutani::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::yutani::wire::EventLoop<T>,
        _client: &mut ::yutani::server::Client<T>,
        _message: ::yutani::wire::Message,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        let _this: ::yutani::lease::Lease<Self> =
            _this.downcast().ok_or(::yutani::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_toplevel", "@{}.", "destroy", "(", ")"),
                        _this.id(),
                    );
                }
                Self::r#destroy(_this, _event_loop, _client)
            }
            1u16 => {
                let _stream = _client.stream();
                let r#parent = _stream.object()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_toplevel", "@{}.", "set_parent", "(", "{:?}", ")"),
                        _this.id(),
                        r#parent,
                    );
                }
                Self::r#set_parent(_this, _event_loop, _client, r#parent)
            }
            2u16 => {
                let _stream = _client.stream();
                let r#title = _stream
                    .string()?
                    .ok_or(::yutani::wire::WlError::NON_NULLABLE)?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_toplevel", "@{}.", "set_title", "(", "{:?}", ")"),
                        _this.id(),
                        r#title,
                    );
                }
                Self::r#set_title(_this, _event_loop, _client, r#title)
            }
            3u16 => {
                let _stream = _client.stream();
                let r#app_id = _stream
                    .string()?
                    .ok_or(::yutani::wire::WlError::NON_NULLABLE)?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_toplevel", "@{}.", "set_app_id", "(", "{:?}", ")"),
                        _this.id(),
                        r#app_id,
                    );
                }
                Self::r#set_app_id(_this, _event_loop, _client, r#app_id)
            }
            4u16 => {
                let _stream = _client.stream();
                let r#seat = _stream
                    .object()?
                    .ok_or(::yutani::wire::WlError::NON_NULLABLE)?;
                let r#serial = _stream.u32()?;
                let r#x = _stream.i32()?;
                let r#y = _stream.i32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "xdg_toplevel",
                            "@{}.",
                            "show_window_menu",
                            "(",
                            "{:?}",
                            ", {:?}",
                            ", {:?}",
                            ", {:?}",
                            ")"
                        ),
                        _this.id(),
                        r#seat,
                        r#serial,
                        r#x,
                        r#y,
                    );
                }
                Self::r#show_window_menu(_this, _event_loop, _client, r#seat, r#serial, r#x, r#y)
            }
            5u16 => {
                let _stream = _client.stream();
                let r#seat = _stream
                    .object()?
                    .ok_or(::yutani::wire::WlError::NON_NULLABLE)?;
                let r#serial = _stream.u32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_toplevel", "@{}.", "move", "(", "{:?}", ", {:?}", ")"),
                        _this.id(),
                        r#seat,
                        r#serial,
                    );
                }
                Self::r#move(_this, _event_loop, _client, r#seat, r#serial)
            }
            6u16 => {
                let _stream = _client.stream();
                let r#seat = _stream
                    .object()?
                    .ok_or(::yutani::wire::WlError::NON_NULLABLE)?;
                let r#serial = _stream.u32()?;
                let r#edges = _stream.u32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "xdg_toplevel",
                            "@{}.",
                            "resize",
                            "(",
                            "{:?}",
                            ", {:?}",
                            ", {:?}",
                            ")"
                        ),
                        _this.id(),
                        r#seat,
                        r#serial,
                        r#edges,
                    );
                }
                Self::r#resize(_this, _event_loop, _client, r#seat, r#serial, r#edges)
            }
            7u16 => {
                let _stream = _client.stream();
                let r#width = _stream.i32()?;
                let r#height = _stream.i32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "xdg_toplevel",
                            "@{}.",
                            "set_max_size",
                            "(",
                            "{:?}",
                            ", {:?}",
                            ")"
                        ),
                        _this.id(),
                        r#width,
                        r#height,
                    );
                }
                Self::r#set_max_size(_this, _event_loop, _client, r#width, r#height)
            }
            8u16 => {
                let _stream = _client.stream();
                let r#width = _stream.i32()?;
                let r#height = _stream.i32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "xdg_toplevel",
                            "@{}.",
                            "set_min_size",
                            "(",
                            "{:?}",
                            ", {:?}",
                            ")"
                        ),
                        _this.id(),
                        r#width,
                        r#height,
                    );
                }
                Self::r#set_min_size(_this, _event_loop, _client, r#width, r#height)
            }
            9u16 => {
                let _stream = _client.stream();
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_toplevel", "@{}.", "set_maximized", "(", ")"),
                        _this.id(),
                    );
                }
                Self::r#set_maximized(_this, _event_loop, _client)
            }
            10u16 => {
                let _stream = _client.stream();
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_toplevel", "@{}.", "unset_maximized", "(", ")"),
                        _this.id(),
                    );
                }
                Self::r#unset_maximized(_this, _event_loop, _client)
            }
            11u16 => {
                let _stream = _client.stream();
                let r#output = _stream.object()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_toplevel", "@{}.", "set_fullscreen", "(", "{:?}", ")"),
                        _this.id(),
                        r#output,
                    );
                }
                Self::r#set_fullscreen(_this, _event_loop, _client, r#output)
            }
            12u16 => {
                let _stream = _client.stream();
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_toplevel", "@{}.", "unset_fullscreen", "(", ")"),
                        _this.id(),
                    );
                }
                Self::r#unset_fullscreen(_this, _event_loop, _client)
            }
            13u16 => {
                let _stream = _client.stream();
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_toplevel", "@{}.", "set_minimized", "(", ")"),
                        _this.id(),
                    );
                }
                Self::r#set_minimized(_this, _event_loop, _client)
            }
            _ => ::core::result::Result::Err(::yutani::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `yutani`"]
    fn into_object(
        self,
        id: ::yutani::Id,
    ) -> ::yutani::lease::Resident<Self, T, ::yutani::server::Client<T>> {
        ::yutani::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `yutani`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::yutani::Id,
        version: u32,
    ) -> ::core::result::Result<
        ::yutani::lease::Resident<Self, T, ::yutani::server::Client<T>>,
        ::yutani::wire::WlError<'static>,
    > {
        if version > Self::VERSION {
            ::core::result::Result::Err(::yutani::wire::WlError::UNSUPPORTED_VERSION)
        } else {
            ::core::result::Result::Ok(::yutani::lease::Resident::new(
                id,
                Self::dispatch,
                Self::INTERFACE,
                version,
                self,
            ))
        }
    }
    #[doc = "destroy the xdg_toplevel"]
    #[doc = ""]
    #[doc = "This request destroys the role surface and unmaps the surface;\nsee \"Unmapping\" behavior in interface section for details."]
    fn r#destroy(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "set the parent of this surface"]
    #[doc = ""]
    #[doc = "Set the \"parent\" of this surface. This surface should be stacked\nabove the parent surface and all other ancestor surfaces.\n\nParent surfaces should be set on dialogs, toolboxes, or other\n\"auxiliary\" surfaces, so that the parent is raised when the dialog\nis raised.\n\nSetting a null parent for a child surface unsets its parent. Setting\na null parent for a surface which currently has no parent is a no-op.\n\nOnly mapped surfaces can have child surfaces. Setting a parent which\nis not mapped is equivalent to setting a null parent. If a surface\nbecomes unmapped, its children's parent is set to the parent of\nthe now-unmapped surface. If the now-unmapped surface has no parent,\nits children's parent is unset. If the now-unmapped surface becomes\nmapped again, its parent-child relationship is not restored.\n\nThe parent toplevel must not be one of the child toplevel's\ndescendants, and the parent must be different from the child toplevel,\notherwise the invalid_parent protocol error is raised."]
    fn r#set_parent(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#parent: ::core::option::Option<::yutani::Id>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "set surface title"]
    #[doc = ""]
    #[doc = "Set a short title for the surface.\n\nThis string may be used to identify the surface in a task bar,\nwindow list, or other user interface elements provided by the\ncompositor.\n\nThe string must be encoded in UTF-8."]
    fn r#set_title(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#title: ::std::string::String,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "set application ID"]
    #[doc = ""]
    #[doc = "Set an application identifier for the surface.\n\nThe app ID identifies the general class of applications to which\nthe surface belongs. The compositor can use this to group multiple\nsurfaces together, or to determine how to launch a new application.\n\nFor D-Bus activatable applications, the app ID is used as the D-Bus\nservice name.\n\nThe compositor shell will try to group application surfaces together\nby their app ID. As a best practice, it is suggested to select app\nID's that match the basename of the application's .desktop file.\nFor example, \"org.freedesktop.FooViewer\" where the .desktop file is\n\"org.freedesktop.FooViewer.desktop\".\n\nLike other properties, a set_app_id request can be sent after the\nxdg_toplevel has been mapped to update the property.\n\nSee the desktop-entry specification [0] for more details on\napplication identifiers and how they relate to well-known D-Bus\nnames and .desktop files.\n\n[0] http://standards.freedesktop.org/desktop-entry-spec/"]
    fn r#set_app_id(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#app_id: ::std::string::String,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "show the window menu"]
    #[doc = ""]
    #[doc = "Clients implementing client-side decorations might want to show\na context menu when right-clicking on the decorations, giving the\nuser a menu that they can use to maximize or minimize the window.\n\nThis request asks the compositor to pop up such a window menu at\nthe given position, relative to the local surface coordinates of\nthe parent surface. There are no guarantees as to what menu items\nthe window menu contains.\n\nThis request must be used in response to some sort of user action\nlike a button press, key press, or touch down event."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`seat`: the wl_seat of the user event"]
    #[doc = "\n`serial`: the serial of the user event"]
    #[doc = "\n`x`: the x position to pop up the window menu at"]
    #[doc = "\n`y`: the y position to pop up the window menu at"]
    fn r#show_window_menu(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#seat: ::yutani::Id,
        r#serial: ::core::primitive::u32,
        r#x: ::core::primitive::i32,
        r#y: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "start an interactive move"]
    #[doc = ""]
    #[doc = "Start an interactive, user-driven move of the surface.\n\nThis request must be used in response to some sort of user action\nlike a button press, key press, or touch down event. The passed\nserial is used to determine the type of interactive move (touch,\npointer, etc).\n\nThe server may ignore move requests depending on the state of\nthe surface (e.g. fullscreen or maximized), or if the passed serial\nis no longer valid.\n\nIf triggered, the surface will lose the focus of the device\n(wl_pointer, wl_touch, etc) used for the move. It is up to the\ncompositor to visually indicate that the move is taking place, such as\nupdating a pointer cursor, during the move. There is no guarantee\nthat the device focus will return when the move is completed."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`seat`: the wl_seat of the user event"]
    #[doc = "\n`serial`: the serial of the user event"]
    fn r#move(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#seat: ::yutani::Id,
        r#serial: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "start an interactive resize"]
    #[doc = ""]
    #[doc = "Start a user-driven, interactive resize of the surface.\n\nThis request must be used in response to some sort of user action\nlike a button press, key press, or touch down event. The passed\nserial is used to determine the type of interactive resize (touch,\npointer, etc).\n\nThe server may ignore resize requests depending on the state of\nthe surface (e.g. fullscreen or maximized).\n\nIf triggered, the client will receive configure events with the\n\"resize\" state enum value and the expected sizes. See the \"resize\"\nenum value for more details about what is required. The client\nmust also acknowledge configure events using \"ack_configure\". After\nthe resize is completed, the client will receive another \"configure\"\nevent without the resize state.\n\nIf triggered, the surface also will lose the focus of the device\n(wl_pointer, wl_touch, etc) used for the resize. It is up to the\ncompositor to visually indicate that the resize is taking place,\nsuch as updating a pointer cursor, during the resize. There is no\nguarantee that the device focus will return when the resize is\ncompleted.\n\nThe edges parameter specifies how the surface should be resized, and\nis one of the values of the resize_edge enum. Values not matching\na variant of the enum will cause a protocol error. The compositor\nmay use this information to update the surface position for example\nwhen dragging the top left corner. The compositor may also use\nthis information to adapt its behavior, e.g. choose an appropriate\ncursor image."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`seat`: the wl_seat of the user event"]
    #[doc = "\n`serial`: the serial of the user event"]
    #[doc = "\n`edges`: which edge or corner is being dragged"]
    fn r#resize(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#seat: ::yutani::Id,
        r#serial: ::core::primitive::u32,
        r#edges: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "set the maximum size"]
    #[doc = ""]
    #[doc = "Set a maximum size for the window.\n\nThe client can specify a maximum size so that the compositor does\nnot try to configure the window beyond this size.\n\nThe width and height arguments are in window geometry coordinates.\nSee xdg_surface.set_window_geometry.\n\nValues set in this way are double-buffered. They will get applied\non the next commit.\n\nThe compositor can use this information to allow or disallow\ndifferent states like maximize or fullscreen and draw accurate\nanimations.\n\nSimilarly, a tiling window manager may use this information to\nplace and resize client windows in a more effective way.\n\nThe client should not rely on the compositor to obey the maximum\nsize. The compositor may decide to ignore the values set by the\nclient and request a larger size.\n\nIf never set, or a value of zero in the request, means that the\nclient has no expected maximum size in the given dimension.\nAs a result, a client wishing to reset the maximum size\nto an unspecified state can use zero for width and height in the\nrequest.\n\nRequesting a maximum size to be smaller than the minimum size of\na surface is illegal and will result in a protocol error.\n\nThe width and height must be greater than or equal to zero. Using\nstrictly negative values for width and height will result in a\nprotocol error."]
    fn r#set_max_size(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#width: ::core::primitive::i32,
        r#height: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "set the minimum size"]
    #[doc = ""]
    #[doc = "Set a minimum size for the window.\n\nThe client can specify a minimum size so that the compositor does\nnot try to configure the window below this size.\n\nThe width and height arguments are in window geometry coordinates.\nSee xdg_surface.set_window_geometry.\n\nValues set in this way are double-buffered. They will get applied\non the next commit.\n\nThe compositor can use this information to allow or disallow\ndifferent states like maximize or fullscreen and draw accurate\nanimations.\n\nSimilarly, a tiling window manager may use this information to\nplace and resize client windows in a more effective way.\n\nThe client should not rely on the compositor to obey the minimum\nsize. The compositor may decide to ignore the values set by the\nclient and request a smaller size.\n\nIf never set, or a value of zero in the request, means that the\nclient has no expected minimum size in the given dimension.\nAs a result, a client wishing to reset the minimum size\nto an unspecified state can use zero for width and height in the\nrequest.\n\nRequesting a minimum size to be larger than the maximum size of\na surface is illegal and will result in a protocol error.\n\nThe width and height must be greater than or equal to zero. Using\nstrictly negative values for width and height will result in a\nprotocol error."]
    fn r#set_min_size(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#width: ::core::primitive::i32,
        r#height: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "maximize the window"]
    #[doc = ""]
    #[doc = "Maximize the surface.\n\nAfter requesting that the surface should be maximized, the compositor\nwill respond by emitting a configure event. Whether this configure\nactually sets the window maximized is subject to compositor policies.\nThe client must then update its content, drawing in the configured\nstate. The client must also acknowledge the configure when committing\nthe new content (see ack_configure).\n\nIt is up to the compositor to decide how and where to maximize the\nsurface, for example which output and what region of the screen should\nbe used.\n\nIf the surface was already maximized, the compositor will still emit\na configure event with the \"maximized\" state.\n\nIf the surface is in a fullscreen state, this request has no direct\neffect. It may alter the state the surface is returned to when\nunmaximized unless overridden by the compositor."]
    fn r#set_maximized(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "unmaximize the window"]
    #[doc = ""]
    #[doc = "Unmaximize the surface.\n\nAfter requesting that the surface should be unmaximized, the compositor\nwill respond by emitting a configure event. Whether this actually\nun-maximizes the window is subject to compositor policies.\nIf available and applicable, the compositor will include the window\ngeometry dimensions the window had prior to being maximized in the\nconfigure event. The client must then update its content, drawing it in\nthe configured state. The client must also acknowledge the configure\nwhen committing the new content (see ack_configure).\n\nIt is up to the compositor to position the surface after it was\nunmaximized; usually the position the surface had before maximizing, if\napplicable.\n\nIf the surface was already not maximized, the compositor will still\nemit a configure event without the \"maximized\" state.\n\nIf the surface is in a fullscreen state, this request has no direct\neffect. It may alter the state the surface is returned to when\nunmaximized unless overridden by the compositor."]
    fn r#unset_maximized(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "set the window as fullscreen on an output"]
    #[doc = ""]
    #[doc = "Make the surface fullscreen.\n\nAfter requesting that the surface should be fullscreened, the\ncompositor will respond by emitting a configure event. Whether the\nclient is actually put into a fullscreen state is subject to compositor\npolicies. The client must also acknowledge the configure when\ncommitting the new content (see ack_configure).\n\nThe output passed by the request indicates the client's preference as\nto which display it should be set fullscreen on. If this value is NULL,\nit's up to the compositor to choose which display will be used to map\nthis surface.\n\nIf the surface doesn't cover the whole output, the compositor will\nposition the surface in the center of the output and compensate with\nwith border fill covering the rest of the output. The content of the\nborder fill is undefined, but should be assumed to be in some way that\nattempts to blend into the surrounding area (e.g. solid black).\n\nIf the fullscreened surface is not opaque, the compositor must make\nsure that other screen content not part of the same surface tree (made\nup of subsurfaces, popups or similarly coupled surfaces) are not\nvisible below the fullscreened surface."]
    fn r#set_fullscreen(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#output: ::core::option::Option<::yutani::Id>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "unset the window as fullscreen"]
    #[doc = ""]
    #[doc = "Make the surface no longer fullscreen.\n\nAfter requesting that the surface should be unfullscreened, the\ncompositor will respond by emitting a configure event.\nWhether this actually removes the fullscreen state of the client is\nsubject to compositor policies.\n\nMaking a surface unfullscreen sets states for the surface based on the following:\n* the state(s) it may have had before becoming fullscreen\n* any state(s) decided by the compositor\n* any state(s) requested by the client while the surface was fullscreen\n\nThe compositor may include the previous window geometry dimensions in\nthe configure event, if applicable.\n\nThe client must also acknowledge the configure when committing the new\ncontent (see ack_configure)."]
    fn r#unset_fullscreen(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "set the window as minimized"]
    #[doc = ""]
    #[doc = "Request that the compositor minimize your surface. There is no\nway to know if the surface is currently minimized, nor is there\nany way to unset minimization on this surface.\n\nIf you are looking to throttle redrawing when minimized, please\ninstead use the wl_surface.frame event for this, as this will\nalso work with live previews on windows in Alt-Tab, Expose or\nsimilar compositor features."]
    fn r#set_minimized(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "suggest a surface change"]
    #[doc = ""]
    #[doc = "This configure event asks the client to resize its toplevel surface or\nto change its state. The configured state should not be applied\nimmediately. See xdg_surface.configure for details.\n\nThe width and height arguments specify a hint to the window\nabout how its surface should be resized in window geometry\ncoordinates. See set_window_geometry.\n\nIf the width or height arguments are zero, it means the client\nshould decide its own window dimension. This may happen when the\ncompositor needs to configure the state of the surface but doesn't\nhave any information about any previous or expected dimension.\n\nThe states listed in the event specify how the width/height\narguments should be interpreted, and possibly how it should be\ndrawn.\n\nClients must send an ack_configure in response to this event. See\nxdg_surface.configure and xdg_surface.ack_configure for details."]
    fn r#configure(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
        r#width: ::core::primitive::i32,
        r#height: ::core::primitive::i32,
        r#states: &'_ [::core::primitive::u8],
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(
                    " -> ",
                    "xdg_toplevel",
                    "@{}.",
                    "configure",
                    "(",
                    "{:?}",
                    ", {:?}",
                    ", {:?}",
                    ")"
                ),
                _this.id(),
                r#width,
                r#height,
                r#states,
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 0u16);
        _stream.send_i32(r#width)?;
        _stream.send_i32(r#height)?;
        _stream.send_bytes(r#states)?;
        _stream.commit(_key)
    }
    #[doc = "surface wants to be closed"]
    #[doc = ""]
    #[doc = "The close event is sent by the compositor when the user\nwants the surface to be closed. This should be equivalent to\nthe user clicking the close button in client-side decorations,\nif your application has any.\n\nThis is only a request that the user intends to close the\nwindow. The client may choose to ignore this request, or show\na dialog to ask the user to save their data, etc."]
    fn r#close(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(" -> ", "xdg_toplevel", "@{}.", "close", "(", ")"),
                _this.id(),
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 1u16);
        _stream.commit(_key)
    }
    #[doc = "recommended window geometry bounds"]
    #[doc = ""]
    #[doc = "`Since version 4`"]
    #[doc = ""]
    #[doc = "The configure_bounds event may be sent prior to a xdg_toplevel.configure\nevent to communicate the bounds a window geometry size is recommended\nto constrain to.\n\nThe passed width and height are in surface coordinate space. If width\nand height are 0, it means bounds is unknown and equivalent to as if no\nconfigure_bounds event was ever sent for this surface.\n\nThe bounds can for example correspond to the size of a monitor excluding\nany panels or other shell components, so that a surface isn't created in\na way that it cannot fit.\n\nThe bounds may change at any point, and in such a case, a new\nxdg_toplevel.configure_bounds will be sent, followed by\nxdg_toplevel.configure and xdg_surface.configure."]
    fn r#configure_bounds(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
        r#width: ::core::primitive::i32,
        r#height: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(
                    " -> ",
                    "xdg_toplevel",
                    "@{}.",
                    "configure_bounds",
                    "(",
                    "{:?}",
                    ", {:?}",
                    ")"
                ),
                _this.id(),
                r#width,
                r#height,
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 2u16);
        _stream.send_i32(r#width)?;
        _stream.send_i32(r#height)?;
        _stream.commit(_key)
    }
    #[doc = "compositor capabilities"]
    #[doc = ""]
    #[doc = "`Since version 5`"]
    #[doc = ""]
    #[doc = "This event advertises the capabilities supported by the compositor. If\na capability isn't supported, clients should hide or disable the UI\nelements that expose this functionality. For instance, if the\ncompositor doesn't advertise support for minimized toplevels, a button\ntriggering the set_minimized request should not be displayed.\n\nThe compositor will ignore requests it doesn't support. For instance,\na compositor which doesn't advertise support for minimized will ignore\nset_minimized requests.\n\nCompositors must send this event once before the first\nxdg_surface.configure event. When the capabilities change, compositors\nmust send this event again and then send an xdg_surface.configure\nevent.\n\nThe configured state should not be applied immediately. See\nxdg_surface.configure for details.\n\nThe capabilities are sent as an array of 32-bit unsigned integers in\nnative endianness."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`capabilities`: array of 32-bit capabilities"]
    fn r#wm_capabilities(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
        r#capabilities: &'_ [::core::primitive::u8],
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(
                    " -> ",
                    "xdg_toplevel",
                    "@{}.",
                    "wm_capabilities",
                    "(",
                    "{:?}",
                    ")"
                ),
                _this.id(),
                r#capabilities,
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 3u16);
        _stream.send_bytes(r#capabilities)?;
        _stream.commit(_key)
    }
}
pub mod r#xdg_toplevel {
    #[doc = ""]
    #[repr(transparent)]
    pub struct r#Error(u32);
    impl r#Error {
        #[doc = "provided value is\n        not a valid variant of the resize_edge enum"]
        #[doc = ""]
        pub const r#INVALID_RESIZE_EDGE: Self = Self(0u32);
        #[doc = "invalid parent toplevel"]
        #[doc = ""]
        pub const r#INVALID_PARENT: Self = Self(1u32);
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
                0u32 => ::core::write!(f, "{}({})", "INVALID_RESIZE_EDGE", 0u32),
                1u32 => ::core::write!(f, "{}({})", "INVALID_PARENT", 1u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
    #[doc = "edge values for resizing"]
    #[doc = ""]
    #[doc = "These values are used to indicate which edge of a surface\nis being dragged in a resize operation."]
    #[repr(transparent)]
    pub struct r#ResizeEdge(u32);
    impl r#ResizeEdge {
        #[doc = ""]
        pub const r#NONE: Self = Self(0u32);
        #[doc = ""]
        pub const r#TOP: Self = Self(1u32);
        #[doc = ""]
        pub const r#BOTTOM: Self = Self(2u32);
        #[doc = ""]
        pub const r#LEFT: Self = Self(4u32);
        #[doc = ""]
        pub const r#TOP_LEFT: Self = Self(5u32);
        #[doc = ""]
        pub const r#BOTTOM_LEFT: Self = Self(6u32);
        #[doc = ""]
        pub const r#RIGHT: Self = Self(8u32);
        #[doc = ""]
        pub const r#TOP_RIGHT: Self = Self(9u32);
        #[doc = ""]
        pub const r#BOTTOM_RIGHT: Self = Self(10u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#ResizeEdge {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#ResizeEdge {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#ResizeEdge {
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
    #[doc = "types of state on the surface"]
    #[doc = ""]
    #[doc = "The different state values used on the surface. This is designed for\nstate values like maximized, fullscreen. It is paired with the\nconfigure event to ensure that both the client and the compositor\nsetting the state can be synchronized.\n\nStates set in this way are double-buffered. They will get applied on\nthe next commit."]
    #[repr(transparent)]
    pub struct r#State(u32);
    impl r#State {
        #[doc = "the surface is maximized"]
        #[doc = ""]
        #[doc = "The surface is maximized. The window geometry specified in the configure\nevent must be obeyed by the client.\n\nThe client should draw without shadow or other\ndecoration outside of the window geometry."]
        pub const r#MAXIMIZED: Self = Self(1u32);
        #[doc = "the surface is fullscreen"]
        #[doc = ""]
        #[doc = "The surface is fullscreen. The window geometry specified in the\nconfigure event is a maximum; the client cannot resize beyond it. For\na surface to cover the whole fullscreened area, the geometry\ndimensions must be obeyed by the client. For more details, see\nxdg_toplevel.set_fullscreen."]
        pub const r#FULLSCREEN: Self = Self(2u32);
        #[doc = "the surface is being resized"]
        #[doc = ""]
        #[doc = "The surface is being resized. The window geometry specified in the\nconfigure event is a maximum; the client cannot resize beyond it.\nClients that have aspect ratio or cell sizing configuration can use\na smaller size, however."]
        pub const r#RESIZING: Self = Self(3u32);
        #[doc = "the surface is now activated"]
        #[doc = ""]
        #[doc = "Client window decorations should be painted as if the window is\nactive. Do not assume this means that the window actually has\nkeyboard or pointer focus."]
        pub const r#ACTIVATED: Self = Self(4u32);
        #[doc = "the surface’s left edge is tiled"]
        #[doc = ""]
        #[doc = "`Since version 2`"]
        #[doc = ""]
        #[doc = "The window is currently in a tiled layout and the left edge is\nconsidered to be adjacent to another part of the tiling grid."]
        pub const r#TILED_LEFT: Self = Self(5u32);
        #[doc = "the surface’s right edge is tiled"]
        #[doc = ""]
        #[doc = "`Since version 2`"]
        #[doc = ""]
        #[doc = "The window is currently in a tiled layout and the right edge is\nconsidered to be adjacent to another part of the tiling grid."]
        pub const r#TILED_RIGHT: Self = Self(6u32);
        #[doc = "the surface’s top edge is tiled"]
        #[doc = ""]
        #[doc = "`Since version 2`"]
        #[doc = ""]
        #[doc = "The window is currently in a tiled layout and the top edge is\nconsidered to be adjacent to another part of the tiling grid."]
        pub const r#TILED_TOP: Self = Self(7u32);
        #[doc = "the surface’s bottom edge is tiled"]
        #[doc = ""]
        #[doc = "`Since version 2`"]
        #[doc = ""]
        #[doc = "The window is currently in a tiled layout and the bottom edge is\nconsidered to be adjacent to another part of the tiling grid."]
        pub const r#TILED_BOTTOM: Self = Self(8u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#State {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#State {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#State {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                1u32 => ::core::write!(f, "{}({})", "MAXIMIZED", 1u32),
                2u32 => ::core::write!(f, "{}({})", "FULLSCREEN", 2u32),
                3u32 => ::core::write!(f, "{}({})", "RESIZING", 3u32),
                4u32 => ::core::write!(f, "{}({})", "ACTIVATED", 4u32),
                5u32 => ::core::write!(f, "{}({})", "TILED_LEFT", 5u32),
                6u32 => ::core::write!(f, "{}({})", "TILED_RIGHT", 6u32),
                7u32 => ::core::write!(f, "{}({})", "TILED_TOP", 7u32),
                8u32 => ::core::write!(f, "{}({})", "TILED_BOTTOM", 8u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
    #[doc = ""]
    #[doc = "`Since version 5`"]
    #[doc = ""]
    #[repr(transparent)]
    pub struct r#WmCapabilities(u32);
    impl r#WmCapabilities {
        #[doc = "show_window_menu is available"]
        #[doc = ""]
        pub const r#WINDOW_MENU: Self = Self(1u32);
        #[doc = "set_maximized and unset_maximized are available"]
        #[doc = ""]
        pub const r#MAXIMIZE: Self = Self(2u32);
        #[doc = "set_fullscreen and unset_fullscreen are available"]
        #[doc = ""]
        pub const r#FULLSCREEN: Self = Self(3u32);
        #[doc = "set_minimized is available"]
        #[doc = ""]
        pub const r#MINIMIZE: Self = Self(4u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#WmCapabilities {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#WmCapabilities {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#WmCapabilities {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                1u32 => ::core::write!(f, "{}({})", "WINDOW_MENU", 1u32),
                2u32 => ::core::write!(f, "{}({})", "MAXIMIZE", 2u32),
                3u32 => ::core::write!(f, "{}({})", "FULLSCREEN", 3u32),
                4u32 => ::core::write!(f, "{}({})", "MINIMIZE", 4u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "short-lived, popup surfaces for menus"]
#[doc = ""]
#[doc = "`Version 5`"]
#[doc = ""]
#[doc = "A popup surface is a short-lived, temporary surface. It can be used to\nimplement for example menus, popovers, tooltips and other similar user\ninterface concepts.\n\nA popup can be made to take an explicit grab. See xdg_popup.grab for\ndetails.\n\nWhen the popup is dismissed, a popup_done event will be sent out, and at\nthe same time the surface will be unmapped. See the xdg_popup.popup_done\nevent for details.\n\nExplicitly destroying the xdg_popup object will also dismiss the popup and\nunmap the surface. Clients that want to dismiss the popup when another\nsurface of their own is clicked should dismiss the popup using the destroy\nrequest.\n\nA newly created xdg_popup will be stacked on top of all previously created\nxdg_popup surfaces associated with the same xdg_toplevel.\n\nThe parent of an xdg_popup must be mapped (see the xdg_surface\ndescription) before the xdg_popup itself.\n\nThe client must call wl_surface.commit on the corresponding wl_surface\nfor the xdg_popup state to take effect."]
pub trait r#XdgPopup<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "xdg_popup";
    const VERSION: ::core::primitive::u32 = 5u32;
    #[doc(hidden)]
    fn dispatch(
        _this: ::yutani::lease::Lease<dyn::core::any::Any>,
        _event_loop: &mut ::yutani::wire::EventLoop<T>,
        _client: &mut ::yutani::server::Client<T>,
        _message: ::yutani::wire::Message,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        let _this: ::yutani::lease::Lease<Self> =
            _this.downcast().ok_or(::yutani::wire::WlError::INTERNAL)?;
        match _message.opcode {
            0u16 => {
                let _stream = _client.stream();
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_popup", "@{}.", "destroy", "(", ")"),
                        _this.id(),
                    );
                }
                Self::r#destroy(_this, _event_loop, _client)
            }
            1u16 => {
                let _stream = _client.stream();
                let r#seat = _stream
                    .object()?
                    .ok_or(::yutani::wire::WlError::NON_NULLABLE)?;
                let r#serial = _stream.u32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!("xdg_popup", "@{}.", "grab", "(", "{:?}", ", {:?}", ")"),
                        _this.id(),
                        r#seat,
                        r#serial,
                    );
                }
                Self::r#grab(_this, _event_loop, _client, r#seat, r#serial)
            }
            2u16 => {
                let _stream = _client.stream();
                let r#positioner = _stream
                    .object()?
                    .ok_or(::yutani::wire::WlError::NON_NULLABLE)?;
                let r#token = _stream.u32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "xdg_popup",
                            "@{}.",
                            "reposition",
                            "(",
                            "{:?}",
                            ", {:?}",
                            ")"
                        ),
                        _this.id(),
                        r#positioner,
                        r#token,
                    );
                }
                Self::r#reposition(_this, _event_loop, _client, r#positioner, r#token)
            }
            _ => ::core::result::Result::Err(::yutani::wire::WlError::INVALID_OPCODE),
        }
    }
    #[doc = "Create a new object that can be tracked by `yutani`"]
    fn into_object(
        self,
        id: ::yutani::Id,
    ) -> ::yutani::lease::Resident<Self, T, ::yutani::server::Client<T>> {
        ::yutani::lease::Resident::new(id, Self::dispatch, Self::INTERFACE, Self::VERSION, self)
    }
    #[doc = "Create a new object that can be tracked by `yutani`, with a given version"]
    fn into_versioned_object(
        self,
        id: ::yutani::Id,
        version: u32,
    ) -> ::core::result::Result<
        ::yutani::lease::Resident<Self, T, ::yutani::server::Client<T>>,
        ::yutani::wire::WlError<'static>,
    > {
        if version > Self::VERSION {
            ::core::result::Result::Err(::yutani::wire::WlError::UNSUPPORTED_VERSION)
        } else {
            ::core::result::Result::Ok(::yutani::lease::Resident::new(
                id,
                Self::dispatch,
                Self::INTERFACE,
                version,
                self,
            ))
        }
    }
    #[doc = "remove xdg_popup interface"]
    #[doc = ""]
    #[doc = "This destroys the popup. Explicitly destroying the xdg_popup\nobject will also dismiss the popup, and unmap the surface.\n\nIf this xdg_popup is not the \"topmost\" popup, a protocol error\nwill be sent."]
    fn r#destroy(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "make the popup take an explicit grab"]
    #[doc = ""]
    #[doc = "This request makes the created popup take an explicit grab. An explicit\ngrab will be dismissed when the user dismisses the popup, or when the\nclient destroys the xdg_popup. This can be done by the user clicking\noutside the surface, using the keyboard, or even locking the screen\nthrough closing the lid or a timeout.\n\nIf the compositor denies the grab, the popup will be immediately\ndismissed.\n\nThis request must be used in response to some sort of user action like a\nbutton press, key press, or touch down event. The serial number of the\nevent should be passed as 'serial'.\n\nThe parent of a grabbing popup must either be an xdg_toplevel surface or\nanother xdg_popup with an explicit grab. If the parent is another\nxdg_popup it means that the popups are nested, with this popup now being\nthe topmost popup.\n\nNested popups must be destroyed in the reverse order they were created\nin, e.g. the only popup you are allowed to destroy at all times is the\ntopmost one.\n\nWhen compositors choose to dismiss a popup, they may dismiss every\nnested grabbing popup as well. When a compositor dismisses popups, it\nwill follow the same dismissing order as required from the client.\n\nIf the topmost grabbing popup is destroyed, the grab will be returned to\nthe parent of the popup, if that parent previously had an explicit grab.\n\nIf the parent is a grabbing popup which has already been dismissed, this\npopup will be immediately dismissed. If the parent is a popup that did\nnot take an explicit grab, an error will be raised.\n\nDuring a popup grab, the client owning the grab will receive pointer\nand touch events for all their surfaces as normal (similar to an\n\"owner-events\" grab in X11 parlance), while the top most grabbing popup\nwill always have keyboard focus."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`seat`: the wl_seat of the user event"]
    #[doc = "\n`serial`: the serial of the user event"]
    fn r#grab(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#seat: ::yutani::Id,
        r#serial: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "recalculate the popup's location"]
    #[doc = ""]
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "Reposition an already-mapped popup. The popup will be placed given the\ndetails in the passed xdg_positioner object, and a\nxdg_popup.repositioned followed by xdg_popup.configure and\nxdg_surface.configure will be emitted in response. Any parameters set\nby the previous positioner will be discarded.\n\nThe passed token will be sent in the corresponding\nxdg_popup.repositioned event. The new popup position will not take\neffect until the corresponding configure event is acknowledged by the\nclient. See xdg_popup.repositioned for details. The token itself is\nopaque, and has no other special meaning.\n\nIf multiple reposition requests are sent, the compositor may skip all\nbut the last one.\n\nIf the popup is repositioned in response to a configure event for its\nparent, the client should send an xdg_positioner.set_parent_configure\nand possibly an xdg_positioner.set_parent_size request to allow the\ncompositor to properly constrain the popup.\n\nIf the popup is repositioned together with a parent that is being\nresized, but not in response to a configure event, the client should\nsend an xdg_positioner.set_parent_size request."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`token`: reposition request token"]
    fn r#reposition(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#positioner: ::yutani::Id,
        r#token: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "configure the popup surface"]
    #[doc = ""]
    #[doc = "This event asks the popup surface to configure itself given the\nconfiguration. The configured state should not be applied immediately.\nSee xdg_surface.configure for details.\n\nThe x and y arguments represent the position the popup was placed at\ngiven the xdg_positioner rule, relative to the upper left corner of the\nwindow geometry of the parent surface.\n\nFor version 2 or older, the configure event for an xdg_popup is only\never sent once for the initial configuration. Starting with version 3,\nit may be sent again if the popup is setup with an xdg_positioner with\nset_reactive requested, or in response to xdg_popup.reposition requests."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`x`: x position relative to parent surface window geometry"]
    #[doc = "\n`y`: y position relative to parent surface window geometry"]
    #[doc = "\n`width`: window geometry width"]
    #[doc = "\n`height`: window geometry height"]
    fn r#configure(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
        r#x: ::core::primitive::i32,
        r#y: ::core::primitive::i32,
        r#width: ::core::primitive::i32,
        r#height: ::core::primitive::i32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(
                    " -> ",
                    "xdg_popup",
                    "@{}.",
                    "configure",
                    "(",
                    "{:?}",
                    ", {:?}",
                    ", {:?}",
                    ", {:?}",
                    ")"
                ),
                _this.id(),
                r#x,
                r#y,
                r#width,
                r#height,
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 0u16);
        _stream.send_i32(r#x)?;
        _stream.send_i32(r#y)?;
        _stream.send_i32(r#width)?;
        _stream.send_i32(r#height)?;
        _stream.commit(_key)
    }
    #[doc = "popup interaction is done"]
    #[doc = ""]
    #[doc = "The popup_done event is sent out when a popup is dismissed by the\ncompositor. The client should destroy the xdg_popup object at this\npoint."]
    fn r#popup_done(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(" -> ", "xdg_popup", "@{}.", "popup_done", "(", ")"),
                _this.id(),
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 1u16);
        _stream.commit(_key)
    }
    #[doc = "signal the completion of a repositioned request"]
    #[doc = ""]
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "The repositioned event is sent as part of a popup configuration\nsequence, together with xdg_popup.configure and lastly\nxdg_surface.configure to notify the completion of a reposition request.\n\nThe repositioned event is to notify about the completion of a\nxdg_popup.reposition request. The token argument is the token passed\nin the xdg_popup.reposition request.\n\nImmediately after this event is emitted, xdg_popup.configure and\nxdg_surface.configure will be sent with the updated size and position,\nas well as a new configure serial.\n\nThe client should optionally update the content of the popup, but must\nacknowledge the new popup configuration for the new position to take\neffect. See xdg_surface.ack_configure for details."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`token`: reposition request token"]
    fn r#repositioned(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
        r#token: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(
                    " -> ",
                    "xdg_popup",
                    "@{}.",
                    "repositioned",
                    "(",
                    "{:?}",
                    ")"
                ),
                _this.id(),
                r#token,
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 2u16);
        _stream.send_u32(r#token)?;
        _stream.commit(_key)
    }
}
pub mod r#xdg_popup {
    #[doc = ""]
    #[repr(transparent)]
    pub struct r#Error(u32);
    impl r#Error {
        #[doc = "tried to grab after being mapped"]
        #[doc = ""]
        pub const r#INVALID_GRAB: Self = Self(0u32);
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
                0u32 => ::core::write!(f, "{}({})", "INVALID_GRAB", 0u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}

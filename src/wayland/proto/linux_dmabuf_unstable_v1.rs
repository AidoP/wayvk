// Auto-Generated file. Do not edit.
#![allow(dead_code)]
#![doc = "# linux_dmabuf_unstable_v1"]
#![doc = ""]
#![doc = "## Copyright"]
#![doc = "Copyright © 2014, 2015 Collabora, Ltd.\n\nPermission is hereby granted, free of charge, to any person obtaining a\ncopy of this software and associated documentation files (the \"Software\"),\nto deal in the Software without restriction, including without limitation\nthe rights to use, copy, modify, merge, publish, distribute, sublicense,\nand/or sell copies of the Software, and to permit persons to whom the\nSoftware is furnished to do so, subject to the following conditions:\n\nThe above copyright notice and this permission notice (including the next\nparagraph) shall be included in all copies or substantial portions of the\nSoftware.\n\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\nIMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\nFITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL\nTHE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\nLIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING\nFROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER\nDEALINGS IN THE SOFTWARE."]
#[doc = "factory for creating dmabuf-based wl_buffers"]
#[doc = ""]
#[doc = "`Version 4`"]
#[doc = ""]
#[doc = "Following the interfaces from:\nhttps://www.khronos.org/registry/egl/extensions/EXT/EGL_EXT_image_dma_buf_import.txt\nhttps://www.khronos.org/registry/EGL/extensions/EXT/EGL_EXT_image_dma_buf_import_modifiers.txt\nand the Linux DRM sub-system's AddFb2 ioctl.\n\nThis interface offers ways to create generic dmabuf-based wl_buffers.\n\nClients can use the get_surface_feedback request to get dmabuf feedback\nfor a particular surface. If the client wants to retrieve feedback not\ntied to a surface, they can use the get_default_feedback request.\n\nThe following are required from clients:\n\n- Clients must ensure that either all data in the dma-buf is\ncoherent for all subsequent read access or that coherency is\ncorrectly handled by the underlying kernel-side dma-buf\nimplementation.\n\n- Don't make any more attachments after sending the buffer to the\ncompositor. Making more attachments later increases the risk of\nthe compositor not being able to use (re-import) an existing\ndmabuf-based wl_buffer.\n\nThe underlying graphics stack must ensure the following:\n\n- The dmabuf file descriptors relayed to the server will stay valid\nfor the whole lifetime of the wl_buffer. This means the server may\nat any time use those fds to import the dmabuf into any kernel\nsub-system that might accept it.\n\nHowever, when the underlying graphics stack fails to deliver the\npromise, because of e.g. a device hot-unplug which raises internal\nerrors, after the wl_buffer has been successfully created the\ncompositor must not raise protocol errors to the client when dmabuf\nimport later fails.\n\nTo create a wl_buffer from one or more dmabufs, a client creates a\nzwp_linux_dmabuf_params_v1 object with a zwp_linux_dmabuf_v1.create_params\nrequest. All planes required by the intended format are added with\nthe 'add' request. Finally, a 'create' or 'create_immed' request is\nissued, which has the following outcome depending on the import success.\n\nThe 'create' request,\n- on success, triggers a 'created' event which provides the final\nwl_buffer to the client.\n- on failure, triggers a 'failed' event to convey that the server\ncannot use the dmabufs received from the client.\n\nFor the 'create_immed' request,\n- on success, the server immediately imports the added dmabufs to\ncreate a wl_buffer. No event is sent from the server in this case.\n- on failure, the server can choose to either:\n- terminate the client by raising a fatal error.\n- mark the wl_buffer as failed, and send a 'failed' event to the\nclient. If the client uses a failed wl_buffer as an argument to any\nrequest, the behaviour is compositor implementation-defined.\n\nFor all DRM formats and unless specified in another protocol extension,\npre-multiplied alpha is used for pixel values.\n\nWarning! The protocol described in this file is experimental and\nbackward incompatible changes may be made. Backward compatible changes\nmay be added together with the corresponding interface version bump.\nBackward incompatible changes are done by bumping the version number in\nthe protocol and interface names and resetting the interface version.\nOnce the protocol is to be declared stable, the 'z' prefix and the\nversion number in the protocol and interface names are removed and the\ninterface version number is reset."]
pub trait r#ZwpLinuxDmabufV1<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "zwp_linux_dmabuf_v1";
    const VERSION: ::core::primitive::u32 = 4u32;
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
                        ::std::concat!("zwp_linux_dmabuf_v1", "@{}.", "destroy", "(", ")"),
                        _this.id(),
                    );
                }
                Self::r#destroy(_this, _event_loop, _client)
            }
            1u16 => {
                let _stream = _client.stream();
                let r#params_id = _stream
                    .object()?
                    .ok_or(::yutani::wire::WlError::NON_NULLABLE)?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "zwp_linux_dmabuf_v1",
                            "@{}.",
                            "create_params",
                            "(",
                            "{:?}",
                            ")"
                        ),
                        _this.id(),
                        r#params_id,
                    );
                }
                Self::r#create_params(_this, _event_loop, _client, r#params_id)
            }
            2u16 => {
                let _stream = _client.stream();
                let r#id = _stream
                    .object()?
                    .ok_or(::yutani::wire::WlError::NON_NULLABLE)?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "zwp_linux_dmabuf_v1",
                            "@{}.",
                            "get_default_feedback",
                            "(",
                            "{:?}",
                            ")"
                        ),
                        _this.id(),
                        r#id,
                    );
                }
                Self::r#get_default_feedback(_this, _event_loop, _client, r#id)
            }
            3u16 => {
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
                            "zwp_linux_dmabuf_v1",
                            "@{}.",
                            "get_surface_feedback",
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
                Self::r#get_surface_feedback(_this, _event_loop, _client, r#id, r#surface)
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
    #[doc = "unbind the factory"]
    #[doc = ""]
    #[doc = "Objects created through this interface, especially wl_buffers, will\nremain valid."]
    fn r#destroy(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "create a temporary object for buffer parameters"]
    #[doc = ""]
    #[doc = "This temporary object is used to collect multiple dmabuf handles into\na single batch to create a wl_buffer. It can only be used once and\nshould be destroyed after a 'created' or 'failed' event has been\nreceived."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`params_id`: the new temporary"]
    fn r#create_params(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#params_id: ::yutani::Id,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "get default feedback"]
    #[doc = ""]
    #[doc = "`Since version 4`"]
    #[doc = ""]
    #[doc = "This request creates a new wp_linux_dmabuf_feedback object not bound\nto a particular surface. This object will deliver feedback about dmabuf\nparameters to use if the client doesn't support per-surface feedback\n(see get_surface_feedback)."]
    fn r#get_default_feedback(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#id: ::yutani::Id,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "get feedback for a surface"]
    #[doc = ""]
    #[doc = "`Since version 4`"]
    #[doc = ""]
    #[doc = "This request creates a new wp_linux_dmabuf_feedback object for the\nspecified wl_surface. This object will deliver feedback about dmabuf\nparameters to use for buffers attached to this surface.\n\nIf the surface is destroyed before the wp_linux_dmabuf_feedback object,\nthe feedback object becomes inert."]
    fn r#get_surface_feedback(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#id: ::yutani::Id,
        r#surface: ::yutani::Id,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "supported buffer format"]
    #[doc = ""]
    #[doc = "This event advertises one buffer format that the server supports.\nAll the supported formats are advertised once when the client\nbinds to this interface. A roundtrip after binding guarantees\nthat the client has received all supported formats.\n\nFor the definition of the format codes, see the\nzwp_linux_buffer_params_v1::create request.\n\nStarting version 4, the format event is deprecated and must not be\nsent by compositors. Instead, use get_default_feedback or\nget_surface_feedback."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`format`: DRM_FORMAT code"]
    fn r#format(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
        r#format: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(
                    " -> ",
                    "zwp_linux_dmabuf_v1",
                    "@{}.",
                    "format",
                    "(",
                    "{:?}",
                    ")"
                ),
                _this.id(),
                r#format,
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 0u16);
        _stream.send_u32(r#format)?;
        _stream.commit(_key)
    }
    #[doc = "supported buffer format modifier"]
    #[doc = ""]
    #[doc = "`Since version 3`"]
    #[doc = ""]
    #[doc = "This event advertises the formats that the server supports, along with\nthe modifiers supported for each format. All the supported modifiers\nfor all the supported formats are advertised once when the client\nbinds to this interface. A roundtrip after binding guarantees that\nthe client has received all supported format-modifier pairs.\n\nFor legacy support, DRM_FORMAT_MOD_INVALID (that is, modifier_hi ==\n0x00ffffff and modifier_lo == 0xffffffff) is allowed in this event.\nIt indicates that the server can support the format with an implicit\nmodifier. When a plane has DRM_FORMAT_MOD_INVALID as its modifier, it\nis as if no explicit modifier is specified. The effective modifier\nwill be derived from the dmabuf.\n\nA compositor that sends valid modifiers and DRM_FORMAT_MOD_INVALID for\na given format supports both explicit modifiers and implicit modifiers.\n\nFor the definition of the format and modifier codes, see the\nzwp_linux_buffer_params_v1::create and zwp_linux_buffer_params_v1::add\nrequests.\n\nStarting version 4, the modifier event is deprecated and must not be\nsent by compositors. Instead, use get_default_feedback or\nget_surface_feedback."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`format`: DRM_FORMAT code"]
    #[doc = "\n`modifier_hi`: high 32 bits of layout modifier"]
    #[doc = "\n`modifier_lo`: low 32 bits of layout modifier"]
    fn r#modifier(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
        r#format: ::core::primitive::u32,
        r#modifier_hi: ::core::primitive::u32,
        r#modifier_lo: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(
                    " -> ",
                    "zwp_linux_dmabuf_v1",
                    "@{}.",
                    "modifier",
                    "(",
                    "{:?}",
                    ", {:?}",
                    ", {:?}",
                    ")"
                ),
                _this.id(),
                r#format,
                r#modifier_hi,
                r#modifier_lo,
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 1u16);
        _stream.send_u32(r#format)?;
        _stream.send_u32(r#modifier_hi)?;
        _stream.send_u32(r#modifier_lo)?;
        _stream.commit(_key)
    }
}
pub mod r#zwp_linux_dmabuf_v1 {}
#[doc = "parameters for creating a dmabuf-based wl_buffer"]
#[doc = ""]
#[doc = "`Version 4`"]
#[doc = ""]
#[doc = "This temporary object is a collection of dmabufs and other\nparameters that together form a single logical buffer. The temporary\nobject may eventually create one wl_buffer unless cancelled by\ndestroying it before requesting 'create'.\n\nSingle-planar formats only require one dmabuf, however\nmulti-planar formats may require more than one dmabuf. For all\nformats, an 'add' request must be called once per plane (even if the\nunderlying dmabuf fd is identical).\n\nYou must use consecutive plane indices ('plane_idx' argument for 'add')\nfrom zero to the number of planes used by the drm_fourcc format code.\nAll planes required by the format must be given exactly once, but can\nbe given in any order. Each plane index can be set only once."]
pub trait r#ZwpLinuxBufferParamsV1<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "zwp_linux_buffer_params_v1";
    const VERSION: ::core::primitive::u32 = 4u32;
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
                        ::std::concat!("zwp_linux_buffer_params_v1", "@{}.", "destroy", "(", ")"),
                        _this.id(),
                    );
                }
                Self::r#destroy(_this, _event_loop, _client)
            }
            1u16 => {
                let _stream = _client.stream();
                let r#fd = _stream.file()?;
                let r#plane_idx = _stream.u32()?;
                let r#offset = _stream.u32()?;
                let r#stride = _stream.u32()?;
                let r#modifier_hi = _stream.u32()?;
                let r#modifier_lo = _stream.u32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "zwp_linux_buffer_params_v1",
                            "@{}.",
                            "add",
                            "(",
                            "{:?}",
                            ", {:?}",
                            ", {:?}",
                            ", {:?}",
                            ", {:?}",
                            ", {:?}",
                            ")"
                        ),
                        _this.id(),
                        r#fd,
                        r#plane_idx,
                        r#offset,
                        r#stride,
                        r#modifier_hi,
                        r#modifier_lo,
                    );
                }
                Self::r#add(
                    _this,
                    _event_loop,
                    _client,
                    r#fd,
                    r#plane_idx,
                    r#offset,
                    r#stride,
                    r#modifier_hi,
                    r#modifier_lo,
                )
            }
            2u16 => {
                let _stream = _client.stream();
                let r#width = _stream.i32()?;
                let r#height = _stream.i32()?;
                let r#format = _stream.u32()?;
                let r#flags = _stream.u32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "zwp_linux_buffer_params_v1",
                            "@{}.",
                            "create",
                            "(",
                            "{:?}",
                            ", {:?}",
                            ", {:?}",
                            ", {:?}",
                            ")"
                        ),
                        _this.id(),
                        r#width,
                        r#height,
                        r#format,
                        r#flags,
                    );
                }
                Self::r#create(
                    _this,
                    _event_loop,
                    _client,
                    r#width,
                    r#height,
                    r#format,
                    r#flags,
                )
            }
            3u16 => {
                let _stream = _client.stream();
                let r#buffer_id = _stream
                    .object()?
                    .ok_or(::yutani::wire::WlError::NON_NULLABLE)?;
                let r#width = _stream.i32()?;
                let r#height = _stream.i32()?;
                let r#format = _stream.u32()?;
                let r#flags = _stream.u32()?;
                #[cfg(debug_assertions)]
                {
                    ::std::println!(
                        ::std::concat!(
                            "zwp_linux_buffer_params_v1",
                            "@{}.",
                            "create_immed",
                            "(",
                            "{:?}",
                            ", {:?}",
                            ", {:?}",
                            ", {:?}",
                            ", {:?}",
                            ")"
                        ),
                        _this.id(),
                        r#buffer_id,
                        r#width,
                        r#height,
                        r#format,
                        r#flags,
                    );
                }
                Self::r#create_immed(
                    _this,
                    _event_loop,
                    _client,
                    r#buffer_id,
                    r#width,
                    r#height,
                    r#format,
                    r#flags,
                )
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
    #[doc = "delete this object, used or not"]
    #[doc = ""]
    #[doc = "Cleans up the temporary data sent to the server for dmabuf-based\nwl_buffer creation."]
    fn r#destroy(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "add a dmabuf to the temporary set"]
    #[doc = ""]
    #[doc = "This request adds one dmabuf to the set in this\nzwp_linux_buffer_params_v1.\n\nThe 64-bit unsigned value combined from modifier_hi and modifier_lo\nis the dmabuf layout modifier. DRM AddFB2 ioctl calls this the\nfb modifier, which is defined in drm_mode.h of Linux UAPI.\nThis is an opaque token. Drivers use this token to express tiling,\ncompression, etc. driver-specific modifications to the base format\ndefined by the DRM fourcc code.\n\nStarting from version 4, the invalid_format protocol error is sent if\nthe format + modifier pair was not advertised as supported.\n\nThis request raises the PLANE_IDX error if plane_idx is too large.\nThe error PLANE_SET is raised if attempting to set a plane that\nwas already set."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`fd`: dmabuf fd"]
    #[doc = "\n`plane_idx`: plane index"]
    #[doc = "\n`offset`: offset in bytes"]
    #[doc = "\n`stride`: stride in bytes"]
    #[doc = "\n`modifier_hi`: high 32 bits of layout modifier"]
    #[doc = "\n`modifier_lo`: low 32 bits of layout modifier"]
    fn r#add(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#fd: ::yutani::File,
        r#plane_idx: ::core::primitive::u32,
        r#offset: ::core::primitive::u32,
        r#stride: ::core::primitive::u32,
        r#modifier_hi: ::core::primitive::u32,
        r#modifier_lo: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "create a wl_buffer from the given dmabufs"]
    #[doc = ""]
    #[doc = "This asks for creation of a wl_buffer from the added dmabuf\nbuffers. The wl_buffer is not created immediately but returned via\nthe 'created' event if the dmabuf sharing succeeds. The sharing\nmay fail at runtime for reasons a client cannot predict, in\nwhich case the 'failed' event is triggered.\n\nThe 'format' argument is a DRM_FORMAT code, as defined by the\nlibdrm's drm_fourcc.h. The Linux kernel's DRM sub-system is the\nauthoritative source on how the format codes should work.\n\nThe 'flags' is a bitfield of the flags defined in enum \"flags\".\n'y_invert' means the that the image needs to be y-flipped.\n\nFlag 'interlaced' means that the frame in the buffer is not\nprogressive as usual, but interlaced. An interlaced buffer as\nsupported here must always contain both top and bottom fields.\nThe top field always begins on the first pixel row. The temporal\nordering between the two fields is top field first, unless\n'bottom_first' is specified. It is undefined whether 'bottom_first'\nis ignored if 'interlaced' is not set.\n\nThis protocol does not convey any information about field rate,\nduration, or timing, other than the relative ordering between the\ntwo fields in one buffer. A compositor may have to estimate the\nintended field rate from the incoming buffer rate. It is undefined\nwhether the time of receiving wl_surface.commit with a new buffer\nattached, applying the wl_surface state, wl_surface.frame callback\ntrigger, presentation, or any other point in the compositor cycle\nis used to measure the frame or field times. There is no support\nfor detecting missed or late frames/fields/buffers either, and\nthere is no support whatsoever for cooperating with interlaced\ncompositor output.\n\nThe composited image quality resulting from the use of interlaced\nbuffers is explicitly undefined. A compositor may use elaborate\nhardware features or software to deinterlace and create progressive\noutput frames from a sequence of interlaced input buffers, or it\nmay produce substandard image quality. However, compositors that\ncannot guarantee reasonable image quality in all cases are recommended\nto just reject all interlaced buffers.\n\nAny argument errors, including non-positive width or height,\nmismatch between the number of planes and the format, bad\nformat, bad offset or stride, may be indicated by fatal protocol\nerrors: INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS,\nOUT_OF_BOUNDS.\n\nDmabuf import errors in the server that are not obvious client\nbugs are returned via the 'failed' event as non-fatal. This\nallows attempting dmabuf sharing and falling back in the client\nif it fails.\n\nThis request can be sent only once in the object's lifetime, after\nwhich the only legal request is destroy. This object should be\ndestroyed after issuing a 'create' request. Attempting to use this\nobject after issuing 'create' raises ALREADY_USED protocol error.\n\nIt is not mandatory to issue 'create'. If a client wants to\ncancel the buffer creation, it can just destroy this object."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`width`: base plane width in pixels"]
    #[doc = "\n`height`: base plane height in pixels"]
    #[doc = "\n`format`: DRM_FORMAT code"]
    #[doc = "\n`flags`: see enum flags"]
    fn r#create(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#width: ::core::primitive::i32,
        r#height: ::core::primitive::i32,
        r#format: ::core::primitive::u32,
        r#flags: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "immediately create a wl_buffer from the given\ndmabufs"]
    #[doc = ""]
    #[doc = "`Since version 2`"]
    #[doc = ""]
    #[doc = "This asks for immediate creation of a wl_buffer by importing the\nadded dmabufs.\n\nIn case of import success, no event is sent from the server, and the\nwl_buffer is ready to be used by the client.\n\nUpon import failure, either of the following may happen, as seen fit\nby the implementation:\n- the client is terminated with one of the following fatal protocol\nerrors:\n- INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS, OUT_OF_BOUNDS,\nin case of argument errors such as mismatch between the number\nof planes and the format, bad format, non-positive width or\nheight, or bad offset or stride.\n- INVALID_WL_BUFFER, in case the cause for failure is unknown or\nplaform specific.\n- the server creates an invalid wl_buffer, marks it as failed and\nsends a 'failed' event to the client. The result of using this\ninvalid wl_buffer as an argument in any request by the client is\ndefined by the compositor implementation.\n\nThis takes the same arguments as a 'create' request, and obeys the\nsame restrictions."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`buffer_id`: id for the newly created wl_buffer"]
    #[doc = "\n`width`: base plane width in pixels"]
    #[doc = "\n`height`: base plane height in pixels"]
    #[doc = "\n`format`: DRM_FORMAT code"]
    #[doc = "\n`flags`: see enum flags"]
    fn r#create_immed(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
        r#buffer_id: ::yutani::Id,
        r#width: ::core::primitive::i32,
        r#height: ::core::primitive::i32,
        r#format: ::core::primitive::u32,
        r#flags: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "buffer creation succeeded"]
    #[doc = ""]
    #[doc = "This event indicates that the attempted buffer creation was\nsuccessful. It provides the new wl_buffer referencing the dmabuf(s).\n\nUpon receiving this event, the client should destroy the\nzlinux_dmabuf_params object."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`buffer`: the newly created wl_buffer"]
    fn r#created(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
        r#buffer: ::yutani::Id,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(
                    " -> ",
                    "zwp_linux_buffer_params_v1",
                    "@{}.",
                    "created",
                    "(",
                    "{:?}",
                    ")"
                ),
                _this.id(),
                r#buffer,
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 0u16);
        _stream.send_object(Some(r#buffer))?;
        _stream.commit(_key)
    }
    #[doc = "buffer creation failed"]
    #[doc = ""]
    #[doc = "This event indicates that the attempted buffer creation has\nfailed. It usually means that one of the dmabuf constraints\nhas not been fulfilled.\n\nUpon receiving this event, the client should destroy the\nzlinux_buffer_params object."]
    fn r#failed(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(
                    " -> ",
                    "zwp_linux_buffer_params_v1",
                    "@{}.",
                    "failed",
                    "(",
                    ")"
                ),
                _this.id(),
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 1u16);
        _stream.commit(_key)
    }
}
pub mod r#zwp_linux_buffer_params_v1 {
    #[doc = ""]
    #[repr(transparent)]
    pub struct r#Error(u32);
    impl r#Error {
        #[doc = "the dmabuf_batch object has already been used to create a wl_buffer"]
        #[doc = ""]
        pub const r#ALREADY_USED: Self = Self(0u32);
        #[doc = "plane index out of bounds"]
        #[doc = ""]
        pub const r#PLANE_IDX: Self = Self(1u32);
        #[doc = "the plane index was already set"]
        #[doc = ""]
        pub const r#PLANE_SET: Self = Self(2u32);
        #[doc = "missing or too many planes to create a buffer"]
        #[doc = ""]
        pub const r#INCOMPLETE: Self = Self(3u32);
        #[doc = "format not supported"]
        #[doc = ""]
        pub const r#INVALID_FORMAT: Self = Self(4u32);
        #[doc = "invalid width or height"]
        #[doc = ""]
        pub const r#INVALID_DIMENSIONS: Self = Self(5u32);
        #[doc = "offset + stride * height goes out of dmabuf bounds"]
        #[doc = ""]
        pub const r#OUT_OF_BOUNDS: Self = Self(6u32);
        #[doc = "invalid wl_buffer resulted from importing dmabufs via\n               the create_immed request on given buffer_params"]
        #[doc = ""]
        pub const r#INVALID_WL_BUFFER: Self = Self(7u32);
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
                0u32 => ::core::write!(f, "{}({})", "ALREADY_USED", 0u32),
                1u32 => ::core::write!(f, "{}({})", "PLANE_IDX", 1u32),
                2u32 => ::core::write!(f, "{}({})", "PLANE_SET", 2u32),
                3u32 => ::core::write!(f, "{}({})", "INCOMPLETE", 3u32),
                4u32 => ::core::write!(f, "{}({})", "INVALID_FORMAT", 4u32),
                5u32 => ::core::write!(f, "{}({})", "INVALID_DIMENSIONS", 5u32),
                6u32 => ::core::write!(f, "{}({})", "OUT_OF_BOUNDS", 6u32),
                7u32 => ::core::write!(f, "{}({})", "INVALID_WL_BUFFER", 7u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
    #[doc = ""]
    #[repr(transparent)]
    pub struct r#Flags(u32);
    impl r#Flags {
        #[doc = "contents are y-inverted"]
        #[doc = ""]
        pub const r#Y_INVERT: Self = Self(1u32);
        #[doc = "content is interlaced"]
        #[doc = ""]
        pub const r#INTERLACED: Self = Self(2u32);
        #[doc = "bottom field first"]
        #[doc = ""]
        pub const r#BOTTOM_FIRST: Self = Self(4u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#Flags {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#Flags {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#Flags {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                1u32 => ::core::write!(f, "{}({})", "Y_INVERT", 1u32),
                2u32 => ::core::write!(f, "{}({})", "INTERLACED", 2u32),
                4u32 => ::core::write!(f, "{}({})", "BOTTOM_FIRST", 4u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}
#[doc = "dmabuf feedback"]
#[doc = ""]
#[doc = "`Version 4`"]
#[doc = ""]
#[doc = "This object advertises dmabuf parameters feedback. This includes the\npreferred devices and the supported formats/modifiers.\n\nThe parameters are sent once when this object is created and whenever they\nchange. The done event is always sent once after all parameters have been\nsent. When a single parameter changes, all parameters are re-sent by the\ncompositor.\n\nCompositors can re-send the parameters when the current client buffer\nallocations are sub-optimal. Compositors should not re-send the\nparameters if re-allocating the buffers would not result in a more optimal\nconfiguration. In particular, compositors should avoid sending the exact\nsame parameters multiple times in a row.\n\nThe tranche_target_device and tranche_modifier events are grouped by\ntranches of preference. For each tranche, a tranche_target_device, one\ntranche_flags and one or more tranche_modifier events are sent, followed\nby a tranche_done event finishing the list. The tranches are sent in\ndescending order of preference. All formats and modifiers in the same\ntranche have the same preference.\n\nTo send parameters, the compositor sends one main_device event, tranches\n(each consisting of one tranche_target_device event, one tranche_flags\nevent, tranche_modifier events and then a tranche_done event), then one\ndone event."]
pub trait r#ZwpLinuxDmabufFeedbackV1<T>: 'static + ::core::marker::Sized {
    const INTERFACE: &'static ::core::primitive::str = "zwp_linux_dmabuf_feedback_v1";
    const VERSION: ::core::primitive::u32 = 4u32;
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
                        ::std::concat!("zwp_linux_dmabuf_feedback_v1", "@{}.", "destroy", "(", ")"),
                        _this.id(),
                    );
                }
                Self::r#destroy(_this, _event_loop, _client)
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
    #[doc = "destroy the feedback object"]
    #[doc = ""]
    #[doc = "Using this request a client can tell the server that it is not going to\nuse the wp_linux_dmabuf_feedback object anymore."]
    fn r#destroy(
        this: ::yutani::lease::Lease<Self>,
        event_loop: &mut ::yutani::wire::EventLoop<T>,
        client: &mut ::yutani::server::Client<T>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>>;
    #[doc = "all feedback has been sent"]
    #[doc = ""]
    #[doc = "This event is sent after all parameters of a wp_linux_dmabuf_feedback\nobject have been sent.\n\nThis allows changes to the wp_linux_dmabuf_feedback parameters to be\nseen as atomic, even if they happen via multiple events."]
    fn r#done(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(
                    " -> ",
                    "zwp_linux_dmabuf_feedback_v1",
                    "@{}.",
                    "done",
                    "(",
                    ")"
                ),
                _this.id(),
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 0u16);
        _stream.commit(_key)
    }
    #[doc = "format and modifier table"]
    #[doc = ""]
    #[doc = "This event provides a file descriptor which can be memory-mapped to\naccess the format and modifier table.\n\nThe table contains a tightly packed array of consecutive format +\nmodifier pairs. Each pair is 16 bytes wide. It contains a format as a\n32-bit unsigned integer, followed by 4 bytes of unused padding, and a\nmodifier as a 64-bit unsigned integer. The native endianness is used.\n\nThe client must map the file descriptor in read-only private mode.\n\nCompositors are not allowed to mutate the table file contents once this\nevent has been sent. Instead, compositors must create a new, separate\ntable file and re-send feedback parameters. Compositors are allowed to\nstore duplicate format + modifier pairs in the table."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`fd`: table file descriptor"]
    #[doc = "\n`size`: table size, in bytes"]
    fn r#format_table(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
        r#fd: ::yutani::Fd<'static>,
        r#size: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(
                    " -> ",
                    "zwp_linux_dmabuf_feedback_v1",
                    "@{}.",
                    "format_table",
                    "(",
                    "{:?}",
                    ", {:?}",
                    ")"
                ),
                _this.id(),
                r#fd,
                r#size,
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 1u16);
        _stream.send_file(r#fd)?;
        _stream.send_u32(r#size)?;
        _stream.commit(_key)
    }
    #[doc = "preferred main device"]
    #[doc = ""]
    #[doc = "This event advertises the main device that the server prefers to use\nwhen direct scan-out to the target device isn't possible. The\nadvertised main device may be different for each\nwp_linux_dmabuf_feedback object, and may change over time.\n\nThere is exactly one main device. The compositor must send at least\none preference tranche with tranche_target_device equal to main_device.\n\nClients need to create buffers that the main device can import and\nread from, otherwise creating the dmabuf wl_buffer will fail (see the\nwp_linux_buffer_params.create and create_immed requests for details).\nThe main device will also likely be kept active by the compositor,\nso clients can use it instead of waking up another device for power\nsavings.\n\nIn general the device is a DRM node. The DRM node type (primary vs.\nrender) is unspecified. Clients must not rely on the compositor sending\na particular node type. Clients cannot check two devices for equality\nby comparing the dev_t value.\n\nIf explicit modifiers are not supported and the client performs buffer\nallocations on a different device than the main device, then the client\nmust force the buffer to have a linear layout."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`device`: device dev_t value"]
    fn r#main_device(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
        r#device: &'_ [::core::primitive::u8],
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(
                    " -> ",
                    "zwp_linux_dmabuf_feedback_v1",
                    "@{}.",
                    "main_device",
                    "(",
                    "{:?}",
                    ")"
                ),
                _this.id(),
                r#device,
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 2u16);
        _stream.send_bytes(r#device)?;
        _stream.commit(_key)
    }
    #[doc = "a preference tranche has been sent"]
    #[doc = ""]
    #[doc = "This event splits tranche_target_device and tranche_modifier events in\npreference tranches. It is sent after a set of tranche_target_device\nand tranche_modifier events; it represents the end of a tranche. The\nnext tranche will have a lower preference."]
    fn r#tranche_done(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(
                    " -> ",
                    "zwp_linux_dmabuf_feedback_v1",
                    "@{}.",
                    "tranche_done",
                    "(",
                    ")"
                ),
                _this.id(),
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 3u16);
        _stream.commit(_key)
    }
    #[doc = "target device"]
    #[doc = ""]
    #[doc = "This event advertises the target device that the server prefers to use\nfor a buffer created given this tranche. The advertised target device\nmay be different for each preference tranche, and may change over time.\n\nThere is exactly one target device per tranche.\n\nThe target device may be a scan-out device, for example if the\ncompositor prefers to directly scan-out a buffer created given this\ntranche. The target device may be a rendering device, for example if\nthe compositor prefers to texture from said buffer.\n\nThe client can use this hint to allocate the buffer in a way that makes\nit accessible from the target device, ideally directly. The buffer must\nstill be accessible from the main device, either through direct import\nor through a potentially more expensive fallback path. If the buffer\ncan't be directly imported from the main device then clients must be\nprepared for the compositor changing the tranche priority or making\nwl_buffer creation fail (see the wp_linux_buffer_params.create and\ncreate_immed requests for details).\n\nIf the device is a DRM node, the DRM node type (primary vs. render) is\nunspecified. Clients must not rely on the compositor sending a\nparticular node type. Clients cannot check two devices for equality by\ncomparing the dev_t value.\n\nThis event is tied to a preference tranche, see the tranche_done event."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`device`: device dev_t value"]
    fn r#tranche_target_device(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
        r#device: &'_ [::core::primitive::u8],
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(
                    " -> ",
                    "zwp_linux_dmabuf_feedback_v1",
                    "@{}.",
                    "tranche_target_device",
                    "(",
                    "{:?}",
                    ")"
                ),
                _this.id(),
                r#device,
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 4u16);
        _stream.send_bytes(r#device)?;
        _stream.commit(_key)
    }
    #[doc = "supported buffer format modifier"]
    #[doc = ""]
    #[doc = "This event advertises the format + modifier combinations that the\ncompositor supports.\n\nIt carries an array of indices, each referring to a format + modifier\npair in the last received format table (see the format_table event).\nEach index is a 16-bit unsigned integer in native endianness.\n\nFor legacy support, DRM_FORMAT_MOD_INVALID is an allowed modifier.\nIt indicates that the server can support the format with an implicit\nmodifier. When a buffer has DRM_FORMAT_MOD_INVALID as its modifier, it\nis as if no explicit modifier is specified. The effective modifier\nwill be derived from the dmabuf.\n\nA compositor that sends valid modifiers and DRM_FORMAT_MOD_INVALID for\na given format supports both explicit modifiers and implicit modifiers.\n\nCompositors must not send duplicate format + modifier pairs within the\nsame tranche or across two different tranches with the same target\ndevice and flags.\n\nThis event is tied to a preference tranche, see the tranche_done event.\n\nFor the definition of the format and modifier codes, see the\nwp_linux_buffer_params.create request."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`indices`: array of 16-bit indexes"]
    fn r#tranche_formats(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
        r#indices: &'_ [::core::primitive::u8],
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(
                    " -> ",
                    "zwp_linux_dmabuf_feedback_v1",
                    "@{}.",
                    "tranche_formats",
                    "(",
                    "{:?}",
                    ")"
                ),
                _this.id(),
                r#indices,
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 5u16);
        _stream.send_bytes(r#indices)?;
        _stream.commit(_key)
    }
    #[doc = "tranche flags"]
    #[doc = ""]
    #[doc = "This event sets tranche-specific flags.\n\nThe scanout flag is a hint that direct scan-out may be attempted by the\ncompositor on the target device if the client appropriately allocates a\nbuffer. How to allocate a buffer that can be scanned out on the target\ndevice is implementation-defined.\n\nThis event is tied to a preference tranche, see the tranche_done event."]
    #[doc = ""]
    #[doc = "## Arguments"]
    #[doc = "\n`flags`: tranche flags"]
    fn r#tranche_flags(
        _this: &mut ::yutani::lease::Lease<Self>,
        _client: &mut ::yutani::server::Client<T>,
        r#flags: ::core::primitive::u32,
    ) -> ::core::result::Result<(), ::yutani::wire::WlError<'static>> {
        #[cfg(debug_assertions)]
        {
            ::std::println!(
                ::std::concat!(
                    " -> ",
                    "zwp_linux_dmabuf_feedback_v1",
                    "@{}.",
                    "tranche_flags",
                    "(",
                    "{:?}",
                    ")"
                ),
                _this.id(),
                r#flags,
            );
        }
        let _stream = _client.stream();
        let _key = _stream.start_message(_this.id(), 6u16);
        _stream.send_u32(r#flags)?;
        _stream.commit(_key)
    }
}
pub mod r#zwp_linux_dmabuf_feedback_v1 {
    #[doc = ""]
    #[repr(transparent)]
    pub struct r#TrancheFlags(u32);
    impl r#TrancheFlags {
        #[doc = "direct scan-out tranche"]
        #[doc = ""]
        pub const r#SCANOUT: Self = Self(1u32);
    }
    impl ::core::convert::From<::core::primitive::u32> for r#TrancheFlags {
        fn from(value: ::core::primitive::u32) -> Self {
            Self(value)
        }
    }
    impl ::core::convert::Into<::core::primitive::u32> for r#TrancheFlags {
        fn into(self) -> ::core::primitive::u32 {
            self.0
        }
    }
    impl ::core::fmt::Debug for r#TrancheFlags {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self.0 {
                1u32 => ::core::write!(f, "{}({})", "SCANOUT", 1u32),
                value => ::core::write!(f, "UNKNOWN({})", value),
            }
        }
    }
}

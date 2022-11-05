# wayvk
A Vulkan-based Wayland compositor for Linux.

# Dependencies
**Most are still in development and their names may be subject to change**
- **syslib** A Linux system call library and libc alternative, because libc sucks.
- **unnamed drm replacement** A C (and legacy API mistake)-free DRM replacement for interfacing with DRI on Linux.
- **yutani & yutani-codegen** Wayland in pure Rust.
- **ash** Vulkan library for Rust.
- **config** Infallible and easy configuration library.
#pragma once

#include <wayland-server-core.h>

typedef struct wl {
	struct wl_display* display;
	struct wl_event_loop* event_loop;

	struct wl_global* global_compositor;
	struct wl_global* global_output;
	struct wl_global* global_shm;
	struct wl_global* global_seat;
	struct wl_global* global_data_device_manager;
	struct wl_global* global_xdg_wm_base;
} Wayland;

Wayland wl_setup(void);
void wl_cleanup(Wayland*);

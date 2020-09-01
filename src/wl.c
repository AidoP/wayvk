#include "wl.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define __USE_POSIX199309
#include <time.h>

#include "util.h"

#include "protocol/wayland.h"
#include "protocol/xdg_shell.h"

// The delay before sending a configure event.
// This should allow for other possible surface events to be retrieved and processed
#define CONFIGURE_DELAY 5

struct rect {
	uint32_t x;
	uint32_t y;
	uint32_t width;
	uint32_t height;
};
struct region {

};
static void destroy_region(struct wl_resource* resource) {
	struct region* region_data = wl_resource_get_user_data(resource);
	if (region_data)
		free(region_data);
}
static void region_destroy(struct wl_client* client, struct wl_resource* resource) {

}
static void region_add(struct wl_client* client, struct wl_resource* resource, int32_t x, int32_t y, int32_t width, int32_t height) {
	TODO
}
static void region_subtract(struct wl_client* client, struct wl_resource* resource, int32_t x, int32_t y, int32_t width, int32_t height) {
	TODO
}
static struct wl_region_interface implement_region = {
	.destroy = region_destroy,
	.add = region_add,
	.subtract = region_subtract
};

struct surface_state {
	struct wl_resource* opaque_region;
	struct wl_resource* input_region;
	uint32_t transform;
	uint32_t scale;
	struct rect damaged;
};
struct surface {
	struct surface_state pending;
	struct surface_state current;
};
static void destroy_surface(struct wl_resource* resource) {
	struct surface* surface_data = wl_resource_get_user_data(resource);
	if (surface_data)
		free(surface_data);
}
static void surface_destroy(struct wl_client* client, struct wl_resource* resource) {
	wl_resource_destroy(resource);
}
static void surface_frame(struct wl_client* client, struct wl_resource* resource, uint32_t callback) {
	struct wl_resource* callback_resource = wl_resource_create(client, &wl_callback_interface, wl_callback_interface.version, callback);

	struct timespec time;
	clock_gettime(CLOCK_MONOTONIC, &time);
	wl_callback_send_done(callback_resource, time.tv_nsec / 1000);
	wl_resource_destroy(callback_resource);
}
static void surface_commit(struct wl_client* client, struct wl_resource* resource) {
	struct surface* surface_data = wl_resource_get_user_data(resource);
	if (!surface_data)
		panic("Surface Data NULL!");
	surface_data->current = surface_data->pending;
}
static const struct wl_surface_interface implement_surface = {
	.destroy = surface_destroy,
	.frame = surface_frame,
	.commit = surface_commit,
};

static void destroy_compositor(struct wl_resource* resource) {
}
static void compositor_create_surface(struct wl_client* client, struct wl_resource* resource, uint32_t id) {
	struct wl_resource* surface_resource = wl_resource_create(client, &wl_surface_interface, wl_surface_interface.version, id);
	struct surface* surface_data = malloc(sizeof(struct surface));
	wl_resource_set_implementation(surface_resource, &implement_surface, surface_data, destroy_surface);
}
static void compositor_create_region(struct wl_client* client, struct wl_resource* resource, uint32_t id) {
	struct wl_resource* region_resource = wl_resource_create(client, &wl_region_interface, wl_region_interface.version, id);
	struct region* region_data = malloc(sizeof(struct region));
	wl_resource_set_implementation(region_resource, &implement_region, region_data, destroy_region);
}
static const struct wl_compositor_interface implement_compositor = {
	.create_surface = compositor_create_surface,
	.create_region = compositor_create_region
};
static void register_compositor(struct wl_client* client, void* data, uint32_t version, uint32_t id) {
	struct wl_resource* resource = wl_resource_create(client, &wl_compositor_interface, wl_compositor_interface.version, id);
	wl_resource_set_implementation(resource, &implement_compositor, NULL, destroy_compositor);
}

static void destroy_output(struct wl_resource* resource) {

}
static void output_release(struct wl_client* client, struct wl_resource* resource) {
	wl_resource_destroy(resource);
}
static const struct wl_output_interface implement_output = {
	.release = output_release
};
static void register_output(struct wl_client* client, void* data, uint32_t version, uint32_t id) {
	struct wl_resource* resource = wl_resource_create(client, &wl_output_interface, wl_output_interface.version, id);
	wl_resource_set_implementation(resource, &implement_output, NULL, destroy_output);

	//TODO
	wl_output_send_geometry(resource, 0, 0, 300, 170, WL_OUTPUT_SUBPIXEL_UNKNOWN, "Unknown", "0x07B5", WL_OUTPUT_TRANSFORM_NORMAL);
	wl_output_send_mode(resource, WL_OUTPUT_MODE_CURRENT, 1366, 768, 59994);
	wl_output_send_scale(resource, 1);
}

static void destroy_keyboard(struct wl_resource* resource) {}
static void keyboard_release(struct wl_client* client, struct wl_resource* resource) {
	wl_resource_destroy(resource);
}
static const struct wl_keyboard_interface implement_keyboard = {
	.release = keyboard_release
};

static void destroy_seat(struct wl_resource* resource) {}
static void seat_get_pointer(struct wl_client* client, struct wl_resource* resource, uint32_t id) {
	TODO
}
static void seat_get_keyboard(struct wl_client* client, struct wl_resource* resource, uint32_t id) {
	struct wl_resource* keyboard_resource = wl_resource_create(client, &wl_keyboard_interface, wl_keyboard_interface.version, id);
	wl_resource_set_implementation(keyboard_resource, &implement_keyboard, NULL, destroy_keyboard);
}
static void seat_get_touch(struct wl_client* client, struct wl_resource* resource, uint32_t id) {
	TODO
}
static void seat_release(struct wl_client* client, struct wl_resource* resource) {
	wl_resource_destroy(resource);
}
static const struct wl_seat_interface implement_seat = {
	.get_pointer = seat_get_pointer,
	.get_keyboard = seat_get_keyboard,
	.get_touch = seat_get_touch,
	.release = seat_release
};
static void register_seat(struct wl_client* client, void* data, uint32_t version, uint32_t id) {
	struct wl_resource* resource = wl_resource_create(client, &wl_seat_interface, wl_seat_interface.version, id);
	wl_resource_set_implementation(resource, &implement_seat, NULL, destroy_seat);
	wl_seat_send_name(resource, "seat0");
	wl_seat_send_capabilities(resource, WL_SEAT_CAPABILITY_KEYBOARD);
}

static void destroy_data_device(struct wl_resource* resource) {}
static void data_device_release(struct wl_client* client, struct wl_resource* resource) {
	wl_resource_destroy(resource);
}
static const struct wl_data_device_interface implement_data_device = {
	.release = data_device_release
};

static void destroy_data_device_manager(struct wl_resource* resource) {}
static void data_device_manager_create_data_source(struct wl_client* client, struct wl_resource* resource, uint32_t id) {

}
static void data_device_manager_get_data_device(struct wl_client* client, struct wl_resource* resource, uint32_t id, struct wl_resource* seat) {
	struct wl_resource* data_device_resource = wl_resource_create(client, &wl_data_device_interface, wl_data_device_interface.version, id);
	wl_resource_set_implementation(data_device_resource, &implement_data_device, NULL, destroy_data_device);
}
static const struct wl_data_device_manager_interface implement_data_device_manager = {
	.create_data_source = data_device_manager_create_data_source,
	.get_data_device = data_device_manager_get_data_device
};
static void register_data_device_manager(struct wl_client* client, void* data, uint32_t version, uint32_t id) {
	struct wl_resource* resource = wl_resource_create(client, &wl_data_device_manager_interface, wl_data_device_manager_interface.version, id);
	wl_resource_set_implementation(resource, &implement_data_device_manager, NULL, destroy_data_device_manager);
};

struct xdg_surface {
	struct wl_array states;

	struct wl_resource* resource;
	struct wl_event_source* configure_timer;
};
struct xdg_toplevel_buffered_state {
	char* title;
	char* appid;
	struct wl_resource* parent;
};
struct xdg_toplevel {
	struct xdg_toplevel_buffered_state pending;
	struct xdg_toplevel_buffered_state current;
	struct wl_resource* xdg_surface;
};
static void destroy_xdg_toplevel(struct wl_resource* resource) {
	struct xdg_toplevel* xdg_toplevel_data = wl_resource_get_user_data(resource);
	printf("Destroying window \"%s\"\n", xdg_toplevel_data->current.title);
	if (xdg_toplevel_data)
		free(xdg_toplevel_data);
	wl_resource_set_user_data(resource, NULL);
}
static void xdg_toplevel_destroy(struct wl_client* client, struct wl_resource* resource) {
	wl_resource_destroy(resource);
}
static void xdg_toplevel_set_parent(struct wl_client* client, struct wl_resource* resource, struct wl_resource* parent) {
	struct xdg_toplevel* xdg_toplevel_data = wl_resource_get_user_data(resource);
	xdg_toplevel_data->pending.parent = parent;
	struct xdg_surface* xdg_surface_data = wl_resource_get_user_data(xdg_toplevel_data->xdg_surface);
	wl_event_source_timer_update(xdg_surface_data->configure_timer, CONFIGURE_DELAY);
}
static void xdg_toplevel_set_title(struct wl_client* client, struct wl_resource* resource, const char* title) {
	struct xdg_toplevel* xdg_toplevel_data = wl_resource_get_user_data(resource);
	if (xdg_toplevel_data->pending.title)
		free(xdg_toplevel_data->pending.title);
	xdg_toplevel_data->pending.title = malloc(strlen(title));
	strcpy(xdg_toplevel_data->pending.title, title);
	struct xdg_surface* xdg_surface_data = wl_resource_get_user_data(xdg_toplevel_data->xdg_surface);
	wl_event_source_timer_update(xdg_surface_data->configure_timer, CONFIGURE_DELAY);
}
static void xdg_toplevel_set_app_id(struct wl_client* client, struct wl_resource* resource, const char* appid) {
	struct xdg_toplevel* xdg_toplevel_data = wl_resource_get_user_data(resource);
	if (xdg_toplevel_data->pending.appid)
		free(xdg_toplevel_data->pending.appid);
	xdg_toplevel_data->pending.appid = malloc(strlen(appid));
	strcpy(xdg_toplevel_data->pending.appid, appid);
	struct xdg_surface* xdg_surface_data = wl_resource_get_user_data(xdg_toplevel_data->xdg_surface);
	wl_event_source_timer_update(xdg_surface_data->configure_timer, CONFIGURE_DELAY);
}
static const struct xdg_toplevel_interface implement_xdg_toplevel = {
	.destroy = xdg_toplevel_destroy,
	.set_parent = xdg_toplevel_set_parent,
	.set_title = xdg_toplevel_set_title,
	.set_app_id = xdg_toplevel_set_app_id,
};

static void destroy_xdg_surface(struct wl_resource* resource) {
	struct xdg_surface* xdg_surface_data = wl_resource_get_user_data(resource);
	if (xdg_surface_data) {
		wl_event_source_remove(xdg_surface_data->configure_timer);
		wl_array_release(&xdg_surface_data->states);
		free(xdg_surface_data);
	}
	wl_resource_set_user_data(resource, NULL);

}
static int xdg_surface_configure_timer(void* user_data) {
	struct xdg_surface* xdg_surface_data = (struct xdg_surface*)user_data;
	printf("Configure\n");
	xdg_toplevel_send_configure(xdg_surface_data->resource, 0, 0, &xdg_surface_data->states);
	return 0;
}
static void xdg_surface_destroy(struct wl_client* client, struct wl_resource* resource) {
	wl_resource_destroy(resource);
}
static void xdg_surface_get_toplevel(struct wl_client* client, struct wl_resource* resource, uint32_t id) {
	struct wl_resource* xdg_toplevel_resource = wl_resource_create(client, &xdg_toplevel_interface, xdg_toplevel_interface.version, id);
	struct xdg_toplevel* xdg_toplevel_data = malloc(sizeof(struct xdg_toplevel));
	struct xdg_toplevel_buffered_state default_state = {};
	xdg_toplevel_data->pending = default_state;
	xdg_toplevel_data->current = default_state;
	wl_resource_set_implementation(xdg_toplevel_resource, &implement_xdg_toplevel, xdg_toplevel_data, destroy_xdg_toplevel);
}
static const struct xdg_surface_interface implement_xdg_surface = {
	.destroy = xdg_surface_destroy,
	.get_toplevel = xdg_surface_get_toplevel,
};

static void destroy_xdg_wm_base(struct wl_resource* resource) {
	
}
static void xdg_wm_base_destroy(struct wl_client* client, struct wl_resource* resource) {
	wl_resource_destroy(resource);
}
static void xdg_wm_base_create_positioner(struct wl_client* client, struct wl_resource* resource, uint32_t id) {
}
static void xdg_wm_base_get_xdg_surface(struct wl_client* client, struct wl_resource* resource, uint32_t id, struct wl_resource* surface) {
	struct wl_resource* xdg_surface_resource = wl_resource_create(client, &xdg_surface_interface, xdg_surface_interface.version, id);
	struct xdg_surface* xdg_surface_data = malloc(sizeof(struct xdg_surface));
	xdg_surface_data->resource = xdg_surface_resource;
	xdg_surface_data->configure_timer = wl_event_loop_add_timer(wl_display_get_event_loop(wl_client_get_display(client)), xdg_surface_configure_timer, xdg_surface_data);
	wl_event_source_timer_update(xdg_surface_data->configure_timer, CONFIGURE_DELAY);
	wl_resource_set_implementation(xdg_surface_resource, &implement_xdg_surface, NULL, destroy_xdg_surface);
}
static void xdg_wm_base_pong(struct wl_client* client, struct wl_resource* resource, uint32_t serial) {
	printf("Client responded to ping for serial %i\n", serial);
}

static const struct xdg_wm_base_interface implement_xdg_wm_base = {
	.destroy = xdg_wm_base_destroy,
	.create_positioner = xdg_wm_base_create_positioner,
	.get_xdg_surface = xdg_wm_base_get_xdg_surface,
	.pong = xdg_wm_base_pong
};
static void register_xdg_wm_base(struct wl_client* client, void* data, uint32_t version, uint32_t id) {
	struct wl_resource* resource = wl_resource_create(client, &xdg_wm_base_interface, xdg_wm_base_interface.version, id);
	wl_resource_set_implementation(resource, &implement_xdg_wm_base, NULL, destroy_xdg_wm_base);
}


void handle_client_created(struct wl_listener* listener, struct wl_client* client) {
	pid_t pid;
	uid_t uid;
	gid_t gid;
	wl_client_get_credentials(client, &pid, &uid, &gid);
	printf("Client Connected: %i\n", pid);

}

struct wl_listener client_created_listener = { .notify = (wl_notify_func_t)handle_client_created };

Wayland wl_setup() {
	Wayland wl;

	wl.display = wl_display_create();
	wl_display_add_socket_auto(wl.display);

	wl.global_compositor = wl_global_create(wl.display, &wl_compositor_interface, wl_compositor_interface.version, NULL, register_compositor);
	wl.global_output = wl_global_create(wl.display, &wl_output_interface, wl_output_interface.version, NULL, register_output);
	wl.global_seat = wl_global_create(wl.display, &wl_seat_interface, wl_seat_interface.version, NULL, register_seat);
	//wl.global_shell = wl_global_create(wl.display, &wl_shell_interface, wl_shell_interface.version, NULL, register_shell);
	//wl.global_surface = wl_global_create(wl.display, &wl_surface_interface, wl_surface_interface.version, NULL, register_surface);
	//wl.global_xdg_surface = wl_global_create(wl.display, &xdg_surface_interface, xdg_surface_interface.version, NULL, register_xdg_surface);
	wl.global_data_device_manager = wl_global_create(wl.display, &wl_data_device_manager_interface, wl_data_device_manager_interface.version, NULL, register_data_device_manager);
	wl.global_xdg_wm_base = wl_global_create(wl.display, &xdg_wm_base_interface, xdg_wm_base_interface.version, NULL, register_xdg_wm_base);

	wl_display_init_shm(wl.display);
	//wl_display_add_shm_format(wl.display, WL_SHM_FORMAT_ARGB8888);
	//wl_display_add_shm_format(wl.display, WL_SHM_FORMAT_XRGB8888);

	wl_display_add_client_created_listener(wl.display, &client_created_listener);
	wl.event_loop = wl_display_get_event_loop(wl.display);

	return wl;
}

void wl_cleanup(Wayland* wl) {
	wl_global_destroy(wl->global_compositor);
	wl_global_destroy(wl->global_output);
	wl_global_destroy(wl->global_seat);
	wl_global_destroy(wl->global_data_device_manager);
	wl_global_destroy(wl->global_xdg_wm_base);

	wl_display_destroy(wl->display);
}


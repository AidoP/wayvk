#include "wl.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define __USE_POSIX199309
#include <time.h>

#include "util.h"

#include "protocol/wayland.h"
#include "protocol/xdg_shell.h"

#include "wl/xdg_wm_base.h"

struct rect {
	uint32_t x;
	uint32_t y;
	uint32_t width;
	uint32_t height;
};
struct region {
	struct wl_array rects;
};
static void destroy_region(struct wl_resource* resource) {
	struct region* region_data = wl_resource_get_user_data(resource);
	if (region_data) {
		wl_array_release(&region_data->rects);
		free(region_data);
	}
}
static void region_destroy(struct wl_client* client, struct wl_resource* resource) {
	wl_resource_destroy(resource);
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
static void surface_set_opaque_region(struct wl_client* client, struct wl_resource* resource, struct wl_resource* region) {
	TODO
}
static void surface_commit(struct wl_client* client, struct wl_resource* resource) {
	struct surface* surface_data = wl_resource_get_user_data(resource);
	if (!surface_data)
		panic("Surface Data NULL!");
	surface_data->current = surface_data->pending;
}
static void surface_set_buffer_scale(struct wl_client* client, struct wl_resource* resource, int32_t scale) {
	TODO
}
static const struct wl_surface_interface implement_surface = {
	.destroy = surface_destroy,
	.frame = surface_frame,
	.set_opaque_region = surface_set_opaque_region,
	.commit = surface_commit,
	.set_buffer_scale = surface_set_buffer_scale
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
	wl_array_init(&region_data->rects);
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

	TODO
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


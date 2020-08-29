#include "wl.h"

#include <stdio.h>
#define __USE_POSIX199309
#include <time.h>

#include "protocol/wayland.h"
#include "protocol/xdg_shell.h"

static void destroy_surface(struct wl_resource* resource) {
	
}
static void surface_frame(struct wl_client* client, struct wl_resource* resource, uint32_t callback) {
	struct wl_resource* callback_resource = wl_resource_create(client, &wl_callback_interface, wl_callback_interface.version, callback);

	struct timespec time;
	clock_gettime(CLOCK_MONOTONIC, &time);
	wl_callback_send_done(callback_resource, time.tv_nsec / 1000);
	wl_resource_destroy(callback_resource);
}
static const struct wl_surface_interface implement_surface = {
	.frame = surface_frame
};

static void destroy_compositor(struct wl_resource* resource) {

}
static void compositor_create_surface(struct wl_client* client, struct wl_resource* resource, uint32_t id) {
	printf("Create Surface\n");
	struct wl_resource* surface_resource = wl_resource_create(client, &wl_surface_interface, wl_surface_interface.version, id);
	wl_resource_set_implementation(surface_resource, &implement_surface, NULL, destroy_surface);
}
static void compositor_create_region(struct wl_client* client, struct wl_resource* resource, uint32_t id) {
	printf("Create Region\n");
}
static const struct wl_compositor_interface implement_compositor = {
	.create_surface = compositor_create_surface,
	.create_region = compositor_create_region
};
static void register_compositor(struct wl_client* client, void* data, uint32_t version, uint32_t id) {
	printf("Register Compositor\n");
	struct wl_resource* resource = wl_resource_create(client, &wl_compositor_interface, wl_compositor_interface.version, id);
	wl_resource_set_implementation(resource, &implement_compositor, NULL, destroy_compositor);
}

static void destroy_keyboard(struct wl_resource* resource) {
	printf("Keyboard Resource Destroyed\n");
}
static void keyboard_release(struct wl_client* client, struct wl_resource* resource) {
	wl_resource_destroy(resource);
}
static const struct wl_keyboard_interface implement_keyboard = {
	.release = keyboard_release
};

static void destroy_seat(struct wl_resource* resource) {
	printf("Seat Destroyed\n");
}
static void seat_get_pointer(struct wl_client* client, struct wl_resource* resource, uint32_t id) {

}
static void seat_get_keyboard(struct wl_client* client, struct wl_resource* resource, uint32_t id) {
	printf("Get Keyboard\n");
	struct wl_resource* keyboard_resource = wl_resource_create(client, &wl_keyboard_interface, wl_keyboard_interface.version, id);
	wl_resource_set_implementation(keyboard_resource, &implement_keyboard, NULL, destroy_keyboard);
}
static void seat_get_touch(struct wl_client* client, struct wl_resource* resource, uint32_t id) {

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
	printf("Register Seat\n");
	struct wl_resource* resource = wl_resource_create(client, &wl_seat_interface, wl_seat_interface.version, id);
	wl_resource_set_implementation(resource, &implement_seat, NULL, destroy_seat);
	wl_seat_send_capabilities(resource, WL_SEAT_CAPABILITY_KEYBOARD);
}

static void destroy_data_device(struct wl_resource* resource) {

}
static void data_device_release(struct wl_client* client, struct wl_resource* resource) {
	wl_resource_destroy(resource);
}
static const struct wl_data_device_interface implement_data_device = {
	.release = data_device_release
};

static void destroy_data_device_manager(struct wl_resource* resource) {

}
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

static void destroy_xdg_toplevel(struct wl_resource* resource) {

}
static void xdg_toplevel_destroy(struct wl_client* client, struct wl_resource* resource) {
	wl_resource_destroy(resource);
}
static const struct xdg_toplevel_interface implement_xdg_toplevel = {
	.destroy = xdg_toplevel_destroy,
};


static void destroy_xdg_surface(struct wl_resource* resource) {

}
static void xdg_surface_destroy(struct wl_client* client, struct wl_resource* resource) {
	wl_resource_destroy(resource);
}
static void xdg_surface_get_toplevel(struct wl_client* client, struct wl_resource* resource, uint32_t id) {
	struct wl_resource* xdg_toplevel_resource = wl_resource_create(client, &xdg_toplevel_interface, xdg_toplevel_interface.version, id);
	wl_resource_set_implementation(xdg_toplevel_resource, &implement_xdg_toplevel, NULL, destroy_xdg_toplevel);
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
	printf("Register XDG WM Base\n");
	struct wl_resource* resource = wl_resource_create(client, &xdg_wm_base_interface, xdg_wm_base_interface.version, id);
	wl_resource_set_implementation(resource, &implement_xdg_wm_base, NULL, destroy_xdg_wm_base);

	xdg_wm_base_send_ping(resource, 1);
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
	wl_global_destroy(wl->global_seat);
	wl_global_destroy(wl->global_data_device_manager);
	wl_global_destroy(wl->global_xdg_wm_base);

	wl_display_destroy(wl->display);
}


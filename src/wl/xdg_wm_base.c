#include "xdg_wm_base.h"

struct xdg_toplevel_staged {
	char* title;
	char* appid;
	struct wl_resource* parent;
};

struct xdg_toplevel {
	struct wl_resource* xdg_surface;
	struct wl_array states;
	struct xdg_toplevel_staged current;
	struct xdg_toplevel_staged pending;
};

struct xdg_surface {
	struct wl_resource* xdg_toplevel;
	struct wl_event_source* configure_timer;
};

static void xdg_toplevel_staged_destroy(struct xdg_toplevel_staged* staged_data) {
	if (staged_data->title)
		free(staged_data->title);
	if (staged_data->appid)
		free(staged_data->appid);
}
static void destroy_xdg_toplevel(struct wl_resource* resource) {
	struct xdg_toplevel* xdg_toplevel_data = wl_resource_get_user_data(resource);
	if (xdg_toplevel_data->current.title)
		printf("Destroying window \"%s\"\n", xdg_toplevel_data->current.title);
	else if (xdg_toplevel_data->pending.title)
		printf("Destroying window \"%s\"\n", xdg_toplevel_data->pending.title);
	else
		printf("Destroying un-named window\n");
	if (xdg_toplevel_data) {
		wl_array_init(&xdg_toplevel_data->states);
		xdg_toplevel_staged_destroy(&xdg_toplevel_data->pending);
		xdg_toplevel_staged_destroy(&xdg_toplevel_data->current);
		free(xdg_toplevel_data);
	}
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
	xdg_toplevel_data->pending.title = malloc(strlen(title) + 1);
	strcpy(xdg_toplevel_data->pending.title, title);
	struct xdg_surface* xdg_surface_data = wl_resource_get_user_data(xdg_toplevel_data->xdg_surface);
	wl_event_source_timer_update(xdg_surface_data->configure_timer, CONFIGURE_DELAY);
}
static void xdg_toplevel_set_app_id(struct wl_client* client, struct wl_resource* resource, const char* appid) {
	struct xdg_toplevel* xdg_toplevel_data = wl_resource_get_user_data(resource);
	if (xdg_toplevel_data->pending.appid)
		free(xdg_toplevel_data->pending.appid);
	xdg_toplevel_data->pending.appid = malloc(strlen(appid) + 1);
	strcpy(xdg_toplevel_data->pending.appid, appid);
	struct xdg_surface* xdg_surface_data = wl_resource_get_user_data(xdg_toplevel_data->xdg_surface);
	wl_event_source_timer_update(xdg_surface_data->configure_timer, CONFIGURE_DELAY);
}
static void xdg_toplevel_set_min_size(struct wl_client* client, struct wl_resource* resource, int32_t width, int32_t height) {
	//TODO
}
static void xdg_toplevel_set_maximized(struct wl_client* client, struct wl_resource* resource) {
	//TODO
}
static const struct xdg_toplevel_interface implement_xdg_toplevel = {
	.destroy = xdg_toplevel_destroy,
	.set_parent = xdg_toplevel_set_parent,
	.set_title = xdg_toplevel_set_title,
	.set_app_id = xdg_toplevel_set_app_id,
	.set_min_size = xdg_toplevel_set_min_size,
	.set_maximized = xdg_toplevel_set_maximized
};

static void destroy_xdg_surface(struct wl_resource* resource) {
	struct xdg_surface* xdg_surface_data = wl_resource_get_user_data(resource);
	if (xdg_surface_data) {
		wl_event_source_remove(xdg_surface_data->configure_timer);
		free(xdg_surface_data);
	}
	wl_resource_set_user_data(resource, NULL);
}

static void xdg_surface_destroy(struct wl_client* client, struct wl_resource* resource) {
	struct xdg_surface* xdg_surface_data = wl_resource_get_user_data(resource);
	if (xdg_surface_data->xdg_toplevel)
		wl_resource_destroy(xdg_surface_data->xdg_toplevel);
	wl_resource_destroy(resource);
}

static void xdg_surface_get_toplevel(struct wl_client* client, struct wl_resource* resource, uint32_t id) {
	struct xdg_surface* xdg_surface_data = wl_resource_get_user_data(resource);
	struct wl_resource* xdg_toplevel_resource = wl_resource_create(client, &xdg_toplevel_interface, xdg_toplevel_interface.version, id);
	xdg_surface_data->xdg_toplevel = xdg_toplevel_resource;
	struct xdg_toplevel* xdg_toplevel_data = malloc(sizeof(struct xdg_toplevel));
	struct xdg_toplevel_staged default_staged = {};
	xdg_toplevel_data->xdg_surface = resource;
	xdg_toplevel_data->pending = default_staged;
	xdg_toplevel_data->current = default_staged;
	wl_array_init(&xdg_toplevel_data->states);
	wl_resource_set_implementation(xdg_toplevel_resource, &implement_xdg_toplevel, xdg_toplevel_data, destroy_xdg_toplevel);
}
static void xdg_surface_ack_configure(struct wl_client* client, struct wl_resource* resource, uint32_t serial) {
	struct xdg_surface* xdg_surface_data = wl_resource_get_user_data(resource);
	if (xdg_surface_data->xdg_toplevel) {
		struct xdg_toplevel* xdg_toplevel_data = wl_resource_get_user_data(xdg_surface_data->xdg_toplevel);
		xdg_toplevel_data->current = xdg_toplevel_data->pending;
	}
}

static int xdg_surface_configure(void* xdg_surface_resource) {
	struct xdg_surface* xdg_surface_data = wl_resource_get_user_data(xdg_surface_resource);
	if (xdg_surface_data->xdg_toplevel) {
		struct xdg_toplevel* xdg_toplevel_data = wl_resource_get_user_data(xdg_surface_data->xdg_toplevel);
		xdg_toplevel_send_configure(xdg_surface_data->xdg_toplevel, 512, 512, &xdg_toplevel_data->states);
	}
	xdg_surface_send_configure(xdg_surface_resource, 0);
	return 0;
}

static const struct xdg_surface_interface implement_xdg_surface = {
	.destroy = xdg_surface_destroy,
	.get_toplevel = xdg_surface_get_toplevel,

	.ack_configure = xdg_surface_ack_configure
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
	xdg_surface_data->configure_timer = wl_event_loop_add_timer(wl_display_get_event_loop(wl_client_get_display(client)), xdg_surface_configure, xdg_surface_resource);
	wl_event_source_timer_update(xdg_surface_data->configure_timer, CONFIGURE_DELAY);
	wl_resource_set_implementation(xdg_surface_resource, &implement_xdg_surface, xdg_surface_data, destroy_xdg_surface);
}
static void xdg_wm_base_pong(struct wl_client* client, struct wl_resource* resource, uint32_t serial) {
	printf("Client responded to ping for serial %i\n", serial);
}

const struct xdg_wm_base_interface implement_xdg_wm_base = {
	.destroy = xdg_wm_base_destroy,
	.create_positioner = xdg_wm_base_create_positioner,
	.get_xdg_surface = xdg_wm_base_get_xdg_surface,
	.pong = xdg_wm_base_pong
};
void register_xdg_wm_base(struct wl_client* client, void* data, uint32_t version, uint32_t id) {
	struct wl_resource* resource = wl_resource_create(client, &xdg_wm_base_interface, xdg_wm_base_interface.version, id);
	wl_resource_set_implementation(resource, &implement_xdg_wm_base, NULL, destroy_xdg_wm_base);
}

/*struct xdg_surface {
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
	printf("Set title to \"%s\"\n", title);
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
static void xdg_toplevel_set_maximized(struct wl_client* client, struct wl_resource* resource) {
	//TODO
}
static const struct xdg_toplevel_interface implement_xdg_toplevel = {
	.destroy = xdg_toplevel_destroy,
	.set_parent = xdg_toplevel_set_parent,
	.set_title = xdg_toplevel_set_title,
	.set_app_id = xdg_toplevel_set_app_id,
	.set_maximized = xdg_toplevel_set_maximized
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

static void xdg_surface_destroy(struct wl_client* client, struct wl_resource* resource) {
	wl_resource_destroy(resource);
}

static void xdg_surface_get_toplevel(struct wl_client* client, struct wl_resource* resource, uint32_t id) {
	struct wl_resource* xdg_toplevel_resource = wl_resource_create(client, &xdg_toplevel_interface, xdg_toplevel_interface.version, id);
	struct xdg_toplevel* xdg_toplevel_data = malloc(sizeof(struct xdg_toplevel));
	struct xdg_toplevel_buffered_state default_state = {};
	xdg_toplevel_data->pending = default_state;
	xdg_toplevel_data->current = default_state;
	xdg_toplevel_data->xdg_surface = resource;
	wl_resource_set_implementation(xdg_toplevel_resource, &implement_xdg_toplevel, xdg_toplevel_data, destroy_xdg_toplevel);
}
static void xdg_surface_ack_configure(struct wl_client* client, struct wl_resource* resource, uint32_t serial) {
	
	//TODO
}

static int xdg_surface_configure_timer(void* user_data) {
	struct xdg_surface* xdg_surface_data = (struct xdg_surface*)user_data;
	xdg_toplevel_send_configure(xdg_surface_data->resource, 0, 0, &xdg_surface_data->states);
	xdg_surface_send_configure();
	return 0;
}

static const struct xdg_surface_interface implement_xdg_surface = {
	.destroy = xdg_surface_destroy,
	.get_toplevel = xdg_surface_get_toplevel,

	.ack_configure = xdg_surface_ack_configure
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
	wl_array_init(&xdg_surface_data->states);
	wl_event_source_timer_update(xdg_surface_data->configure_timer, CONFIGURE_DELAY);
	wl_resource_set_implementation(xdg_surface_resource, &implement_xdg_surface, xdg_surface_data, destroy_xdg_surface);
}
static void xdg_wm_base_pong(struct wl_client* client, struct wl_resource* resource, uint32_t serial) {
	printf("Client responded to ping for serial %i\n", serial);
}

const struct xdg_wm_base_interface implement_xdg_wm_base = {
	.destroy = xdg_wm_base_destroy,
	.create_positioner = xdg_wm_base_create_positioner,
	.get_xdg_surface = xdg_wm_base_get_xdg_surface,
	.pong = xdg_wm_base_pong
};
void register_xdg_wm_base(struct wl_client* client, void* data, uint32_t version, uint32_t id) {
	struct wl_resource* resource = wl_resource_create(client, &xdg_wm_base_interface, xdg_wm_base_interface.version, id);
	wl_resource_set_implementation(resource, &implement_xdg_wm_base, NULL, destroy_xdg_wm_base);
}
*/
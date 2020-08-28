#include <wayland-util.h>
#include <wayland-server.h>
#include <stdio.h>
#include <stdlib.h>

#include "vk.h"

int main(void) {
	Vulkan vk = vk_setup();
	
	struct wl_display* display = wl_display_create();
	wl_display_add_socket_auto(display);
	
	struct wl_event_loop* event_loop = wl_display_get_event_loop(display);

	bool running = true;
	while (running) {
		if (wl_event_loop_dispatch(event_loop, 0))
			running = false;
		wl_display_flush_clients(display);

		vk_draw(&vk);
	}

	wl_display_destroy(display);
	vk_cleanup(&vk);

	return 0;
}

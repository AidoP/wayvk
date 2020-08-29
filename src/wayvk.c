#include <stdio.h>
#include <stdlib.h>

#include "vk.h"
#include "wl.h"


int main(void) {
	// Vulkan vk = vk_setup();
	Wayland wl = wl_setup();

	//wl_display_run(wl.display);

	bool running = true;
	while (running) {
		if (wl_event_loop_dispatch(wl.event_loop, 0))
			running = false;
		wl_display_flush_clients(wl.display);

		// vk_draw(&vk);
	}

	wl_cleanup(&wl);
	// vk_cleanup(&vk);

	return 0;
}

#include <wayland-util.h>
#include <wayland-server.h>
#include <stdio.h>
#include <stdlib.h>

#define DEBUG
#include "vk.h"

int main(void) {
	/*
	uint32_t ext_count = 0;
	vkEnumerateInstanceExtensionProperties(NULL, &ext_count, NULL);
	if (ext_count > 0) {
		VkExtensionProperties* ext_array = malloc(sizeof(VkExtensionProperties) * ext_count);
		vkEnumerateInstanceExtensionProperties(NULL, &ext_count, ext_array);
		
		for (int index = 0; index < ext_count; index++)
			printf("Found Extension: %s\n", ext_array[index].extensionName);

		free(ext_array);
	}
	*/

	Vulkan vk = vk_setup();
	
	vk_draw(&vk);

	vk_cleanup(&vk);

	return 0;

	struct wl_display* display = wl_display_create();
	wl_display_add_socket_auto(display);
	
	wl_display_run(display);
	
	wl_display_destroy(display);

	return 0;
}

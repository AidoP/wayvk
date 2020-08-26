#include <wayland-util.h>
#include <wayland-server.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdnoreturn.h>

#include <vulkan/vulkan.h>

noreturn void panic(char* message) {
	fprintf(stderr, "Panic: %s\n", message);
	exit(1);
}

bool physical_device_suitable(VkPhysicalDevice device) {
	return true;
}

typedef struct {
	VkImage image;
	VkImageView view;
} Image;

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


	VkInstance vk_instance;
	VkPhysicalDevice vk_physical_device = VK_NULL_HANDLE;
	uint32_t vk_queue_family;
	VkQueue vk_queue;
	VkDevice vk_device;
	VkSurfaceKHR vk_surface;
	VkSwapchainKHR vk_swapchain;
	uint32_t vk_swapchain_image_len = 0;
	Image* vk_swapchain_images;

	VkDisplayKHR vk_display;
	VkDisplayPropertiesKHR vk_display_properties;
	uint32_t vk_display_plane;
	uint32_t vk_display_stack;
	VkDisplayModeKHR vk_display_mode;
	VkDisplayModeParametersKHR vk_display_mode_params;

	VkSurfaceCapabilitiesKHR vk_surface_capabilities;
	VkSurfaceFormatKHR vk_surface_format;
	VkPresentModeKHR vk_present_mode = VK_PRESENT_MODE_FIFO_KHR;
	VkExtent2D vk_swapchain_extent;

	const char* vk_instance_extensions[] = {
		"VK_KHR_surface",
		"VK_KHR_display",
	};
	const char* vk_device_extensions[] = {
		"VK_KHR_swapchain"
	};

	VkApplicationInfo vk_appinfo = {
		.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO,
		.pApplicationName = "Wayvk",
		.applicationVersion = VK_MAKE_VERSION(0, 0, 1),
		.pEngineName = "No Engine",
		.engineVersion = VK_MAKE_VERSION(0, 0, 1),
		.apiVersion = VK_API_VERSION_1_0
	};
	VkInstanceCreateInfo vk_instance_info = {
		.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
		.pApplicationInfo = &vk_appinfo,
		.enabledExtensionCount = sizeof(vk_instance_extensions) / sizeof(*vk_instance_extensions),
		.ppEnabledExtensionNames = vk_instance_extensions,
		.enabledLayerCount = 0
	};

	if (vkCreateInstance(&vk_instance_info, NULL, &vk_instance) != VK_SUCCESS)
		panic("Error creating instance");

	uint32_t device_len = 0;
	vkEnumeratePhysicalDevices(vk_instance, &device_len, NULL);
	if (device_len <= 0)
		panic("No Vulkan-compatible physical devices could be found");
	VkPhysicalDevice* devices = malloc(sizeof(VkPhysicalDevice) * device_len);
	vkEnumeratePhysicalDevices(vk_instance, &device_len, devices);
	for (int index = 0; index < device_len; index++) {
		if (physical_device_suitable(devices[index])) {
			vk_physical_device = devices[index];
			break;
		}
	}
	free(devices);

	if (vk_physical_device == VK_NULL_HANDLE)
		panic("No suitable physical devices could be found");

	uint32_t queue_family_len = 0;
	vkGetPhysicalDeviceQueueFamilyProperties(vk_physical_device, &queue_family_len, NULL);
	VkQueueFamilyProperties* queue_family_properties = malloc(sizeof(VkQueueFamilyProperties) * queue_family_len);
	vkGetPhysicalDeviceQueueFamilyProperties(vk_physical_device, &queue_family_len, queue_family_properties);
	for (int index = 0; index < queue_family_len; index++) {
		if (queue_family_properties[index].queueFlags & VK_QUEUE_GRAPHICS_BIT) {
			vk_queue_family = index;
			break;
		}
	}
	free(queue_family_properties);

	float vk_queue_priorities = { 1.0f };
	VkDeviceQueueCreateInfo vk_queue_info = {
		.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
		.queueFamilyIndex = vk_queue_family,
		.queueCount = 1,
		.pQueuePriorities = &vk_queue_priorities
	};
	VkPhysicalDeviceFeatures vk_device_features = { VK_FALSE };

	VkDeviceCreateInfo vk_device_info = {
		.sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
		.queueCreateInfoCount = 1,
		.pQueueCreateInfos = &vk_queue_info,
		.pEnabledFeatures = &vk_device_features,
		.enabledExtensionCount = sizeof(vk_device_extensions) / sizeof(*vk_device_extensions),
		.ppEnabledExtensionNames = vk_device_extensions
	};
	if (vkCreateDevice(vk_physical_device, &vk_device_info, NULL, &vk_device) != VK_SUCCESS)
		panic("Unable to create device");
	vkGetDeviceQueue(vk_device, vk_queue_family, 0, &vk_queue);

	// Get Display info
	uint32_t display_len = 0;
	vkGetPhysicalDeviceDisplayPropertiesKHR(vk_physical_device, &display_len, NULL);
	if (display_len == 0)
		panic("Unable to get a direct display");
	VkDisplayPropertiesKHR* displays = malloc(sizeof(VkDisplayPropertiesKHR) * display_len);
	vkGetPhysicalDeviceDisplayPropertiesKHR(vk_physical_device, &display_len, displays);
	for (int index = 0; index < display_len; index++) {
		vk_display = displays[index].display;
		vk_display_properties = displays[index];
		break;
	}
	free(displays);

	// Get Display Plane Info
	bool display_plane_found = false;
	uint32_t display_properties_len = 0;
	vkGetPhysicalDeviceDisplayPlanePropertiesKHR(vk_physical_device, &display_properties_len, NULL);
	if (display_properties_len == 0)
		panic("No valid raw Vulkan display found");
	VkDisplayPlanePropertiesKHR* display_properties = malloc(sizeof(VkDisplayPlanePropertiesKHR) * display_properties_len);
	vkGetPhysicalDeviceDisplayPlanePropertiesKHR(vk_physical_device, &display_properties_len, display_properties);
	for (int index = 0; index < display_properties_len; index++) {
		if (display_properties[index].currentDisplay == NULL || display_properties[index].currentDisplay == vk_display) {
			vk_display = display_properties[index].currentDisplay;
			vk_display_plane = index;
			vk_display_stack = display_properties[index].currentStackIndex;
			display_plane_found = true;
			break;
		}
	}
	free(display_properties);
	if (!display_plane_found)
		panic("Unable to find a suitable display plane");

	// Get Raw Display Mode Info
	uint32_t display_mode_len = 0;
	vkGetDisplayModePropertiesKHR(vk_physical_device, vk_display, &display_mode_len, NULL);
	if (display_mode_len == 0)
		panic("No valid raw Vulkan display mode found");
	VkDisplayModePropertiesKHR* display_modes = malloc(sizeof(VkDisplayModePropertiesKHR) * display_mode_len);
	vkGetDisplayModePropertiesKHR(vk_physical_device, vk_display, &display_mode_len, display_modes);
	for (int index = 0; index < display_mode_len; index++) {
		vk_display_mode = display_modes[index].displayMode;
		vk_display_mode_params = display_modes[index].parameters;
		break;
	}
	free(display_modes);

	// Create Display Surface
	VkDisplaySurfaceCreateInfoKHR vk_surface_info = {
		.sType = VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR,
		.displayMode = vk_display_mode,
		.planeIndex = vk_display_plane,
		.planeStackIndex = vk_display_stack,
		.transform = VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR,
		.alphaMode = VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR,
		.imageExtent = vk_display_mode_params.visibleRegion
	};

	if (vkCreateDisplayPlaneSurfaceKHR(vk_instance, &vk_surface_info, NULL, &vk_surface) != VK_SUCCESS)
		panic("Unable to create surface");

	vkGetPhysicalDeviceSurfaceCapabilitiesKHR(vk_physical_device, vk_surface, &vk_surface_capabilities);
	
	// Get supported surface formats
	bool found_format = false;
	uint32_t format_len = 0;
	vkGetPhysicalDeviceSurfaceFormatsKHR(vk_physical_device, vk_surface, &format_len, NULL);
	if (format_len == 0)
		panic("No supported surface formats");
	VkSurfaceFormatKHR* formats = malloc(sizeof(VkSurfaceFormatKHR) * format_len);
	vkGetPhysicalDeviceSurfaceFormatsKHR(vk_physical_device, vk_surface, &format_len, formats);
	for (int index = 0; index < format_len; index++) {
			if (formats[index].format == VK_FORMAT_B8G8R8A8_SRGB && formats[index].colorSpace == VK_COLOR_SPACE_SRGB_NONLINEAR_KHR) {
				vk_surface_format = formats[index];
				found_format = true;
				break;
			}
	}
	free(formats);
	if (!found_format)
		panic("Could not find an acceptable surface format");

	uint32_t present_mode_len = 0;
	vkGetPhysicalDeviceSurfacePresentModesKHR(vk_physical_device, vk_surface, &present_mode_len, NULL);
	if (present_mode_len == 0)
		panic("No supported present mode");
	VkPresentModeKHR* present_modes = malloc(sizeof(VkPresentModeKHR) * present_mode_len);
	vkGetPhysicalDeviceSurfacePresentModesKHR(vk_physical_device, vk_surface, &present_mode_len, present_modes);
	for (int index = 0; index < present_mode_len; index++) {
		if (present_modes[index] == VK_PRESENT_MODE_MAILBOX_KHR) {
			vk_present_mode = present_modes[index];
			break;
		}
	}
	free(present_modes);

	if (vk_surface_capabilities.currentExtent.width != UINT32_MAX)
		vk_swapchain_extent = vk_surface_capabilities.currentExtent;
	else
		vk_swapchain_extent = vk_display_mode_params.visibleRegion;

	VkSwapchainCreateInfoKHR vk_swapchain_info = {
		.sType = VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
		.surface = vk_surface,
		.minImageCount = vk_surface_capabilities.minImageCount + vk_surface_capabilities.maxImageCount > vk_surface_capabilities.minImageCount ? 1 : 0,
		.imageFormat = vk_surface_format.format,
		.imageColorSpace = vk_surface_format.colorSpace,
		.imageExtent = vk_swapchain_extent,
		.imageArrayLayers = 1,
		.imageUsage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
		.imageSharingMode = VK_SHARING_MODE_EXCLUSIVE,
		.preTransform = vk_surface_capabilities.currentTransform,
		.compositeAlpha = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
		.presentMode = vk_present_mode,
		.clipped = VK_TRUE,
		.oldSwapchain = VK_NULL_HANDLE
	};

	VkResult result;
	if ((result = vkCreateSwapchainKHR(vk_device, &vk_swapchain_info, NULL, &vk_swapchain)) != VK_SUCCESS)
		panic("Unable to create swapchain\nIs the display already in use by Xorg or a Wayland compositor?");

	// Get the swapchain images
	vkGetSwapchainImagesKHR(vk_device, vk_swapchain, &vk_swapchain_image_len, NULL);
	vk_swapchain_images = malloc(sizeof(Image) * vk_swapchain_image_len);
	VkImage* swapchain_image_buffer = malloc(sizeof(VkImage) * vk_swapchain_image_len);
	vkGetSwapchainImagesKHR(vk_device, vk_swapchain, &vk_swapchain_image_len, swapchain_image_buffer);
	VkImageViewCreateInfo vk_image_view_info = {
		.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
		.viewType = VK_IMAGE_VIEW_TYPE_2D,
		.format = vk_surface_format.format,
		.components = { VK_COMPONENT_SWIZZLE_IDENTITY },
		.subresourceRange = {
			.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT,
			.baseMipLevel = 0,
			.levelCount = 1,
			.baseArrayLayer = 0,
			.layerCount = 1
		}
	};
	for (int index = 0; index < vk_swapchain_image_len; index++) {
		vk_image_view_info.image = vk_swapchain_images[index].image = swapchain_image_buffer[index];
		if (vkCreateImageView(vk_device, &vk_image_view_info, NULL, &vk_swapchain_images[index].view) != VK_SUCCESS)
			panic("Unable to create swapchain image view");
	}
	free(swapchain_image_buffer);

	
	// Clean up
	for (int index = 0; index < vk_swapchain_image_len; index++)
		vkDestroyImageView(vk_device, vk_swapchain_images[index].view, NULL);
	free(vk_swapchain_images);
	vkDestroySwapchainKHR(vk_device, vk_swapchain, NULL);
	vkDestroySurfaceKHR(vk_instance, vk_surface, NULL);
	vkDestroyDevice(vk_device, NULL);
	vkDestroyInstance(vk_instance, NULL);

	return 0;

	struct wl_display* display = wl_display_create();
	wl_display_add_socket_auto(display);
	
	wl_display_run(display);
	
	wl_display_destroy(display);

	return 0;
}

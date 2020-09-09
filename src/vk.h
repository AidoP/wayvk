#pragma once

#include <vulkan/vulkan.h>
#include <stdbool.h>

/// An image with a view
typedef struct {
	VkImage image;
	VkImageView view;
} Image;

typedef struct vk {
	VkInstance instance;
	VkPhysicalDevice physical_device;
	VkPhysicalDeviceMemoryProperties physical_device_memory_properties;
	uint32_t queue_family;
	VkQueue queue;
	VkDevice device;
	VkSurfaceKHR surface;
	VkSwapchainKHR swapchain;
	uint32_t swapchain_image_len;
	Image* swapchain_images;
	VkFramebuffer* framebuffers;
	VkRenderPass renderpass;
	VkCommandPool command_pool;
	VkCommandBuffer* command_buffers;

	VkSemaphore render_semaphore;
	VkSemaphore present_semaphore;

	VkDisplayKHR display;
	VkDisplayPropertiesKHR display_properties;
	uint32_t display_plane;
	uint32_t display_stack;
	VkDisplayModeKHR display_mode;
	VkDisplayModeParametersKHR display_mode_params;

	VkSurfaceCapabilitiesKHR surface_capabilities;
	VkSurfaceFormatKHR surface_format;
	VkPresentModeKHR present_mode;
	VkExtent2D swapchain_extent;
} Vulkan;

Vulkan vk_setup(void);
void vk_cleanup(Vulkan*);

bool load_shader(const char* path, uint8_t** shader_data, size_t* shader_len);
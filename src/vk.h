#pragma once

#include <vulkan/vulkan.h>

/// An image with a view
typedef struct {
	VkImage image;
	VkImageView view;
} Image;

typedef struct vk {
	VkInstance instance;
	VkPhysicalDevice physical_device;
	uint32_t queue_family;
	VkQueue queue;
	VkDevice device;
	VkSurfaceKHR surface;
	VkSwapchainKHR swapchain;
	uint32_t swapchain_image_len;
	Image* swapchain_images;
	VkFramebuffer* framebuffers;
	VkPipelineLayout pipeline_layout;
	VkPipeline pipeline;
	VkRenderPass renderpass;
	VkCommandPool command_pool;
	VkCommandBuffer* command_buffers;

	VkSemaphore render_semaphore;
	VkSemaphore present_semaphore;

	VkShaderModule vert_shader;
	VkShaderModule frag_shader;

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
void vk_draw(Vulkan* vk);

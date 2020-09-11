#pragma once

#include <vulkan/vulkan.h>
#include <stdbool.h>

/// An image with a view
typedef struct {
	VkImage image;
	VkImageView view;
} Image;

#define VK_MAX_INFLIGHT 2

typedef struct vk_inflight {
	VkSemaphore render_semaphore;
	VkSemaphore present_semaphore;
	VkFence fence;
} InFlight;

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

	InFlight inflight[VK_MAX_INFLIGHT];
	uint_fast8_t current_inflight;

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

InFlight vk_inflight_setup(Vulkan*);
void vk_inflight_cleanup(Vulkan*, InFlight*);

uint32_t vk_find_memory_type(Vulkan* vk, uint32_t memory_types, VkMemoryPropertyFlags memory_properties);

bool load_shader(const char* path, uint8_t** shader_data, size_t* shader_len);

struct vk_staging_buffer {
	VkBuffer buffer;
	VkDeviceMemory memory;
	VkMemoryRequirements memory_requirements;
	uint32_t buffer_len;
};

struct vk_glyph {
	VkImage image;
	VkImageView view;
	VkSampler sampler;
	VkDeviceMemory memory;
	VkMemoryRequirements memory_requirements;
};

// Copies data to a buffer in GPU memory
struct vk_staging_buffer vk_staging_buffer_create(Vulkan*, void* data, size_t data_len);
void vk_staging_buffer_destroy(Vulkan*, struct vk_staging_buffer*);
/// Initiates a transfer command buffer for a series of buffer transfers
VkCommandBuffer vk_staging_buffer_start_transfer(Vulkan*);
/// Submits buffer transfers to the queue and waits for completion
void vk_staging_buffer_end_transfer(Vulkan*, VkCommandBuffer);

struct vk_glyph vk_create_glyph(Vulkan*, struct vk_staging_buffer*, VkCommandBuffer, uint32_t width, uint32_t height);
void vk_destroy_glyph(Vulkan*, struct vk_glyph*);
void vk_bind_glyph(Vulkan* vk, struct vk_glyph* glyph, VkDescriptorSet descriptor_set, uint32_t binding);
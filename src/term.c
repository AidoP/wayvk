#include "term.h"
#include "util.h"

#include <stdlib.h>
#include <stdio.h>

struct term_data {
	float colr;
	float colg;
	float colb;
};

static struct session term_setup(void** data, Vulkan* vk) {
	*data = malloc(sizeof(struct term_data));
	struct term_data* term = *data;
	term->colr = (float)(rand() % 1000) / 1000.0;
	term->colg = (float)(rand() % 1000) / 1000.0;
	term->colb = (float)(rand() % 1000) / 1000.0;
    struct session session;

    return session;
}

static void term_cleanup(void* data, struct session* session, Vulkan* vk) {
	//struct term_data* term = data;
	//vkQueueWaitIdle(vk->queue);
	//vkDeviceWaitIdle(vk->device);
	//vkDestroyPipeline(vk->device, session->pipeline, NULL);
	//vkDestroyPipelineLayout(vk->device, session->pipeline_layout, NULL);
	//vkDestroyShaderModule(vk->device, session->vert_shader, NULL);
	//vkDestroyShaderModule(vk->device, session->frag_shader, NULL);
}

static void term_shown(void* data, struct session* session, Vulkan* vk) {

}

static void term_hidden(void* data, struct session* session, Vulkan* vk) {

}

static bool term_update(void* data, struct session* session, Vulkan* vk) {
	struct term_data* term = data;
	vk->current_inflight = (vk->current_inflight + 1) % VK_MAX_INFLIGHT;
	InFlight* inflight = &vk->inflight[vk->current_inflight];

	vkWaitForFences(vk->device, 1, &inflight->fence, VK_TRUE, UINT64_MAX);
	vkResetFences(vk->device, 1, &inflight->fence);

    uint32_t image_index;
	VkResult vk_result = vkAcquireNextImageKHR(vk->device, vk->swapchain, UINT64_MAX, inflight->render_semaphore, VK_NULL_HANDLE, &image_index);
	switch (vk_result) {
		case VK_SUCCESS:
			break;
		case VK_TIMEOUT:
		case VK_NOT_READY:
			return false;
		case VK_SUBOPTIMAL_KHR:
			TODO
		default:
			panic("Unexpected error when acquiring next swapchain image");
	}

	VkCommandBufferBeginInfo vk_command_begin_info = {
		.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO
	};
	if (vkBeginCommandBuffer(vk->command_buffers[image_index], &vk_command_begin_info) != VK_SUCCESS)
		panic("Unable to start command buffer");

	VkClearValue vk_clear_values[] = {
		{ { { term->colr, term->colg, term->colb, 1.0f } } }
	};
	VkRenderPassBeginInfo vk_renderpass_begin_info = {
		.sType = VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
		.renderPass = vk->renderpass,
		.framebuffer = vk->framebuffers[image_index],
		.renderArea = {
			.offset = { 0, 0 },
			.extent = vk->swapchain_extent
		},
		.clearValueCount = 1,
		.pClearValues = vk_clear_values,
	};
	vkCmdBeginRenderPass(vk->command_buffers[image_index], &vk_renderpass_begin_info, VK_SUBPASS_CONTENTS_INLINE);
	vkCmdBindPipeline(vk->command_buffers[image_index], VK_PIPELINE_BIND_POINT_GRAPHICS, vk->glyph_pipeline.pipeline);

	#define strln(string) string, sizeof(string)-1
	ft_draw_string(vk, strln("Hello, World!"), 12.0f, image_index);

	vkCmdEndRenderPass(vk->command_buffers[image_index]);
	if (vkEndCommandBuffer(vk->command_buffers[image_index]) != VK_SUCCESS)
		panic("Unable to complete command buffer");

	VkPipelineStageFlags wait_stages[] = {VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT};
	VkSubmitInfo vk_submit_info = {
		.sType = VK_STRUCTURE_TYPE_SUBMIT_INFO,
		.waitSemaphoreCount = 1,
		.pWaitSemaphores = &inflight->render_semaphore,
		.pWaitDstStageMask = wait_stages,
		.commandBufferCount = 1,
		.pCommandBuffers = &vk->command_buffers[image_index],
		.signalSemaphoreCount = 1,
		.pSignalSemaphores = &inflight->present_semaphore
	};
	if (vkQueueSubmit(vk->queue, 1, &vk_submit_info, inflight->fence) != VK_SUCCESS)
		panic("Unable to submit render queue");

	VkPresentInfoKHR vk_present_info = {
		.sType = VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
		.waitSemaphoreCount = 1,
		.pWaitSemaphores = &inflight->present_semaphore,
		.swapchainCount = 1,
		.pSwapchains = &vk->swapchain,
		.pImageIndices = &image_index
	};
	if (vkQueuePresentKHR(vk->queue, &vk_present_info) != VK_SUCCESS)
		panic("Unable to present the swapchain");

	return false;
}

static void key_event(void* data, struct session* session, uint8_t modifiers, uint32_t key) {

}

const struct session_handler term_session_handler = {
    .setup = term_setup,
    .cleanup = term_cleanup,
    .shown = term_shown,
    .hidden = term_hidden,
    .update = term_update,
	.key_event = key_event
};
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
	((struct term_data*)*data)->colr = (float)(rand() % 1000) / 1000.0;
	((struct term_data*)*data)->colg = (float)(rand() % 1000) / 1000.0;
	((struct term_data*)*data)->colb = (float)(rand() % 1000) / 1000.0;
    struct session session;
	
	// Create the graphics pipeline
	VkPipelineVertexInputStateCreateInfo vk_vertex_input_info = {
		.sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
		.vertexBindingDescriptionCount = 0,
		.vertexAttributeDescriptionCount = 0
	};
	VkPipelineInputAssemblyStateCreateInfo vk_input_assembly_info = {
		.sType = VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
		.topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
		.primitiveRestartEnable = VK_FALSE
	};
	VkViewport vk_viewport = {
		.x = 0.0f,
		.y = 0.0f,
		.width = (float) vk->swapchain_extent.width,
		.height = (float) vk->swapchain_extent.height,
		.minDepth = 0.0f,
		.maxDepth = 1.0f
	};
	VkRect2D vk_scissor = {
		.offset = { 0 },
		.extent = vk->swapchain_extent
	};
	VkPipelineViewportStateCreateInfo vk_viewport_info = {
		.sType = VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
		.viewportCount = 1,
		.pViewports = &vk_viewport,
		.scissorCount = 1,
		.pScissors = &vk_scissor
	};
	VkPipelineRasterizationStateCreateInfo vk_raster_info = {
		.sType = VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
		.depthClampEnable = VK_FALSE,
		.rasterizerDiscardEnable = VK_FALSE,
		.polygonMode = VK_POLYGON_MODE_FILL,
		.lineWidth = 1.0f,
		.cullMode = VK_CULL_MODE_BACK_BIT,
		.frontFace = VK_FRONT_FACE_CLOCKWISE,
		.depthBiasEnable = VK_FALSE,
	};
	VkPipelineMultisampleStateCreateInfo vk_multisample_info = {
		.sType = VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
		.sampleShadingEnable = VK_FALSE,
		.rasterizationSamples = VK_SAMPLE_COUNT_1_BIT
	};
	VkPipelineColorBlendAttachmentState vk_framebuffer_blend_state = {
		.colorWriteMask = VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT | VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT,
		.blendEnable = VK_FALSE,
	};
	VkPipelineColorBlendStateCreateInfo vk_framebuffer_blend_info = {
		.sType = VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
		.logicOpEnable = VK_FALSE,
		.attachmentCount = 1,
		.pAttachments = &vk_framebuffer_blend_state
	};
	VkPipelineLayoutCreateInfo vk_layout_info = {
		.sType = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
	};
	if (vkCreatePipelineLayout(vk->device, &vk_layout_info, NULL, &session.pipeline_layout) != VK_SUCCESS)
		panic("Unable to create pipeline layout");

	// Load shaders
	size_t vert_shader_len;
	uint8_t* vert_shader;
	size_t frag_shader_len;
	uint8_t* frag_shader;

	if (!load_shader("shader/basic.vert.spv", &vert_shader, &vert_shader_len))
		panic("Failed to load vertex shader");
	if (!load_shader("shader/basic.frag.spv", &frag_shader, &frag_shader_len))
		panic("Failed to load fragment shader");

	VkShaderModuleCreateInfo vk_vert_shader_info = {
		.sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
		.codeSize = vert_shader_len,
		.pCode = (uint32_t*)vert_shader
	};
	if (vkCreateShaderModule(vk->device, &vk_vert_shader_info, NULL, &session.vert_shader) != VK_SUCCESS)
		panic("Unable to create vertex shader module");
	VkShaderModuleCreateInfo vk_frag_shader_info = {
		.sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
		.codeSize = frag_shader_len,
		.pCode = (uint32_t*)frag_shader
	};
	if (vkCreateShaderModule(vk->device, &vk_frag_shader_info, NULL, &session.frag_shader) != VK_SUCCESS)
		panic("Unable to create fragment shader module");
	
	VkPipelineShaderStageCreateInfo vk_vert_stage_info = {
		.sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
		.stage = VK_SHADER_STAGE_VERTEX_BIT,
		.module = session.vert_shader,
		.pName = "main"
	};
	VkPipelineShaderStageCreateInfo vk_frag_stage_info = {
		.sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
		.stage = VK_SHADER_STAGE_FRAGMENT_BIT,
		.module = session.frag_shader,
		.pName = "main"
	};
	VkPipelineShaderStageCreateInfo vk_shader_stages[] = {vk_vert_stage_info, vk_frag_stage_info};

	VkGraphicsPipelineCreateInfo vk_pipeline_info = {
		.sType = VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
		.stageCount = 2,
		.pStages = vk_shader_stages,
		.pVertexInputState = &vk_vertex_input_info,
		.pInputAssemblyState = &vk_input_assembly_info,
		.pViewportState = &vk_viewport_info,
		.pRasterizationState = &vk_raster_info,
		.pMultisampleState = &vk_multisample_info,
		.pDepthStencilState = NULL,
		.pColorBlendState = &vk_framebuffer_blend_info,
		.pDynamicState = NULL,
		.layout = session.pipeline_layout,
		.renderPass = vk->renderpass,
		.subpass = 0,
		.basePipelineHandle = VK_NULL_HANDLE,
		.basePipelineIndex = -1,
	};
	if (vkCreateGraphicsPipelines(vk->device, VK_NULL_HANDLE, 1, &vk_pipeline_info, NULL, &session.pipeline) != VK_SUCCESS)
		panic("Unable to create graphics pipeline");

    return session;
}

static void term_cleanup(void* data, struct session* session, Vulkan* vk) {
	vkQueueWaitIdle(vk->queue);
	vkDeviceWaitIdle(vk->device);
	vkDestroyPipeline(vk->device, session->pipeline, NULL);
	vkDestroyPipelineLayout(vk->device, session->pipeline_layout, NULL);
	vkDestroyShaderModule(vk->device, session->vert_shader, NULL);
	vkDestroyShaderModule(vk->device, session->frag_shader, NULL);
}

static void term_shown(void* data, struct session* session, Vulkan* vk) {

}

static void term_hidden(void* data, struct session* session, Vulkan* vk) {

}

static void term_update(void* data, struct session* session, Vulkan* vk) {
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
			panic("Not Ready");
			return;
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
		{ { { ((struct term_data*)data)->colr, ((struct term_data*)data)->colg, ((struct term_data*)data)->colb, 1.0f } } }
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
	vkCmdBindPipeline(vk->command_buffers[image_index], VK_PIPELINE_BIND_POINT_GRAPHICS, session->pipeline);
	vkCmdDraw(vk->command_buffers[image_index], 3, 1, 0, 0);
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
}

const struct session_handler term_session_handler = {
    .setup = term_setup,
    .cleanup = term_cleanup,
    .shown = term_shown,
    .hidden = term_hidden,
    .update = term_update
};
#include "term.h"
#include "util.h"
#include "font.h"

#include <stdlib.h>
#include <stdio.h>

struct term_data {
	float colr;
	float colg;
	float colb;

	VkDescriptorSet* glyph_descriptor_sets;
	VkDescriptorSetLayout glyph_descriptor_set_layout;
	VkDescriptorPool glyph_descriptor_pool;
};

static struct session term_setup(void** data, Vulkan* vk) {
	*data = malloc(sizeof(struct term_data));
	struct term_data* term = *data;
	term->colr = (float)(rand() % 1000) / 1000.0;
	term->colg = (float)(rand() % 1000) / 1000.0;
	term->colb = (float)(rand() % 1000) / 1000.0;
    struct session session;

	// Create descriptors
	VkDescriptorSetLayoutBinding vk_glyph_sampler_binding = {
		.binding = 0,
		.descriptorCount = 1,
		.descriptorType = VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
		.pImmutableSamplers = NULL,
		.stageFlags = VK_SHADER_STAGE_FRAGMENT_BIT
	};
	VkDescriptorSetLayoutCreateInfo vk_glyph_descriptor_set_info = {
		.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
		.bindingCount = 1,
		.pBindings = &vk_glyph_sampler_binding,
	};
	if (vkCreateDescriptorSetLayout(vk->device, &vk_glyph_descriptor_set_info, NULL, &term->glyph_descriptor_set_layout) != VK_SUCCESS)
		panic("Unable to create glyph descriptor set layout");

	VkDescriptorPoolSize vk_glyph_sampler_pool_size = {
		.type = VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
		.descriptorCount = vk->swapchain_image_len
	};
	VkDescriptorPoolCreateInfo vk_glyph_descriptor_pool_info = {
		.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
		.poolSizeCount = 1,
		.pPoolSizes = &vk_glyph_sampler_pool_size,
		.maxSets = vk->swapchain_image_len
	};
	if (vkCreateDescriptorPool(vk->device, &vk_glyph_descriptor_pool_info, NULL, &term->glyph_descriptor_pool) != VK_SUCCESS)
		panic("Unable to create descriptor pool");
	VkDescriptorSetLayout* vk_glyph_pool_layouts = malloc(sizeof(VkDescriptorSetLayout) * vk->swapchain_image_len);
	for (size_t index = 0; index < vk->swapchain_image_len; index++)
		vk_glyph_pool_layouts[index] = term->glyph_descriptor_set_layout;
	VkDescriptorSetAllocateInfo vk_glyph_descriptor_sets_info = {
		.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
		.descriptorPool = term->glyph_descriptor_pool,
		.descriptorSetCount = vk->swapchain_image_len,
		.pSetLayouts = vk_glyph_pool_layouts
	};
	term->glyph_descriptor_sets = malloc(sizeof(VkDescriptorSet) * vk->swapchain_image_len);
	if (vkAllocateDescriptorSets(vk->device, &vk_glyph_descriptor_sets_info, term->glyph_descriptor_sets) != VK_SUCCESS)
		panic("Unable to allocate glyph descriptor sets");
	
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
		.setLayoutCount = 1,
		.pSetLayouts = &term->glyph_descriptor_set_layout,
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
	struct term_data* term = data;
	vkQueueWaitIdle(vk->queue);
	vkDeviceWaitIdle(vk->device);
	vkDestroyPipeline(vk->device, session->pipeline, NULL);
	vkDestroyPipelineLayout(vk->device, session->pipeline_layout, NULL);
	free(term->glyph_descriptor_sets);
	vkDestroyDescriptorPool(vk->device, term->glyph_descriptor_pool, NULL);
	vkDestroyDescriptorSetLayout(vk->device, term->glyph_descriptor_set_layout, NULL);
	vkDestroyShaderModule(vk->device, session->vert_shader, NULL);
	vkDestroyShaderModule(vk->device, session->frag_shader, NULL);
}

static void term_shown(void* data, struct session* session, Vulkan* vk) {

}

static void term_hidden(void* data, struct session* session, Vulkan* vk) {

}

static void term_update(void* data, struct session* session, Vulkan* vk, Font* ft) {
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

	// Bind texture
	vk_bind_glyph(vk, ft_get_character(ft, '$'), term->glyph_descriptor_sets[image_index], 0);
	vkCmdBindDescriptorSets(vk->command_buffers[image_index], VK_PIPELINE_BIND_POINT_GRAPHICS, session->pipeline_layout, 0, 1, &term->glyph_descriptor_sets[image_index], 0, NULL);

	vkCmdDraw(vk->command_buffers[image_index], 6, 1, 0, 0);
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
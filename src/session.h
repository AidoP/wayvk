#pragma once

#include "vk.h"
#include "stdbool.h"

struct session {
	VkPipelineLayout pipeline_layout;
	VkPipeline pipeline;

	VkShaderModule vert_shader;
	VkShaderModule frag_shader;
};

typedef struct session (*fn_session_setup)(void* data, Vulkan*);
typedef void (*fn_session_cleanup)(void* data, struct session*, Vulkan*);
typedef void (*fn_session_shown)(void* data, struct session*, Vulkan*);
typedef void (*fn_session_hidden)(void* data, struct session*, Vulkan*);

typedef void (*fn_session_update)(void* data, struct session*, Vulkan*);

typedef struct session_handler {
    struct session session;
    void* data;
    fn_session_setup setup;
    fn_session_cleanup cleanup;
    fn_session_shown shown;
    fn_session_hidden hidden;
    fn_session_update update;
} SessionHandler;

static inline void session_setup(Vulkan* vk, SessionHandler* handler) {
    handler->session = handler->setup(handler->data, vk);
}

static inline void session_cleanup(Vulkan* vk, SessionHandler* handler) {
    handler->cleanup(handler->data, &handler->session, vk);
}
static inline void session_update(Vulkan* vk, SessionHandler* handler) {
    handler->update(handler->data, &handler->session, vk);
}
#pragma once

#include "vk.h"
#include "font.h"
#include "stdbool.h"

struct session {};

typedef struct session (*fn_session_setup)(void** data, Vulkan*);
typedef void (*fn_session_cleanup)(void* data, struct session*, Vulkan*);
typedef void (*fn_session_shown)(void* data, struct session*, Vulkan*);
typedef void (*fn_session_hidden)(void* data, struct session*, Vulkan*);

typedef void (*fn_session_update)(void* data, struct session*, Vulkan*);
typedef void (*fn_session_key_event)(void* data, struct session*, uint8_t modifiers, uint32_t key);

typedef struct session_handler {
    struct session session;
    void* data;
    fn_session_setup setup;
    fn_session_cleanup cleanup;
    fn_session_shown shown;
    fn_session_hidden hidden;
    fn_session_update update;
    fn_session_key_event key_event;
} SessionHandler;

static inline void session_setup(Vulkan* vk, SessionHandler* handler) {
    handler->session = handler->setup(&handler->data, vk);
}

static inline void session_cleanup(Vulkan* vk, SessionHandler* handler) {
    handler->cleanup(handler->data, &handler->session, vk);
}
static inline void session_update(Vulkan* vk, SessionHandler* handler) {
    handler->update(handler->data, &handler->session, vk);
}
static inline void session_key_event(SessionHandler* handler, uint8_t modifiers, uint32_t key) {
    handler->key_event(handler->data, &handler->session, modifiers, key);
}
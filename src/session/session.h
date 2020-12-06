#pragma once

#include "../vk.h"
#include "../font.h"
#include <stdbool.h>
#include <pthread.h>

struct session_event_key {
    uint32_t key;
    uint8_t modifiers;
};

typedef void (*fn_session_setup)(void** data, Vulkan*);
typedef void (*fn_session_cleanup)(void* data, Vulkan*);
typedef void (*fn_session_shown)(void* data, Vulkan*);
typedef void (*fn_session_hidden)(void* data, Vulkan*);

typedef void (*fn_session_update)(void* data, Vulkan*);
typedef void (*fn_session_background_update)(void* data);
typedef void (*fn_session_key_event)(void* data, Vulkan*, struct session_event_key*);
typedef void (*fn_session_generic)(void* data, Vulkan*, void* args);

struct session {
    fn_session_setup setup;
    fn_session_cleanup cleanup;
    fn_session_shown shown;
    fn_session_hidden hidden;
    fn_session_update update;
    fn_session_background_update background_update;
    fn_session_key_event key_event;
};
typedef struct session_handler {
    pthread_t thread_id;
    pthread_barrier_t barrier;
    pthread_mutex_t mutex;
    Vulkan* vk;
    void* data;
    const struct session* session;
    /// The session function to call within the session thread
    fn_session_generic function;
    void* args;
} SessionHandler;

struct session_handler* session_setup(Vulkan* vk, const struct session*);
void session_cleanup(SessionHandler* handler);

enum session_function {
    SESSION_FUNCTION_SHOWN = 2,
    SESSION_FUNCTION_HIDDEN = 3,
    SESSION_FUNCTION_UPDATE = 4,
    SESSION_FUNCTION_BACKGROUND_UPDATE= 5,
    SESSION_FUNCTION_KEY_EVENT = 6
};
void session_execute(SessionHandler* handler, fn_session_generic function, void* args);
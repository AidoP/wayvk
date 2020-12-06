#include "session.h"
#include <stdio.h>

void* session_thread_main(void* args) {
    SessionHandler* handler = (SessionHandler*)args;
    handler->session->setup(&handler->data, handler->vk);

    while (true) {
        pthread_barrier_wait(&handler->barrier);

        pthread_mutex_lock(&handler->mutex);
        pthread_mutex_lock(&handler->vk->mutex);
        handler->function(&handler->data, handler->vk, handler->args);
        pthread_mutex_unlock(&handler->mutex);
        pthread_mutex_unlock(&handler->vk->mutex);

        if (handler->session == NULL) {
            pthread_barrier_wait(&handler->barrier);
            return NULL;
        }
    }
}

struct session_handler* session_setup(Vulkan* vk, const struct session* session) {
    struct session_handler* handler = malloc(sizeof(struct session_handler));
    handler->vk = vk;
    handler->session = session;
    pthread_barrier_init(&handler->barrier, NULL, 2);
    pthread_mutex_init(&handler->mutex, NULL);
    pthread_create(&handler->thread_id, NULL, session_thread_main, handler);

    return handler;
}

void session_cleanup(SessionHandler* handler) {
    pthread_mutex_lock(&handler->mutex);
    handler->function = (fn_session_generic)handler->session->cleanup;
    pthread_mutex_unlock(&handler->mutex);

    handler->session = NULL;
    pthread_barrier_wait(&handler->barrier);
    pthread_barrier_wait(&handler->barrier);

    pthread_join(handler->thread_id, NULL);
    pthread_barrier_destroy(&handler->barrier);
    free(handler);
}

void session_execute(SessionHandler* handler, fn_session_generic function, void* args) {
    pthread_mutex_lock(&handler->mutex);
    handler->function = function;
    handler->args = args;
    pthread_mutex_unlock(&handler->mutex);

    pthread_barrier_wait(&handler->barrier);
}
#include "session.h"
#include <stdio.h>

void* session_thread_main(void* args) {
    SessionHandler* handler = (SessionHandler*)args;
    handler->session->setup(&handler->data, handler->vk);

    while (true) {
        // Await next command
        pthread_barrier_wait(&handler->barrier);
        // Allow the main thread to modify handler without a data race on the mutex
        pthread_barrier_wait(&handler->barrier);

        // TODO: mutex should not be needed with the barrier gate, assert this
        pthread_mutex_lock(&handler->mutex);
        
        pthread_mutex_lock(&handler->vk->mutex);
        handler->function(handler->data, handler->vk, handler->args);
        pthread_mutex_unlock(&handler->vk->mutex);

        if (handler->session == NULL) {
            return NULL;
        }
        pthread_mutex_unlock(&handler->mutex);
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
    pthread_barrier_wait(&handler->barrier);
    pthread_mutex_lock(&handler->mutex);
    handler->function = (fn_session_generic)handler->session->cleanup;

    // When behind the mutex and/or only one barrier, which should be more than enough,
    // this change is not observed in the waiting thread...
    handler->session = NULL;
    pthread_mutex_unlock(&handler->mutex);
    pthread_barrier_wait(&handler->barrier);

    pthread_join(handler->thread_id, NULL);
    pthread_barrier_destroy(&handler->barrier);
    free(handler);
}

void session_execute(SessionHandler* handler, fn_session_generic function, void* args) {
    pthread_barrier_wait(&handler->barrier);
    pthread_mutex_lock(&handler->mutex);
    handler->function = function;
    handler->args = args;
    pthread_mutex_unlock(&handler->mutex);

    pthread_barrier_wait(&handler->barrier);
}
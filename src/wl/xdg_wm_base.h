#pragma once
#include "proto.h"

const struct xdg_wm_base_interface implement_xdg_wm_base;
void register_xdg_wm_base(struct wl_client* client, void* data, uint32_t version, uint32_t id);
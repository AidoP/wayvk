#pragma once
#include <wayland-server.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../util.h"

#include "../protocol/wayland.h"
#include "../protocol/xdg_shell.h"


// The delay before sending a configure event.
// Applies to xdg_surface and wl_surface
// This should allow for other possible surface events to be retrieved and processed
#define CONFIGURE_DELAY 1
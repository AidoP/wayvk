#pragma once

#include <stdint.h>
#include <stdbool.h>
#include <stdnoreturn.h>

noreturn void panic(char* message);

#define TODO panic("TODO - Unimplemented");

struct rectangle {
	uint32_t width;
	uint32_t height;
} Rectangle;
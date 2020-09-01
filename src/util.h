#pragma once

#include <stdnoreturn.h>

noreturn void panic(char* message);

#define TODO panic("TODO - Unimplemented");

#include "util.h"

#include <stdio.h>
#include <stdlib.h>

noreturn void panic(char* message) {
	fprintf(stderr, "Panic: %s\n", message);
	exit(1);
}
#!/bin/sh

for shader in shader/*.frag shader/*.vert; do
	glslc "$shader" -o "${shader}.spv"
done
gcc -std=c11 -Wall -g src/*.c -lwayland-server -lvulkan -o target/wayvk

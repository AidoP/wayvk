#!/bin/sh

for shader in shader/*.frag shader/*.vert; do
	glslc "$shader" -o "${shader}.spv"
done

proto_wl='/usr/share/wayland/wayland.xml'
proto_xdg_shell='/usr/share/wayland-protocols/stable/xdg-shell/xdg-shell.xml'
wayland-scanner private-code $proto_wl src/protocol/wayland.c
wayland-scanner server-header $proto_wl src/protocol/wayland.h

wayland-scanner private-code $proto_xdg_shell src/protocol/xdg_shell.c
wayland-scanner server-header $proto_xdg_shell src/protocol/xdg_shell.h

gcc -std=c11 -Wall -Werror -g src/*.c src/protocol/*.c src/wl/*.c -lwayland-server -lvulkan -ludev -linput -o target/wayvk

#!/bin/sh

gcc -std=c11 wayvk.c -lwayland-server -lvulkan -o target/wayvk

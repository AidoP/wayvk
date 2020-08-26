#!/bin/sh

gcc -std=c11 -Wall -g wayvk.c -lwayland-server -lvulkan -o target/wayvk

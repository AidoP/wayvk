#pragma once

#include <stdint.h>
#include <stdlib.h>

struct glyphs;
struct font;
struct session;
typedef struct vk Vulkan;

typedef struct ft {
    struct glyphs* glyphs;
    struct font* font;
} Font;

Font ft_load(char* path, float size);
void ft_unload(Font, Vulkan*);
void ft_raster(Font*, Vulkan*, float size);

size_t ft_glyph_count(Font*);
void ft_draw_string(Vulkan* vk, const char* string, size_t string_len, float size, uint32_t image_index);
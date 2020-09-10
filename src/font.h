#pragma once
#include "vk.h"

struct glyphs;
struct font;

typedef struct ft {
    struct glyphs* glyphs;
    struct font* font;
} Font;

Font ft_load(Vulkan*);
void ft_unload(Font, Vulkan*);

void ft_get_character(Font*, char);
void ft_layout();
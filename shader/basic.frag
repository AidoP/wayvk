#version 450
layout(location = 0) out vec4 colour;
layout(location = 0) in vec2 tex_coord;

layout(binding = 0) uniform sampler2D glyph_sampler;

void main() {
	vec3 text_colour = vec3(1.0);
	colour = vec4(text_colour, texture(glyph_sampler, tex_coord).r);
}

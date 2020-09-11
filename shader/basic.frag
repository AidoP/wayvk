#version 450
layout(location = 0) out vec4 colour;
layout(location = 0) in vec2 tex_coord;

layout(binding = 0) uniform sampler2D glyph_sampler;

void main() {
	float font = texture(glyph_sampler, tex_coord).r;
	colour = vec4(font, font, font, 1.0);
}

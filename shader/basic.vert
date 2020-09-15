#version 450

vec2 positions[6] = vec2[](
	vec2(0.0, 0.0),
	vec2(1.0, 0.0),
	vec2(0.0, 1.0),
	vec2(1.0, 0.0),
	vec2(1.0, 1.0),
	vec2(0.0, 1.0)
);

vec2 tex_coords[6] = vec2[](
	vec2(0.0, 0.0),
	vec2(1.0, 0.0),
	vec2(0.0, 1.0),
	vec2(1.0, 0.0),
	vec2(1.0, 1.0),
	vec2(0.0, 1.0)
);

layout(location = 0) out vec2 tex_coord;

layout(push_constant) uniform push_constant {
	float x;
	float y;
	float width;
	float height;
} glyph_position;

void main() {
	tex_coord = tex_coords[gl_VertexIndex];
	float x = positions[gl_VertexIndex].x * glyph_position.width + glyph_position.x;
	float y = (positions[gl_VertexIndex].y * glyph_position.height) - (glyph_position.y + glyph_position.height);
	gl_Position = vec4(2.0 * vec2(x / 1366.0, y / 768.0) - 1.0, 0.0, 1.0);
}

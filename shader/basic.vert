#version 450

vec2 positions[6] = vec2[](
	vec2(-0.5, -0.5),
	vec2(0.5, -0.5),
	vec2(-0.5, 0.5),
	vec2(0.5, -0.5),
	vec2(0.5, 0.5),
	vec2(-0.5, 0.5)
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

void main() {
	tex_coord = tex_coords[gl_VertexIndex];
	gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
}

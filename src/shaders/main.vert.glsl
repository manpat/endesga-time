#version 450

layout(std140, binding = 0) uniform UniformData {
	mat4 u_projection_view;
};


struct Vertex {
	vec3 position;
	vec3 color;
};

layout(std430, binding = 0) buffer VertexData {
	Vertex[] vertices;
};

out vec4 v_color;


void main() {
	Vertex vertex = vertices[gl_VertexID];

	gl_Position = u_projection_view * vec4(vertex.position, 1.0);
	v_color = vec4(vertex.color, 1.0);
}
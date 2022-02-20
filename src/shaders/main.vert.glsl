#version 450

layout(std140, binding = 0) uniform UniformData {
	mat4 u_projection_view;
};

// layout(location=0) in vec3 a_pos;
// layout(location=1) in vec4 a_color;

out vec4 v_color;



const vec3 vertices[] = {
	vec3( 0.0, 1.0, 0.0),
	vec3( 1.0,-0.7, 0.0),
	vec3(-1.0,-0.7, 0.0),
};


const vec3 colors[] = {
	vec3( 1.0, 0.5, 0.5),
	vec3( 0.5, 1.0, 0.5),
	vec3( 0.5, 0.5, 1.0),
};



void main() {
	// gl_Position = u_projection_view * vec4(a_pos, 1.0);
	// v_color = a_color;

	gl_Position = u_projection_view * vec4(vertices[gl_VertexID], 1.0);
	v_color = vec4(colors[gl_VertexID], 1.0);
}
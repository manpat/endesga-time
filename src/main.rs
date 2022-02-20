pub mod gl {
	include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub mod prelude;
pub mod window;
pub mod shader;
pub mod buffer;

use prelude::*;
use window::Window;
use shader::ShaderProgram;
use buffer::Buffer;


fn main() -> Result<(), Box<dyn Error>> {
	let sdl_ctx = sdl2::init()?;
	let sdl_video = sdl_ctx.video()?;
	let mut event_pump = sdl_ctx.event_pump()?;

	let window = Window::new(&sdl_video)?;
	window.on_resize();


	let main_shader = ShaderProgram::new_simple(
		include_str!("shaders/main.vert.glsl"),
		include_str!("shaders/main.frag.glsl"),
	)?;


	// Create and bind a dummy vao - required to use gl::DrawArrays
	unsafe {
		let mut vao = 0;
		gl::CreateVertexArrays(1, &mut vao);
		gl::BindVertexArray(vao);
	}

	let triangle_vertex_buffer = build_triangle_buffer();
	let mut uniform_buffer = Buffer::new();

	// Here's where all our complex game state goes
	let mut time = 0.0f32;

	'main_loop: loop {
		for event in event_pump.poll_iter() {
			use sdl2::event::{Event, WindowEvent};
			use sdl2::keyboard::Scancode;

			match event {
				Event::Quit {..} => { break 'main_loop }
				Event::Window{ win_event: WindowEvent::Resized(..), .. } => {
					window.on_resize();
				}

				Event::KeyDown { scancode: Some(Scancode::Escape), .. } => {
					break 'main_loop
				},

				_ => {},
			}
		}

		time += 1.0 / 60.0;

		let uniform_data = build_projection_view(time, window.aspect());
		uniform_buffer.upload(&uniform_data);

		unsafe {
			gl::ClearColor(1.0, 0.8, 0.5, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		}

		main_shader.bind();

		// binds to the `UniformData` block
		uniform_buffer.bind_to_uniform_binding(0);

		// binds to the `VertexData` block
		triangle_vertex_buffer.bind_to_shader_storage_binding(0);

		unsafe {
			gl::DrawArrays(gl::TRIANGLES, 0, 3);
		}

		window.end_frame();
	}


	Ok(())
}



/// Your fancy camera logic
fn build_projection_view(time: f32, aspect: f32) -> Mat4 {
	use ultraviolet::projection::perspective_gl;

	let vertical_fov = PI/2.0;
	let projection = perspective_gl(vertical_fov, aspect, 0.01, 100.0);
	let view = Mat4::from_translation(Vec3::new(0.0, 0.0, -2.0))
		* Mat4::from_rotation_y(TAU * time);

	projection * view
}





/// A vertex type appropriately padded for `std430` layout.
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
struct Vertex {
	pos: Vec3,
	_padding0: f32,
	color: Vec3,
	_padding1: f32,
}

impl Vertex {
	fn new(pos: Vec3, color: Vec3) -> Vertex {
		Vertex { pos, color, ..Vertex::default() }
	}
}


/// Build a vertex buffer containing a single measly triangle.
fn build_triangle_buffer() -> Buffer {
	let vertex_data = [
		Vertex::new(Vec3::new( 0.0, 1.0, 0.0), Vec3::new( 1.0, 0.5, 0.5)),
		Vertex::new(Vec3::new( 1.0,-0.7, 0.0), Vec3::new( 0.5, 1.0, 0.5)),
		Vertex::new(Vec3::new(-1.0,-0.7, 0.0), Vec3::new( 0.5, 0.5, 1.0)),
	];

	let mut vertex_buffer = Buffer::new();
	vertex_buffer.upload_slice(&vertex_data);
	vertex_buffer
}
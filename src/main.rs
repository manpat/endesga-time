pub mod gl {
	include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub mod prelude;
pub mod window;
pub mod shader;

use prelude::*;
use window::Window;
use shader::ShaderProgram;


fn main() -> Result<(), Box<dyn Error>> {
	let sdl_ctx = sdl2::init()?;
	let sdl_video = sdl_ctx.video()?;

	let window = Window::new(&sdl_video)?;
	window.on_resize();


	let main_shader = ShaderProgram::new_simple(
		include_str!("shaders/main.vert.glsl"),
		include_str!("shaders/main.frag.glsl"),
	)?;

	unsafe {
		let mut vao = 0;
		gl::CreateVertexArrays(1, &mut vao);
		gl::BindVertexArray(vao);
	}



	let uniform_buffer_handle = unsafe {
		let mut uniform_buffer_handle = 0;
		gl::CreateBuffers(1, &mut uniform_buffer_handle);

		let uniform_binding = 0;
		gl::BindBufferBase(gl::UNIFORM_BUFFER, uniform_binding, uniform_buffer_handle);

		uniform_buffer_handle
	};


	let mut event_pump = sdl_ctx.event_pump()?;

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

		unsafe {
			use ultraviolet::projection::perspective_gl;

			let vertical_fov = PI/2.0;
			let projection = perspective_gl(vertical_fov, window.aspect(), 0.01, 100.0);
			let view = Mat4::from_translation(Vec3::new(0.0, 0.0, -2.0))
				* Mat4::from_rotation_y(TAU * time);

			let uniform_data = projection * view;
			let data_size = std::mem::size_of::<Mat4>();

			gl::NamedBufferData(
				uniform_buffer_handle,
				data_size as _,
				uniform_data.as_ptr() as _,
				gl::STREAM_DRAW,
			);
		}

		unsafe {
			gl::ClearColor(1.0, 0.8, 0.5, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		}

		main_shader.bind();

		unsafe {
			gl::DrawArrays(gl::TRIANGLES, 0, 3);
		}

		window.end_frame();
	}


	Ok(())
}


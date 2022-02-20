use crate::prelude::*;


/// Manages the sdl window and owns the associated opengl context
pub struct Window {
	sdl_window: sdl2::video::Window,
	_gl_ctx: sdl2::video::GLContext,
}

impl Window {
	pub fn new(sdl_video: &sdl2::VideoSubsystem) -> Result<Window, Box<dyn Error>> {
		let gl_attr = sdl_video.gl_attr();
		gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
		gl_attr.set_context_version(4, 5);
		gl_attr.set_context_flags().debug().set();
		
		gl_attr.set_framebuffer_srgb_compatible(true);
		gl_attr.set_stencil_size(8);

		let sdl_window = sdl_video.window("endesga-time", 1366, 768)
			.position_centered()
			.resizable()
			.opengl()
			.build()?;

		let gl_ctx = sdl_window.gl_create_context()?;
		sdl_window.gl_make_current(&gl_ctx)?;

		gl::load_with(|s| sdl_video.gl_get_proc_address(s) as *const _);

		setup_gl_state();

		Ok(Window {
			sdl_window,
			_gl_ctx: gl_ctx,
		})
	}

	pub fn on_resize(&self) {
		let (w, h) = self.sdl_window.drawable_size();

		unsafe {
			gl::Viewport(0, 0, w as i32, h as i32)
		}
	}

	pub fn end_frame(&self) {
		self.sdl_window.gl_swap_window();
	}

	pub fn aspect(&self) -> f32 {
		let (w, h) = self.sdl_window.drawable_size();
		w as f32 / h as f32
	}
}



fn setup_gl_state() {
	unsafe {
		gl::DebugMessageCallback(Some(gl_message_callback), std::ptr::null());
		gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);

		gl::Enable(gl::FRAMEBUFFER_SRGB);
		gl::Enable(gl::DEPTH_TEST);
		
		gl::FrontFace(gl::CCW);
		gl::CullFace(gl::BACK);

		// Disable performance messages
		gl::DebugMessageControl(
			gl::DONT_CARE,
			gl::DEBUG_TYPE_PERFORMANCE,
			gl::DONT_CARE,
			0, std::ptr::null(),
			0 // false
		);

		// Disable notification messages
		gl::DebugMessageControl(
			gl::DONT_CARE,
			gl::DONT_CARE,
			gl::DEBUG_SEVERITY_NOTIFICATION,
			0, std::ptr::null(),
			0 // false
		);
	}
}



extern "system" fn gl_message_callback(source: u32, ty: u32, _id: u32, severity: u32,
	_length: i32, msg: *const i8, _ud: *mut std::ffi::c_void)
{
	let severity_str = match severity {
		gl::DEBUG_SEVERITY_HIGH => "high",
		gl::DEBUG_SEVERITY_MEDIUM => "medium",
		gl::DEBUG_SEVERITY_LOW => "low",
		gl::DEBUG_SEVERITY_NOTIFICATION => return,
		_ => panic!("Unknown severity {}", severity),
	};

	let ty = match ty {
		gl::DEBUG_TYPE_ERROR => "error",
		gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "deprecated behaviour",
		gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "undefined behaviour",
		gl::DEBUG_TYPE_PORTABILITY => "portability",
		gl::DEBUG_TYPE_PERFORMANCE => "performance",
		gl::DEBUG_TYPE_OTHER => "other",
		_ => panic!("Unknown type {}", ty),
	};

	let source = match source {
		gl::DEBUG_SOURCE_API => "api",
		gl::DEBUG_SOURCE_WINDOW_SYSTEM => "window system",
		gl::DEBUG_SOURCE_SHADER_COMPILER => "shader compiler",
		gl::DEBUG_SOURCE_THIRD_PARTY => "third party",
		gl::DEBUG_SOURCE_APPLICATION => "application",
		gl::DEBUG_SOURCE_OTHER => "other",
		_ => panic!("Unknown source {}", source),
	};

	eprintln!("GL ERROR!");
	eprintln!("Source:   {}", source);
	eprintln!("Severity: {}", severity_str);
	eprintln!("Type:     {}", ty);

	unsafe {
		let msg = std::ffi::CStr::from_ptr(msg as _).to_str().unwrap();
		eprintln!("Message: {}", msg);
	}

	match severity {
		gl::DEBUG_SEVERITY_HIGH | gl::DEBUG_SEVERITY_MEDIUM => panic!("GL ERROR!"),
		_ => {}
	}
}

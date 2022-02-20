use crate::prelude::*;


/// 'Manages' opengl program objects.
/// NOTE: won't attempt to delete anything - that is left as an exercise for the reader.
#[derive(Copy, Clone, Debug)]
pub struct ShaderProgram (u32);

impl ShaderProgram {
	/// Compile and link a `ShaderProgram` from vertex and fragment shader source.
	pub fn new_simple(vsrc: &str, fsrc: &str) -> Result<ShaderProgram, CompilationError> {
		compile_shaders(&[
			(gl::VERTEX_SHADER, vsrc),
			(gl::FRAGMENT_SHADER, fsrc),
		])
	}

	/// Compile and link a `ShaderProgram` from compute shader source.
	pub fn new_compute(src: &str) -> Result<ShaderProgram, CompilationError> {
		compile_shaders(&[
			(gl::COMPUTE_SHADER, src),
		])
	}

	/// Bind this program as the active program.
	pub fn bind(&self) {
		unsafe {
			gl::UseProgram(self.0);
		}
	}
}


/// Don't look too hard at this.
fn compile_shaders(shaders: &[(u32, &str)]) -> Result<ShaderProgram, CompilationError> {
	use std::ffi::CString;
	use std::str;

	unsafe {
		let program_handle = gl::CreateProgram();

		for &(ty, src) in shaders.iter() {
			let src = CString::new(src.as_bytes()).unwrap();

			let shader_handle = gl::CreateShader(ty);

			gl::ShaderSource(shader_handle, 1, &src.as_ptr(), std::ptr::null());
			gl::CompileShader(shader_handle);

			let mut status = 0;
			gl::GetShaderiv(shader_handle, gl::COMPILE_STATUS, &mut status);

			if status == 0 {
				let mut length = 0;
				gl::GetShaderiv(shader_handle, gl::INFO_LOG_LENGTH, &mut length);

				let mut buffer = vec![0u8; length as usize];
				gl::GetShaderInfoLog(
					shader_handle,
					length,
					std::ptr::null_mut(),
					buffer.as_mut_ptr() as *mut _
				);

				let error = str::from_utf8(&buffer[..buffer.len()-1])
					.map_err(|_| CompilationError::new("shader compilation", "error message invalid utf-8"))?;

				return Err(CompilationError::new("shader compilation", error));
			}

			gl::AttachShader(program_handle, shader_handle);
			gl::DeleteShader(shader_handle);
		}

		gl::LinkProgram(program_handle);

		let mut status = 0;
		gl::GetProgramiv(program_handle, gl::LINK_STATUS, &mut status);

		if status == 0 {
			let mut buf = [0u8; 1024];
			let mut len = 0;
			gl::GetProgramInfoLog(program_handle, buf.len() as _, &mut len, buf.as_mut_ptr() as _);

			let error = str::from_utf8(&buf[..len as usize])
				.map_err(|_| CompilationError::new("shader linking", "error message invalid utf-8"))?;

			return Err(CompilationError::new("shader linking", error));
		}

		Ok(ShaderProgram(program_handle))
	}
}



#[derive(Debug)]
pub struct CompilationError {
	what: String,
	description: String,
}

impl CompilationError {
	fn new(what: &str, description: &str) -> CompilationError {
		CompilationError {
			what: what.into(),
			description: description.into(),
		}
	}
}

impl std::fmt::Display for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} failed\n", self.what)?;
        write!(f, "{}\n", self.description)
    }
}


impl Error for CompilationError {}
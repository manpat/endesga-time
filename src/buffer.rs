use crate::prelude::*;

/// 'Manages' opengl buffer objects.
/// NOTE: won't attempt to delete anything - that is left as an exercise for the reader.
#[derive(Copy, Clone, Debug)]
pub struct Buffer(u32);


impl Buffer {
	/// Does what it says on the tin.
	pub fn new() -> Buffer {
		unsafe {
			let mut uniform_buffer_handle = 0;
			gl::CreateBuffers(1, &mut uniform_buffer_handle);
			Buffer(uniform_buffer_handle)
		}
	}

	/// Fill the buffer object with the contents of a single object `data`.
	/// NOTE: it is assumed that `T` satisfies all the alignment requirements of whatever it is bound for.
	/// It is also assumed that `T` is `#[repr(C)]`
	pub fn upload<T: Copy>(&mut self, data: &T) {
		unsafe {
			let data_size = std::mem::size_of::<T>();

			gl::NamedBufferData(
				self.0,
				data_size as _,
				data as *const T as *const _,
				gl::STREAM_DRAW,
			);
		}
	}

	/// Fill the buffer object with the contents of a slice of objects `data`.
	/// NOTE: it is assumed that `T` satisfies all the alignment requirements of whatever it is bound for.
	/// It is also assumed that `T` is `#[repr(C)]`
	pub fn upload_slice<T: Copy>(&mut self, data: &[T]) {
		unsafe {
			let data_size = data.len() * std::mem::size_of::<T>();

			gl::NamedBufferData(
				self.0,
				data_size as _,
				data.as_ptr() as *const _,
				gl::STREAM_DRAW,
			);
		}
	}

	/// Bind this buffer to `uniform` blocks with `layout(binding='binding')`
	pub fn bind_to_uniform_binding(&self, binding: u32) {
		unsafe {
			gl::BindBufferBase(gl::UNIFORM_BUFFER, binding, self.0);
		}
	}

	/// Bind this buffer to `buffer` blocks with `layout(binding='binding')`
	pub fn bind_to_shader_storage_binding(&self, binding: u32) {
		unsafe {
			gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, binding, self.0);
		}
	}
}
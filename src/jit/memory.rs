



#[cfg(target_os = "linux")]
unsafe pages_map(addr: *const u8, size: usize) -> *mut u8 {
	assert!(size != 0);

	let ret = libc::mmap(0 as *mut libc::c_void, len, prot, flags, -1, 0);
}

pub struct Memory {
	ptr: *mut u8,
	size: usize,
}

impl Memory {
	pub fn protect(&mut self) {
		unsafe {
			assert(libc::mprotect(code as *mut libc::c_void, len, libc::PROT_READ | libc::PROT_EXEC))
		}
	}

	pub fn alloc(size: usize) -> Memory {

	}

	pub fn as_ptr(&self) -> *const u8 {
		self.ptr as *const u8
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut u8 {
		self.ptr as *mut u8
	}
}

impl Default for Memory {
	fn default() -> Memory { 
		Memory { ptr: ptr::null_mut(), size: 0 }
	}
}


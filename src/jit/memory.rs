 
extern crate libc;


use std::mem;
use std::ptr;
// todo use libc sysconf
const PAGE_SIZE: usize = 4096;

struct Memory {
	contents: *mut u8
}
impl Memory {
	fn new() -> Memory {
		let prot = libc::PROT_READ | libc::PROT_WRITE;
		let flags = libc::MAP_ANONYMOUS | libc::MAP_PRIVATE;
		let contents : *mut u8;
		unsafe {
			let mut _contents : *mut libc::c_void = mem::uninitialized();
			_contents = libc::mmap(0 as *mut libc::c_void, PAGE_SIZE, prot, flags, -1, 0);

			// ptr::write_bytes(_contents, 0\xc3, PAGE_SIZE);
			contents = mem::transmute(_contents);
		}
		Memory { contents: contents }
	}	
	fn finalize(&mut self) {
		unsafe {
			libc::mprotect(self.contents as *mut libc::c_void, PAGE_SIZE, libc::PROT_READ | libc::PROT_EXEC);
		}
		
	}
}

impl Drop for Memory {
    fn drop(&mut self) {
    	unsafe {
    		libc::munmap(self.contents as *mut libc::c_void, PAGE_SIZE);
    	}
    }
}


#[test]
fn test_memory() {

	let mut mem = Memory::new();
	let mut code = [
            0x55,                   // push rbp
            0x48, 0x89, 0xe5,       // mov  rbp, rsp
            0x89, 0x7d, 0xfc,       // mov  DWORD PTR [rbp-0x4],edi
            0x89, 0x75, 0xf8,       // mov  DWORD PTR [rbp-0x8],esi
            0x8b, 0x75, 0xfc,       // mov  esi,DWORD PTR [rbp-04x]
            0x0f, 0xaf, 0x75, 0xf8, // imul esi,DWORD PTR [rbp-0x8]
            0x89, 0xf0,             // mov  eax,esi
            0x5d,                   // pop  rbp
            0xc3                    // ret
    ];

    let mul: extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int;
    
    unsafe {
    	
    	ptr::copy(code.as_mut_ptr(), mem.contents, code.len());

    	mem.finalize();

    	mul = mem::transmute(mem.contents);
    }

    assert_eq!(mul(2 as libc::c_int,2 as libc::c_int) as libc::c_int, 4);
}
#![feature(libc)]
extern crate libc;
use std::ptr;
use std::mem;
const PAGE_SIZE: usize = 4096;
// const C_CALL_CONVECTION: [u8] = 
struct JITFn {
    code: *mut u8,
    len: usize
}

impl JITFn {
    fn new(mut buffer: Vec<u8>) -> JITFn {
    	let prot = libc::PROT_READ | libc::PROT_WRITE;
		let flags = libc::MAP_ANONYMOUS | libc::MAP_PRIVATE;
    	let len = buffer.len();
    	let code: *mut u8;
    	unsafe {
    		let mut _code : *mut libc::c_void = mem::uninitialized();
    		_code = libc::mmap(0 as *mut libc::c_void, len, prot, flags, -1, 0);
    		code = mem::transmute(_code);
    		ptr::copy_nonoverlapping(buffer.as_ptr(), code, len);
    		libc::mprotect(code as *mut libc::c_void, len, libc::PROT_READ | libc::PROT_EXEC);
    	}
        JITFn { code: code, len: len }
    }
    fn as_c_fn<CFn: CFnFromPtr, Fn: FnOnce(CFn) -> R, R>(&self, f: Fn) -> R {
		unsafe {
			let cfn = CFn::from_ptr(self.code as *const _);
			f(cfn)
		}
	}
}
impl Drop for JITFn {
    fn drop(&mut self) {
    	unsafe {
    		libc::munmap(self.code as *mut libc::c_void, self.len);
    	}
    }
}

trait CFnFromPtr {
    unsafe fn from_ptr(ptr: *const ()) -> Self;
}

impl<T> CFnFromPtr for extern "C" fn(libc::c_int, libc::c_int) -> T {
    unsafe fn from_ptr(ptr: *const ()) -> Self { mem::transmute(ptr) }
}

#[test]
fn test_fn() {
    let code = vec![
            0x55,                   // push rbp       
            0x48, 0x89, 0xe5,       // mov  rbp, rsp 
            						// ^^ Setup stack frame (C calling convention)

            0x89, 0x7d, 0xfc,       // mov  DWORD PTR [rbp-0x4],edi
            0x89, 0x75, 0xf8,       // mov  DWORD PTR [rbp-0x8],esi
            0x8b, 0x75, 0xfc,       // mov  esi,DWORD PTR [rbp-04x]
            0x0f, 0xaf, 0x75, 0xf8, // imul esi,DWORD PTR [rbp-0x8]
            0x89, 0xf0,             // mov  eax,esi
            0x5d,                   // pop  rbp
            0xc3                    // ret
    ];

 	
    let mul = JITFn::new(code.clone());
    let mul2 = JITFn::new(code.clone());


    mul.as_c_fn(|c_fn: extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int | {
    	println!("{:?}", c_fn(2,3));
    });
}
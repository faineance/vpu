extern crate libc;
use std::{ptr, mem};

pub struct Function {
    code: *mut u8,
    size: usize,
}

impl Function {
    pub fn new(buffer: &[u8]) -> Function {
        let prot = libc::PROT_READ | libc::PROT_WRITE;
        let flags = libc::MAP_ANONYMOUS | libc::MAP_PRIVATE;
        let size = buffer.len();
        let code: *mut u8;
        unsafe {
            let _code = libc::mmap(0 as *mut libc::c_void, size, prot,flags, -1, 0);
            code = mem::transmute(_code);
            ptr::copy_nonoverlapping(buffer.as_ptr(), code, size);
            assert!(libc::mprotect(code as *mut libc::c_void, size, libc::PROT_READ | libc::PROT_EXEC) == 0);
        }
        Function { code: code, size: size }
    }
}

impl Drop for Function {
    fn drop(&mut self) {
        unsafe {
            assert!(libc::munmap(self.code as *mut libc::c_void, self.size) == 0);
        }
    }
}


macro_rules! compilefn(
   ($fun:ident, ($($types:ty),+) -> $rettype:ty) => ({
      type FnType = extern "C" fn($($types),+) -> $rettype;
      let code = $fun.code;
      unsafe { mem::transmute::<*mut u8, FnType>(code) }
   })
);


#[test]
fn test() {
    let code = vec![
            0x55,                   // push rbp       
            0x48, 0x89, 0xe5,       // mov  rbp, rsp 
                                    // ^^ Setup stack frame (C calling convention)

            0x89, 0x7d, 0xfc,       // mov  DWORD PTR [rbp-0x4],edi
            0x89, 0x75, 0xf8,       // mov  DWORD PTR [rbp-0x8],esi
            0x8b, 0x75, 0xfc,       // mov  esi,DWORD PTR [rbp-04x]
            0x0f, 0xaf, 0x75, 0xf8, // imul esi,DWORD PTR [rbp-0x8]
            0x89, 0xf0,             // mov  eax\,esi
            0x5d,                   // pop  rbp
            0xc3                    // ret
    ];
    let fun = Function::new(&code);
    let mul = compilefn!(fun, (usize, usize) -> usize);

    assert_eq!(mul(3,2), 6);
}   
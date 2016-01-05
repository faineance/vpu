#![feature(libc, asm, convert)] 
extern crate byteorder;
mod runtime;
mod lir;
mod hir;
mod vm;
#![feature(libc, asm)] 
extern crate byteorder;
mod jit;
mod codegen;
mod lir;
mod hir;
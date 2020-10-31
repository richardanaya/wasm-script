use std::mem::transmute;
use cstring::{cstr,CString,cstr_to_string};

extern "C" {
    fn compiler_error(err: CString);
    fn compiler_log(err: CString);
}

pub fn throw_error(err: &str) {
    unsafe {
        compiler_error(cstr(err));
    }
}

pub fn log(msg: &str) {
    unsafe {
        compiler_log(cstr(msg));
    }
}



#[no_mangle]
pub extern "C" fn malloc(size: i32) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}

#[no_mangle]
pub extern "C" fn compile(codePtr: usize) -> usize {
    let code = cstr_to_string(codePtr as i32);
    log("compiling!");
    log(&code);
    // TODO: write a real compiler
    let b = include_bytes!("./add.wasm");
    let l = b.len();
    let len_bytes: [u8; 4] = unsafe { transmute(l) };
    let mut v = Vec::<u8>::new();
    v.extend(len_bytes.iter());
    v.extend(b.iter());
    let p =  &v as *const _ as usize;
    p  
}

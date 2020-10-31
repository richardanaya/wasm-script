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

fn code_as_string(codePtr: usize) -> String {
    cstr_to_string(codePtr as i32)
}

fn create_compiler_response(wasm_bytes:Vec<u8>) -> Vec<u8> {
    let l = wasm_bytes.len();
    let len_bytes: [u8; 4] = unsafe { transmute(l) };
    let mut v = Vec::<u8>::new();
    v.extend(len_bytes.iter());
    v.extend(wasm_bytes.iter());
    v
}


#[no_mangle]
pub extern "C" fn compile(code_ptr: usize) -> usize {
    let code = code_as_string(code_ptr);

    // we can send info to browser for help
    log("compiling the code below!!");
    log(&code);

    // TODO: write a real compiler
    let wasm_bytes = include_bytes!("./add.wasm").to_vec();

    &create_compiler_response(wasm_bytes) as *const _ as usize
}

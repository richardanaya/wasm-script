use cstring::{cstr, cstr_to_string, CString};
use std::error::Error;

extern "C" {
    fn compiler_error(err: CString) -> !;
    fn compiler_log(err: CString);
}

pub fn throw_error(err: &str) -> ! {
    unsafe {
        compiler_error(cstr(err));
    };
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

fn code_as_string(code_ptr: usize) -> String {
    cstr_to_string(code_ptr as i32)
}

fn create_compiler_response(wasm_bytes: Vec<u8>) -> Vec<u8> {
    let l = wasm_bytes.len() as u32;
    let len_bytes: [u8; 4] = unsafe { std::mem::transmute(l) };
    let mut v = Vec::<u8>::new();
    v.extend(len_bytes.iter());
    v.extend(wasm_bytes.iter());
    v
}

pub fn process<T>(code_ptr: usize, processor: T) -> usize
where
    T: Fn(&str) -> Result<Vec<u8>, Box<dyn Error>>,
{
    let code = code_as_string(code_ptr);
    // we can send info to browser for help
    log("compiling the code below!!");
    log(&code);

    let wasm_bytes = match processor(&code) {
        Ok(b) => b,
        Err(e) => throw_error(&e.to_string()),
    };

    &create_compiler_response(wasm_bytes) as *const _ as usize
}

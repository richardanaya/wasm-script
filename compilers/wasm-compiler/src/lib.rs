#![no_std]
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

extern "C" {
    fn compiler_error(err: usize) -> !;
    fn compiler_log(err: usize);
}

pub fn throw_error(err: &str) -> ! {
    unsafe {
        compiler_error(cstring::from_str(err));
    };
}

pub fn log(msg: &str) {
    unsafe {
        compiler_log(cstring::from_str(msg));
    }
}

#[no_mangle]
pub extern "C" fn malloc(size: i32) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}

fn create_compiler_response(wasm_bytes: Vec<u8>) -> Vec<u8> {
    let l = wasm_bytes.len() as u32;
    let len_bytes: [u8; 4] = unsafe { core::mem::transmute(l) };
    let mut v = Vec::<u8>::new();
    v.extend(len_bytes.iter());
    v.extend(wasm_bytes.iter());
    v
}

pub fn process<T>(code_ptr: usize, processor: T) -> usize
where
    T: Fn(&str) -> Result<Vec<u8>, String>,
{
    match cstring::try_into_string(code_ptr) {
        Ok(code) => {
            let wasm_bytes = match processor(&code) {
                Ok(b) => b,
                Err(e) => throw_error(&e),
            };
            &create_compiler_response(wasm_bytes) as *const _ as usize
        }
        Err(e) => throw_error(&e),
    }
}

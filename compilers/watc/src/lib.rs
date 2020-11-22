use wasm_compiler::{log, process};

#[no_mangle]
pub fn compile(code_ptr: usize) -> usize {
    process(code_ptr, |code| {
        Ok(match wat::parse_str(code) {
            Ok(a) => a,
            Err(e) => return Err(e.to_string()),
        })
    })
}

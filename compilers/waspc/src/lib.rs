use wasm_compiler::{log, process};
use wasp_core::{compiler, parser};

#[no_mangle]
pub fn compile(code_ptr: usize) -> usize {
    process(code_ptr, |code| {
        log("compiling the code below!!");
        log(&code);
        let app = match parser::parse(code) {
            Ok(a) => a,
            Err(e) => return Err(e.to_string()),
        };
        Ok(match compiler::compile(app) {
            Ok(a) => a,
            Err(e) => return Err(e.to_string()),
        })
    })
}

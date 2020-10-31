use wasm_compiler::{process,log};
use wasp_core::{compiler, parser};

#[no_mangle]
pub fn compile(code_ptr: usize) -> usize {
    process(code_ptr, |code| {
        log("compiling the code below!!");
        log(&code);
        let app = parser::parse(code)?;
        Ok(compiler::compile(app)?)
    })
}

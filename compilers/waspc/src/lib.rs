use wasm_compiler::process;
use wasp_core::{compiler, parser};

#[no_mangle]
pub fn compile(code_ptr: usize) -> usize {
    process(code_ptr, |code| {
        let app = parser::parse(code)?;
        Ok(compiler::compile(app)?)
    })
}

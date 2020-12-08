use wasm_compiler::process;

fn brainfuck_to_wat(code: &str) -> String {
    let body:Vec<String> = vec![];
    return [r#"
    (module
        (type $t0 (func))
        (type $t1 (func (param f64 f64) (result f64)))
        (type $t2 (func (param f64 f64 f64) (result f64)))
        (import "env" "js_register_function" (func $js_register_function (type $t1)))
        (import "env" "js_invoke_function" (func $js_invoke_function (type $t2)))
        (func $main (export "main") (type $t0) (local f64)
          i32.const 1024
          f64.convert_u/i32
          f64.const 0x1.e4p+6 (;=121;)
          call $js_register_function
          local.set 0
          (global.set $g0 (local.get 0))
          "#,&body.join("\n"),r#"
        )
        (table $T0 1 1 anyfunc)
        (memory $memory (export "memory") 17)
        (global $g0 (mut f64) (f64.const 0))
        (global $g1 (mut f64) (f64.const 0))
        (data (i32.const 1024) "function(strPtr,strLen){\0a                document.body.innerHTML+= this.readUtf8FromMemory(strPtr,strLen); \0a            }"))      
    "#].join("")        
}

/*
          global.get $g0
          i32.const 1024
          f64.convert_u/i32
          i32.const 40
          f64.convert_u/i32
          call $js_invoke_function*/

#[no_mangle]
pub fn compile(code_ptr: usize) -> usize {
    process(code_ptr, |code| {
        Ok(match wat::parse_str(brainfuck_to_wat(code)) {
            Ok(a) => a,
            Err(e) => return Err(e.to_string()),
        })
    })
}

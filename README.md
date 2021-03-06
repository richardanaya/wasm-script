# wasm-script

This is a library for bringing your WebAssembly compiler to the web.

# Vision

```html
<script src="wasm-script.js"></script>
<wasm-script id="math" lang="c" compiler="http://yourcompiler.com/c.wasm">
     
     extern int add(int a, int b){
          return a + b;
     }  

</wasm-script>
<script type="module">
     // Compile as a module for accelerating javascript
     const mathModule = await document.getElementById("math").compile();
     console.log(mathModule.add(2,2));
</script>
```

Or run stand alone

```html
<script src="wasm-script.min.js"></script>
<wasm-script lang="python" src="tetris.python" execute></wasm-script>
<!-- use default compilers from the community for certain languages -->
```

# Features

- [x] script for loading a WebAssembly module from a compiler
- [ ] ask to load dependent files by url
- [ ] cacheing
- [x] show compile errors in console

# CDN 

```
https://cdn.jsdelivr.net/gh/richardanaya/wasm-script@latest/wasm-script.js
```

# Reference Implementation

A reference implementation compiler is in progress in Rust for the [`wasp`](https://github.com/wasplang/wasp) programming language ( a simple lisp-like syntax language ).

```html
<script src="wasm-script.js"></script>
<wasm-script id="math" lang="wasp" compiler="compilers/waspc/compiler.wasm">
     
    pub fn secret(){
        42
    }

</wasm-script>
<script>
     // top-level await doesn't exist yet, so we have to do it the lame way
    (async function(){
        const mathModule = await document.getElementById("math").compile();
        document.body.innerHTML = mathModule.secret(1);
    })();
</script>
```

See the demo [here](https://richardanaya.github.io/wasm-script/examples/demo.html)

What the compiler is doing is fairly simple:

```rust
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
```

# Exposing Browser Functionality

Until we have DOM exposure in WebAssembly, we still are in a position we need to provide our own exposures to the browser's functionality. This library has some helper functions
for common memory structures.


```html
<script src="../wasm-script.js"></script>
<wasm-script id="math" lang="wasp" compiler="../compilers/waspc/compiler.wasm">
     
    extern console_log(message)
    pub fn main(){
        console_log("hello world!")
    }

</wasm-script>
<script>
     // top-level await doesn't exist yet, so we have to do it the lame way
    (async function(){
        const mathModule = await document.getElementById("math").compile({
            console_log: function(message_start) {
                const message = WasmScript.getCString(mathModule.memory,message_start)
                document.body.innerHTML += message;
              }
        });
        mathModule.main();
    })();
</script>
```

See the demo [here](https://richardanaya.github.io/wasm-script/examples/helloworld.html)

## Memory Helper API

* `WasmScript.getCString(mem,start)` - Returns a utf-8 string from position in memory that ends with a zero character.
* `WasmScript.getUint8ArrayFromMemory(mem,start)` - Returns a Uint8Array from position in memory, starting with a uint32 count followed by the elements.

# Building Your Own Compiler

Right now the current contract for building a compiler is very simple and can be implemented in any language that compiles to WebAssembly:

```rust
// external function to print a long takes a zero-character ending C string
extern void compiler_log(uint32 start_cstr)
// external function to print a error log and throw an exception takes a zero-character ending
// C string ( this function does not return )
extern void compiler_error(uint32 start_cstr)

// allocate some bytes of size
uint32 malloc(uint32 size)
// compile a code by passing in the start of the code in memory (created using malloc above). 
// and get back a list of bytes (the length as u32, followed by the data)
uint32 compile(uint32 code_str)
```

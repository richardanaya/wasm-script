# wasm-script

This is a library for bringing your WebAssembly compiler to the web.

# Vision

```html
<script src="https://unpkg.com/wasm-script@latest/wasm-script.min.js"></script>
<wasm-script id="math" lang="c" compiler="http://yourcompiler.com/c.wasm">
     
     #include <https://hypatia.com/math.h>

     extern int add(int a, int b){
          return hypatia_add(a,b);
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
<script src="https://unpkg.com/wasm-script@latest/wasm-script.min.js"></script>
<wasm-script lang="python" src="tetris.python" execute></wasm-script>
<!-- use default compilers from the community for certain languages -->
```

# Features

- [x] script for loading a WebAssembly module from a compiler
- [ ] ask to load dependent files by url
- [ ] cacheing
- [x] show compile errors in console

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

See the demo [here](https://richardanaya.github.io/wasm-script/demo.html)

What the compiler is doing is fairly simple:

```rust
fn compile(code_ptr: usize) -> usize {
    let code = code_as_string(code_ptr);

    // we can send info to browser for help
    log("compiling the code below!");
    log(&code);

    // TODO: write a real compiler
    let wasm_bytes =  ...

    &create_compiler_response(wasm_bytes) as *const _ as usize
}

```

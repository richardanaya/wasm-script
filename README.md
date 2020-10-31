# wasm-script

This is a library for bringing your WebAssembly compiler to the web.

# Vision

```html
<script src="https://unpkg.com/wasm-script@latest/wasm-script.min.js"></script>
<wasm-script id="math" lang="C" compiler="http://yourcompiler.com/c.wasm">
     
     #include <https://hypatia.com/math.h>

     extern int add(int a, int b){
          return hypatia_add(a,b);
     }  

</wasm-script>
<script type="module">
     const mathModule = await document.getElementById("math").compile();
     console.log(mathModule.add(2,2));
</script>
```

# Features

- [x] script for loading a WebAssembly module from a compiler
- [ ] ask to load dependent files by url
- [ ] cacheing interface

# Reference Implementation

A reference implementation compiler is in progress in Rust for the `wasp` programming language ( a simple lisp-like syntax language ). It's built using `watson` and `wci`.

```html
<script src="wasm-script.min.js"></script>
<wasm-script id="math" lang="wasp" compiler="https://unpkg.com/gh/wasplang/wasp@latest/compiler.wasm">
     
     (extern add (a,b) (+ a b))

</wasm-script>
<script>
     // top-level await doesn't exist yet, so we have to do it the lame way
    (async function(){
        const mathModule = await document.getElementById("math").compile();
        console.log(mathModule.add(2,2));
    })();
</script>
```

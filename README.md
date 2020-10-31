# wasm-script
Compile WebAssembly in your HTML

```html
<script src="https://unpkg.com/wasm-script@latest/wasm-script.min.js"></script>
<wasm-script id="math" lang="C" compiler="http://yourcompiler.com/c.wasm">
extern int add(int a, int b){
     return a+b;
}  
</wasm-script>
<script>
const mathModule = await document.getByID("math").compile();
console.log(mathModule.add(2,2));
</script>
```

class WasmScript extends HTMLElement {
    constructor() {
        super();
        this.utf8dec = new TextDecoder("utf-8");
        this.utf8enc = new TextEncoder("utf-8");
    }

    static getCString(memory,start){
        let utf8dec = new TextDecoder("utf-8");
        const data = new Uint8Array(memory.buffer);
        const str = [];
        let i = start;
        while (data[i] !== 0) {
            str.push(data[i]);
            i++;
        }
        return utf8dec.decode(new Uint8Array(str));
    }

    connectedCallback() {
        if (this.innerHTML === undefined) {
            this.setup()
        } else {
            this.code = this.innerHTML;
            this.innerHTML = "";
        }
    }

    childrenAvailableCallback() {
        this.code = this.innerHTML;
        this.innerHTML = "";
        this.codeResolver(this.code)
    }

    // https://stackoverflow.com/questions/48498581/textcontent-empty-in-connectedcallback-of-a-custom-htmlelement
    setup() {
        self.parentNodes = []
        let el = this;
        while (el.parentNode) {
            el = el.parentNode
            this.parentNodes.push(el)
        }
        if ([this, ...this.parentNodes].some(el => el.nextSibling) || document.readyState !== 'loading') {
            this.childrenAvailableCallback();
        } else {
            this.mutationObserver = new MutationObserver(() => {
                if ([this, ...this.parentNodes].some(el => el.nextSibling) || document.readyState !== 'loading') {
                    this.childrenAvailableCallback()
                    this.mutationObserver.disconnect()
                }
            });
            this.mutationObserver.observe(this, { childList: true });
        }
    }

    async compile(env) {
        if (this.innerHTML) {
            this.code = this.innerHTML;
            this.innerHTML = "";
        }
        if (this.code) {
            return await this.compileCode(this.code, env)
        }
        await new Promise((resolve) => {
            this.codeResolver = resolve;
        });
        return await this.compileCode(this.code, env)
    }

    getBytesFromMemory(t, mem, start) {
        const data32 = new Uint32Array(mem);
        const ptr = data32[start / 4];
        const length = data32[ptr / 4];
        let b = mem.slice(ptr + 4, ptr + 4 + length);
        let a = new t(b);
        return a;
    }

    getStringFromMemory(mem, start) {
        const data = new Uint8Array(mem);
        const str = [];
        let i = start;
        while (data[i] !== 0) {
            str.push(data[i]);
            i++;
        }
        return this.utf8dec.decode(new Uint8Array(str));
    }

    createString(mod, str) {
        let bytes = this.utf8enc.encode(str + String.fromCharCode(0));
        let len = bytes.length;
        let start = mod.instance.exports.malloc(len);
        const memory = new Uint8Array(mod.instance.exports.memory.buffer);
        memory.set(bytes, start);
        return start;
      }


    async compileCode(code, env) {
        // create the compiler
        let compiler = this.getAttribute("compiler");
        if (!compiler) {
            throw "no compiler specified";
        }
        let response = await fetch(compiler);
        let bytes = await response.arrayBuffer();
        let compilerModule = await WebAssembly.instantiate(bytes, {
            env: {
                // functions needed for compiler
                compiler_error: (e) => {
                    let err = this.getStringFromMemory(
                        compilerModule.instance.exports.memory.buffer,
                        e
                    );
                    console.error(err);
                    throw new Error("Web assembly module exited unexpectedly.");
                },
                compiler_log: (e) => {
                    let msg = this.getStringFromMemory(
                        compilerModule.instance.exports.memory.buffer,
                        e
                    );
                    console.log(msg);
                }
            }
        });

        let positionStart = this.createString(compilerModule, code);

        // compile code into bytes
        const wasmBytesPosition = compilerModule.instance.exports.compile(positionStart);

        const wasmBytes = this.getBytesFromMemory(
            Uint8Array,
            compilerModule.instance.exports.memory.buffer,
            wasmBytesPosition
        );

        // start up code with environment 
        let module = await WebAssembly.instantiate(wasmBytes, {
            env
        });

        return module.instance.exports;
    }
}

customElements.define('wasm-script', WasmScript);
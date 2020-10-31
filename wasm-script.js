class WasmScript extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        if(this.innerHTML === undefined){
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

    async compile() {
        if(this.innerHTML){
            this.code = this.innerHTML;
            this.innerHTML = "";
        }
        if(this.code){
            return await this.compileCode(this.code)
        }
        await new Promise((resolve)=>{
            this.codeResolver = resolve;
        });
        return await this.compileCode(this.code)
    }

    async compileCode(code){
        return {
            add(a, b) {
                return a + b;
            }
        };
    }
}

customElements.define('wasm-script', WasmScript);
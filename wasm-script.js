class WasmScript extends HTMLElement {
    constructor() {
      // Always call super first in constructor
      super();
    }

    async compile(){
        return {
            add(a,b){
                return a+b;
            }
        }
    }
  }

  customElements.define('wasm-script', WasmScript);
const js = import("./node_modules/wasm-ui/wasm_ui.js");
js.then(js => {
    js.greet("WebAssembly");
});
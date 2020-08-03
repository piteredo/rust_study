const js = import("../pkg/wasmtest");
js.then(js => {
  js.greet("WebAssembly");
});

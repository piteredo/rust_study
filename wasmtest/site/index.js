const js = import("@piteredo/wasmtest");
js.then(js => {
  js.greet("WebAssembly");
});

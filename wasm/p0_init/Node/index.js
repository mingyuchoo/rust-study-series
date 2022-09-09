const fs = require('fs');

const wasmBufCode = fs.readFileSync('./Rust.wasm');
const wasmModule = new WebAssembly.Module(wasmBufCode);
const wasmInstance = new WebAssembly.Instance(wasmModule, []);

const add = (a, b) => {
  console.log(wasmInstance.exports.add(a, b));
};

add(1, 2);


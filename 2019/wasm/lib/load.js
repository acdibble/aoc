const { readFileSync } = require('fs');

module.exports = async (wasm, imports = {}) => {
  const buffer = readFileSync(wasm);
  const module = await WebAssembly.compile(buffer);
  const instance = await WebAssembly.instantiate(module, imports);
  return instance.exports;
};

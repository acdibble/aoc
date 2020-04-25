const { readFileSync } = require('fs');

module.exports = async (wasm, imports = {}) => {
  const module = await WebAssembly.compile(readFileSync(wasm));
  const instance = await WebAssembly.instantiate(module, imports);
  return instance.exports;
};

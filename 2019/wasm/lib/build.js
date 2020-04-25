#!/usr/bin/env node
const { readFileSync, writeFileSync } = require('fs');
const wabt = require('wabt')();

const [inputWat, outputWasm] = process.argv.slice(2);

const wasmModule = wabt.parseWat(inputWat, readFileSync(inputWat, 'utf8'));
const { buffer } = wasmModule.toBinary({});

(async () => {
  try {
    await WebAssembly.compile(buffer);
    writeFileSync(outputWasm, Buffer.from(buffer))
  } catch (e) {
    console.log(e);
  }
})();

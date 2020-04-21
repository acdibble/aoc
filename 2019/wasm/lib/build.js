#!/usr/bin/env node
const { readFileSync, writeFileSync } = require('fs');
const wabt = require('wabt')();

const [inputWat, outputWasm] = process.argv.slice(2);

const wasmModule = wabt.parseWat(inputWat, readFileSync(inputWat, 'utf8'));
const { buffer } = wasmModule.toBinary({});

writeFileSync(outputWasm, Buffer.from(buffer))

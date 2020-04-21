const { readFileSync } = require('fs');
const load = require("../lib/load");

(async () => {
  const nums = readFileSync(`${__dirname}/data.txt`, 'utf8').split('\n').map(Number);
  const module = await load(`${__dirname}/${process.argv[2]}.wasm`);
  const buffer = new Int32Array(module.memory.buffer, 0, nums.length);
  nums.forEach((num, i) => {
    buffer[i] = num;
  });
  console.log(module.accumulate(nums.length));
})();

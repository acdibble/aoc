const fs = require('fs');
const load = require('../lib/load');

(async () => {
  const intcodes = fs.readFileSync(`${__dirname}/input.txt`, 'utf8').split(',').map(Number);
  const module = await load(`${__dirname}/${process.argv[2]}.wasm`);
  const memory = new Int32Array(module.memory.buffer, 0, intcodes.length * 2);
  intcodes.forEach((code, i) => {
    memory[i] = code;
    memory[i + intcodes.length] = code;
  });

  memory[1] = 12;
  memory[2] = 2;

  console.log(module.processIntcodes(intcodes.length));
})();

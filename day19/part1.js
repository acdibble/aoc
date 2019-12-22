const fs = require('fs');
const intcodeComputer = require('../intcodeComputer');

const intcodes = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .trim()
  .split(',')
  .map(Number);

const MAP_SIZE = 50;

const inputs = [];
let result = 0;

const run = intcodeComputer(() => inputs.shift(), (a) => { result += a; });

for (let y = 0; y < MAP_SIZE; y++) {
  for (let x = 0; x < MAP_SIZE; x++) {
    inputs.push(x, y);
    run(intcodes);
  }
}

console.log(result);

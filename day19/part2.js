const fs = require('fs');
const intcodeComputer = require('../intcodeComputer');

const intcodes = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .trim()
  .split(',')
  .map(Number);

const SHIP_SIZE = 100;

const slots = [0, 0, 0];
let inputIndex = 1;

const run = intcodeComputer(() => {
  inputIndex ^= 1;
  return slots[inputIndex];
}, (a) => {
  slots[2] = a;
});

let output = null;
let y = 5 * SHIP_SIZE;
let x = 0;
while (!output) {
  slots[0] = x;
  slots[1] = y;
  run(intcodes);
  while (slots[2] !== 1) {
    x += 1;
    slots[0] = x;
    run(intcodes);
  }

  slots[0] = x + (SHIP_SIZE - 1);
  slots[1] = y - (SHIP_SIZE - 1);
  run(intcodes);

  if (slots[2] === 0) {
    y += 1;
  } else {
    output = { x, y: y - (SHIP_SIZE - 1) };
  }
}

console.log(output);
console.log(output.x * 10000 + output.y);

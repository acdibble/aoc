const fs = require('fs');
const intcodeComputer = require('../lib/intcodeComputer');

const intcodes = fs.readFileSync(`${__dirname}/data.txt`, 'utf8').split(',').map(Number);
const instructions = fs.readFileSync(`${__dirname}/part2.icc`, 'utf8')
  .split('')
  .map((c) => c.charCodeAt(0));

console.log(intcodes);

const outputs = [];

let damage = 0;

const run = intcodeComputer(() => {
  if (outputs.length) {
    console.log(outputs.splice(0, Infinity).join(''));
  }
  return instructions.shift();
}, (a) => {
  if (a <= 126) {
    outputs.push(String.fromCharCode(a));
  } else {
    damage += a;
  }
});

run(intcodes);

fs.writeFileSync(`${__dirname}/out.txt`, outputs.join(''), 'utf8');
console.log(damage);

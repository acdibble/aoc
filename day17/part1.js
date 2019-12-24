const fs = require('fs');
const intcodeComputer = require('../lib/intcodeComputer');

const intcodes = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .replace(/\s/g, '')
  .split(',')
  .map(Number);

const outputs = [];

intcodeComputer(null, (val) => outputs.push(val))(intcodes);

const lineLength = outputs.indexOf(10) + 1;

const SCAFFOLD = 35;

console.log(outputs.reduce((sum, val, i) => {
  if (
    val === SCAFFOLD
    && outputs[i + 1] === SCAFFOLD
    && outputs[i - 1] === SCAFFOLD
    && outputs[i + lineLength] === SCAFFOLD
    && outputs[i - lineLength] === SCAFFOLD
  ) {
    return sum + Math.floor(i / lineLength) * (i % lineLength);
  }

  return sum;
}, 0));

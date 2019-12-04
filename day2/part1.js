const fs = require('fs');

const processIntcodes = (string) => {
  const intcodes = string.trim().split(',').map(Number);
  intcodes[1] = 12;
  intcodes[2] = 2;

  for (let i = 0; i < intcodes.length; i += 4) {
    const opcode = intcodes[i];

    if (opcode === 99 || (opcode !== 1 && opcode !== 2)) break;
    const operand1 = intcodes[intcodes[i + 1]];
    const operand2 = intcodes[intcodes[i + 2]];

    intcodes[intcodes[i + 3]] = opcode === 1 ? operand1 + operand2 : operand1 * operand2;
  }

  return intcodes[0];
};

console.log(processIntcodes(fs.readFileSync(`${__dirname}/data.txt`, 'utf8')));

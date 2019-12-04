const fs = require('fs');

const processIntcodes = (string, noun, verb) => {
  const intcodes = string.trim().split(',').map(Number);
  intcodes[1] = noun;
  intcodes[2] = verb;

  for (let i = 0; i < intcodes.length; i += 4) {
    const opcode = intcodes[i];

    if (opcode === 99 || (opcode !== 1 && opcode !== 2)) break;
    const operand1 = intcodes[intcodes[i + 1]];
    const operand2 = intcodes[intcodes[i + 2]];

    intcodes[intcodes[i + 3]] = opcode === 1 ? operand1 + operand2 : operand1 * operand2;
  }

  return intcodes[0];
};

let noun = null;
let verb = null;

for (let i = 0; i <= 99; i++) {
  for (let j = 0; j <= 99; j++) {
    if (processIntcodes(fs.readFileSync(`${__dirname}/data.txt`, 'utf8'), i, j) === 19690720) {
      noun = i;
      verb = j;
    }
  }

  if (noun && verb) break;
}

console.log(100 * noun + verb);

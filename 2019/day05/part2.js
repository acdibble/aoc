/* eslint-disable no-await-in-loop */
const fs = require('fs');
const readline = require('readline');

const getInput = () => new Promise((resolve) => {
  const rl = readline.createInterface(process.stdin, process.stdout);

  rl.question('\n> ', (answer) => {
    rl.close();
    resolve(Number(answer));
  });
});

const opLengths = {
  1: 4,
  2: 4,
  3: 2,
  4: 2,
  5: 3,
  6: 3,
  7: 4,
  8: 4,
};

const ops = {
  1(a, b) {
    return a + b;
  },
  2(a, b) {
    return a * b;
  },
  3() {
    return getInput();
  },
  4(a) {
    console.log(a);
  },
  5(a, b) {
    return a !== 0 ? b : -1;
  },
  6(a, b) {
    return a === 0 ? b : -1;
  },
  7(a, b) {
    return a < b ? 1 : 0;
  },
  8(a, b) {
    return a === b ? 1 : 0;
  },
};

const processIntcodes = async (string, noun, verb) => {
  const intcodes = string.replace(/\s/g, '').split(',').map(Number);
  if (noun) intcodes[1] = Number(noun);
  if (verb) intcodes[2] = Number(verb);

  let i = 0;
  while (i < intcodes.length) {
    let opcode = intcodes[i];
    if (opcode === 99) break;

    const padded = opcode.toString().padStart(5, '0');
    opcode = Number(padded.slice(3));
    const param1 = intcodes[i + 1];
    const operand1 = padded[2] === '1' ? param1 : intcodes[param1];
    let operand2 = null;
    if (opcode !== 3 && opcode !== 4) {
      const param2 = intcodes[i + 2];
      operand2 = padded[1] === '1' ? param2 : intcodes[param2];
    }

    const result = await ops[opcode](operand1, operand2);
    if (opcode !== 4 || opcode !== 5 || opcode !== 6) {
      intcodes[intcodes[i + 3]] = result;
    }

    if ((opcode === 5 || opcode === 6) && result !== -1) {
      i = result;
    } else {
      i += opLengths[opcode];
    }
  }
};

processIntcodes(fs.readFileSync(`${__dirname}/data.txt`, 'utf8'));

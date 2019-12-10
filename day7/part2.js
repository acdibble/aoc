/* eslint-disable no-await-in-loop */
const fs = require('fs');
const permutate = require('./heapsAlgorithm');

const ops = {
  // ADD
  1(a, b) {
    return a + b;
  },
  // MULT
  2(a, b) {
    return a * b;
  },
  // SET
  // 3() {
  // },
  // GET
  // 4(a) {
  // },
  // NE
  5(a, b) {
    return a !== 0 ? b : -1;
  },
  // EQ
  6(a, b) {
    return a === 0 ? b : -1;
  },
  // LT
  7(a, b) {
    return a < b ? 1 : 0;
  },
  // EQ
  8(a, b) {
    return a === b ? 1 : 0;
  },
};

const processIntcodes = (inputIntcodes, phaseSettings, ioRegister, ampNumber) => {
  const intcodes = inputIntcodes.slice();
  let phaseSet = false;
  let i = 0;
  let isRunning = true;
  return () => {
    while (i < intcodes.length) {
      let opcode = intcodes[i];
      if (opcode === 99) {
        isRunning = false;
        break;
      }

      const padded = opcode.toString().padStart(5, '0');
      opcode = Number(padded.slice(3));
      const param1 = intcodes[++i];
      const operand1 = padded[2] === '1' ? param1 : intcodes[param1];
      let operand2 = null;
      if (opcode !== 3 && opcode !== 4) {
        const param2 = intcodes[++i];
        operand2 = padded[1] === '1' ? param2 : intcodes[param2];
      }

      let result = null;
      if (opcode !== 3 && opcode !== 4) {
        result = ops[opcode](operand1, operand2);
        if (opcode !== 5 && opcode !== 6) {
          intcodes[intcodes[++i]] = result;
        }
      } else if (opcode === 3) {
        intcodes[param1] = (phaseSet ? ioRegister : phaseSettings)[ampNumber];
        phaseSet = true;
      } else if (opcode === 4) {
        ioRegister[(ampNumber + 1) % 5] = operand1;
        i += 1;
        return isRunning;
      }

      if ((opcode === 5 || opcode === 6) && result !== -1) {
        i = result;
      } else {
        i += 1;
      }
    }

    return isRunning;
  };
};

const intcodes = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .replace(/\s/g, '')
  .split(',')
  .map(Number);

const permutations = permutate([5, 6, 7, 8, 9]);

(async () => {
  const thrustLevels = [];
  // eslint-disable-next-line no-restricted-syntax
  for (const permutation of permutations) {
    const numbers = permutation.slice();
    const ioRegister = [0, 0, 0, 0, 0];
    const amps = [];
    for (let i = 0; i < 5; i++) {
      amps.push(processIntcodes(intcodes, numbers, ioRegister, i));
    }

    while (amps.every((amp) => amp())) { }

    thrustLevels.push(ioRegister[0]);
  }

  console.log(Math.max(...thrustLevels));
})();

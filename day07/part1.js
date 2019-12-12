/* eslint-disable no-await-in-loop */
const fs = require('fs');
const EventEmitter = require('events');
const permutate = require('./heapsAlgorithm');

const emitter = new EventEmitter();

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
  3() {
    return new Promise((resolve) => {
      emitter.once('input:response', resolve);
      emitter.emit('input:request');
    });
  },
  // GET
  4(a) {
    return new Promise((resolve) => {
      emitter.once('output:response', () => resolve());
      emitter.emit('output:request', a);
    });
  },
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

const processIntcodes = async (inputIntcodes) => {
  const intcodes = inputIntcodes.slice();

  let i = 0;
  while (i < intcodes.length) {
    let opcode = intcodes[i];
    if (opcode === 99) break;

    const padded = opcode.toString().padStart(5, '0');
    opcode = Number(padded.slice(3));
    const param1 = intcodes[++i];
    const operand1 = padded[2] === '1' ? param1 : intcodes[param1];
    let operand2 = null;
    if (opcode !== 3 && opcode !== 4) {
      const param2 = intcodes[++i];
      operand2 = padded[1] === '1' ? param2 : intcodes[param2];
    }

    const result = await ops[opcode](operand1, operand2);
    if (opcode !== 3 && opcode !== 4 && opcode !== 5 && opcode !== 6) {
      intcodes[intcodes[++i]] = result;
    } else if (opcode === 3) {
      intcodes[param1] = result;
    }

    if ((opcode === 5 || opcode === 6) && result !== -1) {
      i = result;
    } else {
      i += 1;
    }
  }
};

const intcodes = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .replace(/\s/g, '')
  .split(',')
  .map(Number);

const permutations = permutate([0, 1, 2, 3, 4]);

(async () => {
  let finalOutput = 0;
  // eslint-disable-next-line no-restricted-syntax
  for (const permutation of permutations) {
    const numbers = permutation.slice();
    let callCount = 0;
    const outputs = [0];
    const captureOutput = (e) => {
      outputs.push(e);
      emitter.emit('output:response');
    };
    const cb = () => {
      const number = callCount % 2 === 0 ? numbers.shift() : outputs.shift();
      callCount += 1;
      emitter.emit('input:response', number);
    };
    emitter.on('input:request', cb);
    emitter.on('output:request', captureOutput);
    for (let i = 0; i < 5; i++) {
      await processIntcodes(intcodes);
    }
    emitter.removeListener('input:request', cb);
    emitter.removeListener('output:request', captureOutput);

    finalOutput = finalOutput < outputs[0] ? outputs[0] : finalOutput;
  }
  console.log(finalOutput);
})();

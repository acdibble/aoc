const fs = require('fs');
const { EventEmitter } = require('events');

const emitter = new EventEmitter();

const ops = {
  // ADD
  1(a = 0, b = 0) {
    return a + b;
  },
  // MULT
  2(a = 0, b = 0) {
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
      // console.log(a);
      emitter.once('output:received', resolve);
      emitter.emit('output:new', a);
    });
  },
  // NE
  5(a = 0, b = 0) {
    return a !== 0 ? b : -1;
  },
  // EQ
  6(a = 0, b = 0) {
    return a === 0 ? b : -1;
  },
  // LT
  7(a = 0, b = 0) {
    return a < b ? 1 : 0;
  },
  // EQ
  8(a = 0, b = 0) {
    return a === b ? 1 : 0;
  },
  9(a, relativeBase) {
    return a + relativeBase;
  },
};

const processIntcodes = async (inputIntcodes) => {
  const intcodes = inputIntcodes.slice();
  let relativeBase = 0;
  let i = 0;
  while (i < intcodes.length) {
    let opcode = intcodes[i];
    if (opcode === 99) {
      break;
    }

    const padded = opcode.toString().padStart(5, '0');
    opcode = Number(padded.slice(3));
    let param1 = intcodes[++i];
    let operand1 = null;
    if (padded[2] === '1') {
      operand1 = param1;
    } else if (padded[2] === '2') {
      if (opcode === 3) {
        param1 = relativeBase + param1;
      }
      operand1 = intcodes[relativeBase + param1];
    } else {
      operand1 = intcodes[param1];
    }
    let operand2 = null;
    if (opcode !== 3 && opcode !== 4 && opcode !== 9) {
      const param2 = intcodes[++i];
      if (padded[1] === '1') {
        operand2 = param2;
      } else if (padded[1] === '2') {
        operand2 = intcodes[relativeBase + param2];
      } else {
        operand2 = intcodes[param2];
      }
    } else if (opcode === 9) {
      operand2 = relativeBase;
    }

    const result = await ops[opcode](operand1, operand2);
    if (opcode === 1 || opcode === 2 || opcode === 7 || opcode === 8) {
      const index = intcodes[++i] + (padded[0] === '2' ? relativeBase : 0);
      intcodes[index] = result;
    } else if (opcode === 3) {
      intcodes[param1] = result;
    } else if (opcode === 9) {
      relativeBase = result;
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

const UP = 0;
const RIGHT = 1;
const DOWN = 2;
// const LEFT = 3;

const paintingRobot = async (codes) => {
  const location = {
    x: 0,
    y: 0,
  };
  let direction = UP;

  const panels = {};

  let outputType = 0;
  const handleOutput = (output) => {
    // console.log('type', outputType, 'output', output);
    emitter.emit('output:received');
    if (outputType) {
      direction = (4 + direction + (output || -1)) % 4;
      const sign = direction === UP || direction === RIGHT ? 1 : -1;
      const xOrY = direction === UP || direction === DOWN ? 'y' : 'x';
      location[xOrY] += sign;
      // console.log('new direction', colorMap[direction], JSON.stringify(location));
    } else {
      panels[`${location.x},${location.y}`] = output;
      // console.log(`panels[${`${location.x},${location.y}`}] = ${output}`);
    }
    outputType ^= 1;
  };

  emitter.on('output:new', handleOutput);
  emitter.on('input:request', () => {
    // console.log('input request', panels[`${location.x},${location.y}`]);
    emitter.emit('input:response', panels[`${location.x},${location.y}`] || 0);
  });

  await processIntcodes(codes);
  console.log(Object.keys(panels).length);
};

paintingRobot(intcodes);

const fs = require('fs');

let score = 0;
let paddleXPosition = -1;
let ballXPosition = -1;

const screen = Array.from({ length: 25 }, () => Array.from({ length: 40 }, () => ' '));

/*
0 is an empty tile. No game object appears in this tile.
1 is a wall tile. Walls are indestructible barriers.
2 is a block tile. Blocks can be broken by the ball.
3 is a horizontal paddle tile. The paddle is indestructible.
4 is a ball tile. The ball moves diagonally and bounces off objects.
*/

const handleOutput = (() => {
  const outputs = [];
  return (num) => {
    outputs.push(num);
    if (outputs.length !== 3) return;
    const [x, y, val] = outputs;
    outputs.splice(0, 3);
    if (x === -1) {
      score = val;
      return;
    }
    switch (val) {
      case 0:
        screen[y][x] = ' ';
        break;
      case 1:
        screen[y][x] = 'X';
        break;
      case 2:
        screen[y][x] = '#';
        break;
      case 3:
        paddleXPosition = x;
        screen[y][x] = '-';
        break;
      case 4:
        ballXPosition = x;
        screen[y][x] = 'O';
        break;
      default:
        throw new Error('default reached');
    }
  };
})();

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
  3(a) {
    const lean = ballXPosition > paddleXPosition ? 1 : ballXPosition < paddleXPosition ? -1 : 0;
    return lean;
  },
  // GET
  4(a) {
    handleOutput(a);
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

const processIntcodes = (inputIntcodes) => {
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

    const result = ops[opcode](operand1, operand2);
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

intcodes[0] = 2;
processIntcodes(intcodes);

console.log(score);

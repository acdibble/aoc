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

  const panels = { '0,0': 1 };

  let outputType = 0;
  const handleOutput = (output) => {
    emitter.emit('output:received');
    if (outputType) {
      direction = (4 + direction + (output || -1)) % 4;
      const sign = direction === UP || direction === RIGHT ? 1 : -1;
      const xOrY = direction === UP || direction === DOWN ? 'y' : 'x';
      location[xOrY] += sign;
    } else {
      panels[`${location.x},${location.y}`] = output;
    }
    outputType ^= 1;
  };

  emitter.on('output:new', handleOutput);
  emitter.on('input:request', () => {
    emitter.emit('input:response', panels[`${location.x},${location.y}`] || 0);
  });

  await processIntcodes(codes);
  return panels;
};

const getDistance = (x1, y1, x2 = 0, y2 = 0) => Math.sqrt(((x1 - x2) ** 2) + ((y1 - y2) ** 2));

(async () => {
  const panels = await paintingRobot(intcodes);
  const panelsSortedByDistance = Object.keys(panels).sort((a, b) => (
    getDistance(...a.split(',')) > getDistance(...b.split(',')) ? -1 : 1
  ));
  const [smallestX, smallestY] = panelsSortedByDistance.reduce(([ex, why], current) => {
    const [x, y] = current.split(',').map(Number);
    if (x < ex && y < why) {
      return [x, y];
    }

    if (x < ex) {
      return [x, why];
    }

    if (y < why) {
      return [ex, y];
    }

    return [ex, why];
  }, [Infinity, Infinity])
    .map(Math.abs);
  const positivePanels = {};
  for (let i = 0; i < panelsSortedByDistance.length; i++) {
    const color = panels[panelsSortedByDistance[i]];
    const [x, y] = panelsSortedByDistance[i].split(',').map(Number);
    const posified = `${x + smallestX},${y + smallestY}`;
    positivePanels[posified] = color;
    panelsSortedByDistance[i] = posified;
  }
  const [largestX, largestY] = panelsSortedByDistance.reduce(([ex, why], current) => {
    const [x, y] = current.split(',').map(Number);
    if (x > ex && y > why) {
      return [x, y];
    }

    if (x > ex) {
      return [x, why];
    }

    if (y > why) {
      return [ex, y];
    }

    return [ex, why];
  }, [0, 0]);
  const canvas = Array.from({ length: largestY + 1 }, () => Array.from({ length: largestX + 1 }, () => ' '));
  for (const panel of panelsSortedByDistance) {
    const [x, y] = panel.split(',');
    canvas[largestY - y][x] = positivePanels[panel] ? '#' : ' ';
  }
  console.log(canvas.map((line) => line.join('')).join('\n'));
})();

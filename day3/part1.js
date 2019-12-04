const fs = require('fs');

const wires = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .trim()
  .split('\n')
  .map((wireDesc) => wireDesc.split(','));

const wireToSet = (wireSteps) => {
  const location = {
    x: 0,
    y: 0,
  }

  return wireSteps.reduce((acc, step) => {
    const direction = step[0];
    const sign = direction === 'U' || direction === 'R' ? 1 : -1;
    const xOrY = direction === 'U' || direction === 'D' ? 'y' : 'x';
    const distance = Number(step.slice(1));
    for (let i = 0; i < distance; i++) {
      location[xOrY] += sign;
      acc.add(`${location.x},${location.y}`);
    }
    return acc;
  }, new Set());
}

const wire1 = wireToSet(wires[0]);
const wire2 = wireToSet(wires[1]);

let distance = Infinity;

wire1.forEach((point) => {
  if (wire2.has(point)) {
    const [x, y] = point.split(',');
    const currentDistance = Math.abs(x) + Math.abs(y);
    if (currentDistance < distance) {
      distance = currentDistance;
    }
  }
});

console.log(distance);

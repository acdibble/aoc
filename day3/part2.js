const fs = require('fs');

const wires = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .trim()
  .split('\n')
  .map((wireDesc) => wireDesc.split(','));

const wireToObject = (wireSteps) => {
  const location = {
    x: 0,
    y: 0,
  };

  let numberOfSteps = 0;

  return wireSteps.reduce((acc, step) => {
    const direction = step[0];
    const sign = direction === 'U' || direction === 'R' ? 1 : -1;
    const xOrY = direction === 'U' || direction === 'D' ? 'y' : 'x';
    const distance = Number(step.slice(1));
    for (let i = 0; i < distance; i++) {
      location[xOrY] += sign;
      numberOfSteps += 1;
      const key = `${location.x},${location.y}`;
      if (!acc.hasOwnProperty(key)) {
        acc[key] = numberOfSteps;
      }
    }
    return acc;
  }, {});
};

const wire1 = wireToObject(wires[0]);
const wire2 = wireToObject(wires[1]);

const combinedStepsMap = Object.entries(wire1).reduce((acc, [loc, steps]) => {
  if (wire2[loc]) {
    acc[loc] = steps + wire2[loc];
  }
  return acc;
}, {});

console.log(Object.values(combinedStepsMap).reduce((acc, dist) => (dist < acc ? dist : acc), Infinity));

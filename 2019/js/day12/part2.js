/* eslint-disable no-nested-ternary */
const fs = require('fs');

const moons = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .trim()
  .split('\n')
  .map((moon) => {
    const parsed = moon.match(/[xyz]=(-?\d+)/g).reduce((acc, coord) => {
      const [xyz, value] = coord.split('=');
      acc[xyz] = Number(value);
      return acc;
    }, { x: null, y: null, z: null });

    return [parsed, { x: 0, y: 0, z: 0 }];
  });

const calculateVelocity = ({ x: x1, y: y1, z: z1 }, { x: x2, y: y2, z: z2 }) => {
  const xDelta = x1 === x2 ? [0, 0] : x1 < x2 ? [+1, -1] : [-1, 1];
  const yDelta = y1 === y2 ? [0, 0] : y1 < y2 ? [+1, -1] : [-1, 1];
  const zDelta = z1 === z2 ? [0, 0] : z1 < z2 ? [+1, -1] : [-1, 1];
  return [xDelta, yDelta, zDelta];
};

const velocityChanges = new Map(moons.map(([, vel]) => [vel, { x: [0], y: [0], z: [0] }]));

for (let step = 0; step < 1000000; step++) {
  for (let i = 0; i < moons.length; i++) {
    for (let j = i + 1; j < moons.length; j++) {
      const [[x1, x2], [y1, y2], [z1, z2]] = calculateVelocity(moons[i][0], moons[j][0]);
      const vel1 = moons[i][1];
      const vel2 = moons[j][1];
      vel1.x += x1;
      vel1.y += y1;
      vel1.z += z1;
      vel2.x += x2;
      vel2.y += y2;
      vel2.z += z2;
    }
  }
  for (const [pos, vel] of moons) {
    const tracker = velocityChanges.get(vel);
    tracker.x.push(vel.x);
    tracker.y.push(vel.y);
    tracker.z.push(vel.z);
    pos.x += vel.x;
    pos.y += vel.y;
    pos.z += vel.z;
  }
}

const getLCM2 = (x, y) => {
  const times = (x * y);
  while (y) {
    const t = y;
    y = x % y;
    x = t;
  }
  return times / x;
};

const getLCM = (...nums) => nums.reduce((acc, num) => getLCM2(acc, num));

const cycleTimes = [];

for (const [, vel] of moons) {
  const tracker = velocityChanges.get(vel);

  const cycleLengths = { x: -1, y: -1, z: -1 };
  for (const loc of ['x', 'y', 'z']) {
    const velocities = tracker[loc];
    let nextZero = 1;
    let done = false;
    while (!done && nextZero < velocities.length) {
      // console.log(nextZero);
      while (velocities[nextZero] !== 0) {
        nextZero += 1;
      }

      for (let i = 1, j = nextZero + 1; j < velocities.length; i++, j++) {
        if (velocities[i] !== velocities[j]) {
          nextZero += 1;
          break;
        } else if (velocities[i] === 0) {
          done = true;
          break;
        }
      }
    }

    cycleLengths[loc] = nextZero;
  }
  cycleTimes.push(getLCM(...Object.values(cycleLengths)));
}

console.log(getLCM(...cycleTimes));

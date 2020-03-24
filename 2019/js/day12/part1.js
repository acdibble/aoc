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

const getTotalEnergyForMoon = ([{ x: x1, y: y1, z: z1 }, { x: x2, y: y2, z: z2 }]) => (
  (Math.abs(x1) + Math.abs(y1) + Math.abs(z1)) * (Math.abs(x2) + Math.abs(y2) + Math.abs(z2))
);

for (let step = 0; step < 1000; step++) {
  for (let i = 0; i < moons.length; i++) {
    for (let j = i + 1; j < moons.length; j++) {
      const [[x1, x2], [y1, y2], [z1, z2]] = calculateVelocity(moons[i][0], moons[j][0]);
      moons[i][1].x += x1;
      moons[i][1].y += y1;
      moons[i][1].z += z1;
      moons[j][1].x += x2;
      moons[j][1].y += y2;
      moons[j][1].z += z2;
    }
  }
  for (const [pos, vel] of moons) {
    pos.x += vel.x;
    pos.y += vel.y;
    pos.z += vel.z;
  }
}

console.log(moons.reduce((acc, moon) => acc + getTotalEnergyForMoon(moon), 0));

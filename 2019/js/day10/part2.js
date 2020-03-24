const fs = require('fs');

const map = fs.readFileSync(`${__dirname}/data.txt`, 'utf8').trim().split('\n');

const asteroidCoordinates = [];

for (let i = 0; i < map.length; i++) {
  for (let j = 0; j < map[i].length; j++) {
    if (map[i][j] === '#') {
      asteroidCoordinates.push(`${j},${i}`);
    }
  }
}

const getDistance = (x1, y1, x2, y2) => Math.sqrt(((x1 - x2) ** 2) + ((y1 - y2) ** 2));

const destroyAsteroids = (x, y) => {
  const linesMap = {};
  const slopes = new Set();
  for (let j = 0; j < asteroidCoordinates.length; j++) {
    if (asteroidCoordinates[j] !== `${x},${y}`) {
      const [otherX, otherY] = asteroidCoordinates[j].split(',');
      const slope = (otherY - y) / (otherX - x);
      linesMap[slope] = (linesMap[slope] || []).concat([asteroidCoordinates[j]]);
      slopes.add(slope);
    }
  }

  const [pos, neg] = [...slopes].reduce(([p, n], num) => (
    num >= 0 ? [p.concat([num]), n] : [p, n.concat([num])]
  ), [[], []]);


  pos.sort((a, b) => (a < b ? -1 : 1));
  neg.sort((a, b) => (a < b ? -1 : 1));

  const sortedSlopes = [
    ...neg,
    ...pos,
  ];

  for (const slope of sortedSlopes) {
    linesMap[slope] = linesMap[slope].sort((a, b) => (
      getDistance(x, y, ...a.split(',')) < getDistance(x, y, ...b.split(',')) ? -1 : 1
    ));
  }

  const numberOfSlopes = sortedSlopes.length;
  let wasAsteroidDestroyed = true;
  let destroyedCount = 0;
  while (wasAsteroidDestroyed) {
    wasAsteroidDestroyed = false;
    for (let i = 0; i < numberOfSlopes; i++) {
      let asteroid = null;
      const slope = sortedSlopes[i];
      const asteroidsForSlope = linesMap[slope];
      if ((slope === Infinity || slope === -Infinity) && asteroidsForSlope.length > 0) {
        destroyedCount += 1;
        asteroid = asteroidsForSlope.shift();
        wasAsteroidDestroyed = true;
      } else if (asteroidsForSlope.length) {
        const closest = asteroidsForSlope.findIndex((coord) => coord.split(',')[0] >= x);
        if (closest !== -1) {
          [asteroid] = asteroidsForSlope.splice(closest, 1);
          wasAsteroidDestroyed = true;
          destroyedCount += 1;
        }
      }
      if (asteroid) console.log(destroyedCount, asteroid);
    }
    for (let i = 1; i < sortedSlopes.length - 1; i++) {
      let asteroid = null;
      const slope = sortedSlopes[i];
      const asteroidsForSlope = linesMap[slope];
      if ((slope === Infinity || slope === -Infinity) && asteroidsForSlope.length > 0) {
        destroyedCount += 1;
        asteroid = asteroidsForSlope.shift();
        wasAsteroidDestroyed = true;
      } else if (asteroidsForSlope.length) {
        const closest = asteroidsForSlope.findIndex((coord) => coord.split(',')[0] <= x);
        if (closest !== -1) {
          [asteroid] = asteroidsForSlope.splice(closest, 1);
          wasAsteroidDestroyed = true;
          destroyedCount += 1;
        }
      }
      if (asteroid) console.log(destroyedCount, asteroid);
    }
  }
  console.log(destroyedCount);
};

destroyAsteroids(...process.argv.slice(2).map(Number));

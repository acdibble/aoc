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

const numberOfVisibleAsteroids = asteroidCoordinates.reduce((acc, coordinate, i) => {
  const linesMap = {};
  const [x, y] = coordinate.split(',');
  for (let j = 0; j < asteroidCoordinates.length; j++) {
    if (j !== i) {
      const [otherX, otherY] = asteroidCoordinates[j].split(',');
      const slope = (x - otherX) / (y - otherY);
      linesMap[slope] = (linesMap[slope] || []).concat([asteroidCoordinates[j]]);
    }
  }

  const countForAsteroid = Object.entries(linesMap).reduce((currentCount, [slope, asteroids]) => {
    if (/Infinity/.test(slope) || asteroids.length === 1) {
      return currentCount + 1;
    }

    let higherFound = false;
    let lowerFound = false;
    for (const asteroid of asteroids) {
      const sign = Math.sign(slope);
      const [otherX, otherY] = asteroid.split(',');
      const diffX = x - otherX;
      const diffY = y - otherY;
      if (sign >= 0 && diffX >= 0 && diffY >= 0) {
        higherFound = true;
      } else if (sign < 0 && diffX < 0 && diffY >= 0) {
        higherFound = true;
      } else {
        lowerFound = true;
      }
    }

    return currentCount + Number(higherFound) + Number(lowerFound);
  }, 0);
  acc.push(countForAsteroid);
  return acc;
}, []);

const max = Math.max(...numberOfVisibleAsteroids);

console.log(asteroidCoordinates[numberOfVisibleAsteroids.indexOf(max)], max);

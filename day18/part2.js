const fs = require('fs');

const maze = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .trim()
  .split('\n')
  .map((line) => line.split(''));

const WALL_CHAR = '#';
const LOWER_A = 'a'.charCodeAt(0);

const XS = [1, 0, -1, 0];
const YS = [0, 1, 0, -1];
const START = 40;

maze[START][START] = WALL_CHAR;

for (let idx = 0; idx < 4; idx++) {
  maze[START + YS[idx]][START + XS[idx]] = WALL_CHAR;
  maze[START + YS[idx] + YS[3 - idx]][START + XS[idx] + XS[(idx + 3) % 4]] = idx;
}

// console.log(maze.map((line) => line.join('')).join('\n').replace(/\./g, ' '));

const parseMap = (map, startingChar) => {
  let start = null;
  for (let y = 0; y < map.length && !start; y++) {
    for (let x = 0; x < map[y].length && !start; x++) {
      if (map[y][x] === startingChar) {
        start = {
          x,
          y,
          steps: 0,
          doors: 0,
        };
      }
    }
  }

  if (!start) return null;


  const queue = [start];
  const visited = new Set();
  const mapInfo = {};

  while (queue.length) {
    const {
      x,
      y,
      steps,
      doors,
    } = queue.shift();

    for (let i = 0; i < 4; i++) {
      const neighborX = x + XS[i];
      const neighborY = y + YS[i];
      const char = map[neighborY][neighborX];
      if (char !== WALL_CHAR) {
        const vertex = {
          x: neighborX,
          y: neighborY,
          steps: steps + 1,
          doors: /[A-Z]/.test(char) ? doors | (1 << char.charCodeAt(0) - LOWER_A) : doors,
        };
        const key = `${neighborX},${neighborY}`;
        if (!visited.has(key)) {
          queue.push(vertex);
          visited.add(key);
          if (/[a-z]/.test(char)) {
            mapInfo[char] = vertex;
          }
        }
      }
    }
  }

  return mapInfo;
};

const KEYS = Array.from({ length: 26 }, (_, i) => String.fromCharCode(i + LOWER_A));

const subMaps = [0, 1, 2, 3].reduce((arr, num) => {
  arr.push(KEYS.reduce((acc, key) => {
    if (acc[key]) acc[key].info = parseMap(maze, key);
    return acc;
  }, parseMap(maze, num)));
  return arr;
}, []);

const cache = {};

const findShortestPath = (current, keyring, mapInfo, allKeys) => {
  const info = mapInfo[current] ? mapInfo[current].info : mapInfo;

  return KEYS.filter((key) => info[key] !== undefined).reduce((result, key) => {
    const letterNumber = key.charCodeAt(0) - LOWER_A;
    const keyAlreadyCollected = ((keyring >> letterNumber) & 1) === 1;
    if (keyAlreadyCollected) return result;

    const newKeyring = keyring | (1 << letterNumber);
    const stepsToKeyFromCurrent = info[key].steps;
    let shortestPathForRemainingKeys = newKeyring === allKeys ? 0 : cache[`${key},${newKeyring}`];
    if (shortestPathForRemainingKeys === undefined) {
      shortestPathForRemainingKeys = findShortestPath(key, newKeyring, mapInfo, allKeys);
      cache[`${key},${newKeyring}`] = shortestPathForRemainingKeys;
    }
    const distance = stepsToKeyFromCurrent + shortestPathForRemainingKeys;
    return Math.min(distance, result);
  }, Infinity);
};

console.log(subMaps.reduce((acc, mapInfo, i) => {
  const allKeysForQuadrant = Object.keys(mapInfo)
    .reduce((mask, key) => (mask | (1 << key.charCodeAt(0) - LOWER_A)), 0);
  const distance = findShortestPath(i, 0, mapInfo, allKeysForQuadrant);
  return acc + distance;
}, 0));

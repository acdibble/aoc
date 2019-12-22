const fs = require('fs');

const maze = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .trim()
  .split('\n')
  .map((line) => line.split(''));

const WALL_CHAR = '#';
const LOWER_A = 'a'.charCodeAt(0);

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

  const xs = [1, 0, -1, 0];
  const ys = [0, 1, 0, -1];

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
      const neighborX = x + xs[i];
      const neighborY = y + ys[i];
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

const mapInfo = KEYS.reduce((acc, key) => {
  acc[key].info = parseMap(maze, key);
  return acc;
}, parseMap(maze, '@'));

const ALL_KEYS = 0b11111111111111111111111111;
const cache = {};

const findShortestPath = (current, keyring) => {
  const info = mapInfo[current] ? mapInfo[current].info : mapInfo;

  return KEYS.reduce((result, key) => {
    const allNecessaryDoorsOpen = info[key].doors === (info[key].doors & keyring);
    const letterNumber = key.charCodeAt(0) - LOWER_A;
    const keyAlreadyCollected = ((keyring >> letterNumber) & 1) === 1;
    if (!allNecessaryDoorsOpen || keyAlreadyCollected) return result;

    const newKeyring = keyring | (1 << letterNumber);
    const stepsToKeyFromCurrent = info[key].steps;
    let shortestPathForRemainingKeys = newKeyring === ALL_KEYS ? 0 : cache[`${key},${newKeyring}`];
    if (shortestPathForRemainingKeys === undefined) {
      shortestPathForRemainingKeys = findShortestPath(key, newKeyring);
      cache[`${key},${newKeyring}`] = shortestPathForRemainingKeys;
    }
    const distance = stepsToKeyFromCurrent + shortestPathForRemainingKeys;
    return Math.min(distance, result);
  }, Infinity);
};

console.log(findShortestPath('@', 0));

const fs = require('fs');

const maze = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .split('\n')
  .map((line) => line.split(''));

const XS = [1, 0, -1, 0];
const YS = [0, 1, 0, -1];

const getNeighbor = (x, y, re, map) => {
  for (let i = 0; i < 4; i++) {
    const nextLine = map[y + YS[i]];
    const neighborX = x + XS[i];
    if (nextLine && re.test(nextLine[neighborX])) {
      return { x: x + XS[i], y: y + YS[i], char: nextLine[neighborX] };
    }
  }

  return { x: null, y: null, char: '' };
};

const PATHWAY = /\./;
const LETTER = /[A-Z]/;

const getKey = (x, y) => `${x},${y}`;
const getKeyWithName = (x, y, name) => `${name},${x},${y}`;

const findAllTiles = (map) => [...new Set(map
  .reduce((acc, line, y) => acc.concat(line.reduce((tiles, char, x) => {
    if (LETTER.test(char)) {
      const pair = getNeighbor(x, y, LETTER, map);
      const portalName = x < pair.x || y < pair.y
        ? `${char}${pair.char}`
        : `${pair.char}${char}`;
      let entrance = getNeighbor(x, y, PATHWAY, map);
      if (!entrance.char) {
        entrance = getNeighbor(pair.x, pair.y, PATHWAY, map);
      }
      const key = getKeyWithName(entrance.x, entrance.y, portalName);
      tiles.push(key);
    }
    return tiles;
  }, [])), []))].sort();

const [entrance, ...portalTiles] = findAllTiles(maze);

const tileDict = portalTiles.reduce((acc, tile, maskIndex, tiles) => {
  const [name, x, y] = tile.split(',');
  const pair = tiles.find((t) => t.startsWith(name) && t !== tile);
  const key = getKey(x, y);
  acc[key] = { name, maskIndex: BigInt(maskIndex), pair: { x: null, y: null } };
  if (pair) {
    const [, pairX, pairY] = pair.split(',');
    acc[key].pair = { x: Number(pairX), y: Number(pairY) };
  }
  return acc;
}, {});

const generateDistanceMap = (map, tileMap, startingTile) => {
  const [name, startingX, startingY] = startingTile.split(',');

  const queue = [{
    x: Number(startingX),
    y: Number(startingY),
    steps: -1,
    portals: 0,
  }];
  const visited = new Set([getKey(startingX, startingY)]);
  const mapInfo = {};

  while (queue.length) {
    const {
      x,
      y,
      steps,
      portals,
    } = queue.shift();

    const key = getKey(x, y);

    for (let i = 0; i < 4; i++) {
      const neighborX = x + XS[i];
      const neighborY = y + YS[i];
      const char = map[neighborY][neighborX];
      const neighborKey = getKey(neighborX, neighborY);
      let vertex = null;
      if (PATHWAY.test(char)) {
        vertex = {
          x: neighborX,
          y: neighborY,
          steps: steps + 1,
          portals,
        };
      }

      if (LETTER.test(char)) {
        const portal = tileMap[key];
        if (portal && portal.name !== name) {
          const { pair, maskIndex } = portal;
          const otherSide = { otherSide: getKeyWithName(pair.x, pair.y, portal.name) };
          mapInfo[getKeyWithName(x, y, portal.name)] = {
            x,
            y,
            steps: steps + 1,
            portals,
            maskIndex,
            ...!!pair.x && otherSide,
          };
        }
      }

      if (vertex && !visited.has(neighborKey)) {
        visited.add(neighborKey);
        queue.push(vertex);
      }
    }
  }
  return mapInfo;
};

const distanceMaps = [entrance, ...portalTiles].reduce((acc, tile) => {
  if (!tile.startsWith('ZZ')) {
    acc[tile] = generateDistanceMap(maze, tileDict, tile);
  }
  return acc;
}, {});

const exitKey = portalTiles[portalTiles.length - 1];

const findShortestPath = (source, portalTracker) => {
  const nextPortals = distanceMaps[source];

  return portalTiles.reduce((acc, tile) => {
    const nextPortal = nextPortals[tile];
    if (!nextPortal) return acc;
    const { maskIndex, steps, otherSide } = nextPortal;
    const portalAlreadyTaken = ((portalTracker >> maskIndex) & 1n) === 1n;
    if (portalAlreadyTaken) return acc;
    if (tile === exitKey) return Math.min(acc, steps);
    const stepsFromCurrentToNext = steps;
    const newPortalTracker = portalTracker | (1n << maskIndex);
    const dist = 1 + stepsFromCurrentToNext + findShortestPath(otherSide, newPortalTracker);
    return Math.min(dist, acc);
  }, Infinity);
};

console.log(findShortestPath(entrance, 0n));

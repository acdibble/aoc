import * as fs from 'fs';
import * as path from 'path';

const TREE = '#';

const map = fs.readFileSync(path.join(__dirname, 'data.txt'), 'utf8').split('\n');

const totalTreeCount = [
  [1, 1],
  [3, 1],
  [5, 1],
  [7, 1],
  [1, 2],
].reduce((acc, [run, rise]) => {
  let treeCount = 0;

  for (let x = 0, y = 0; y < map.length; x += run, y += rise) {
    if (map[y][x] === undefined) {
      map[y] = map[y].repeat(Math.ceil((x + 1) / map[y].length));
    }

    treeCount += Number(map[y][x] === TREE);
  }

  return acc * treeCount;
}, 1);

console.log(totalTreeCount);

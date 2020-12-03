import * as fs from 'fs';
import * as path from 'path';

const RISE = 1;
const RUN = 3;
const TREE = '#';

const map = fs.readFileSync(path.join(__dirname, 'data.txt'), 'utf8').split('\n');

let treeCount = 0;

for (let x = 0, y = 0; y < map.length; x += RUN, y += RISE) {
  if (map[y][x] === undefined) {
    map[y] = map[y].repeat(Math.ceil((x + 1) / map[y].length));
  }

  treeCount += Number(map[y][x] === TREE);
}

console.log(treeCount);

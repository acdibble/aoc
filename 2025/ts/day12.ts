import * as fs from 'fs/promises';

const lines = (await fs.readFile('data12.txt', 'utf8')).trimEnd().split('\n');

const areas: number[] = [];

for (let i = 0; i < 6; i += 1) {
  lines.shift();
  let area = 0;
  for (let j = 0; j < 3; j += 1) {
    area += lines
      .shift()!
      .matchAll(/#/g)
      .reduce((acc) => acc + 1, 0);
  }
  lines.shift();
  areas.push(area);
}

const result = lines.reduce((acc, line) => {
  const [x, y, ...presents] = line
    .matchAll(/\d\d/g)
    .map((m) => Number(m[0]))
    .toArray();
  const gridArea = x * y;
  const presentArea = presents.reduce((acc, count, i) => areas[i] * count + acc, 0);
  return gridArea >= presentArea ? acc + 1 : acc;
}, 0);

console.log(result);

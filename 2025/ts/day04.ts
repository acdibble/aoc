import { Grid } from './utils.ts';

const PAPER = '@';
const EMPTY = '.';

const grid = Grid.fromString(await Deno.readTextFile('./data04.txt'), {
  [PAPER]: PAPER,
  [EMPTY]: EMPTY,
} as const);

let part1 = 0;

for (const cell of grid) {
  if (cell.value !== PAPER) continue;

  let total = 0;

  for (const neighbor of cell.neighbors()) {
    if (neighbor.value === PAPER) total += 1;
  }

  if (total < 4) part1 += 1;
}

let changed = true;
let part2 = 0;
while (changed) {
  changed = false;

  for (const cell of grid) {
    if (cell.value !== PAPER) continue;
    let total = 0;

    for (const neighbor of cell.neighbors()) {
      if (neighbor.value === PAPER) total += 1;
    }

    if (total < 4) {
      cell.set(EMPTY);
      changed = true;
      part2 += 1;
    }
  }
}

console.log({ part1, part2 });

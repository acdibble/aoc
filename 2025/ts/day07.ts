import * as fs from 'fs/promises';
import { Cell, Grid } from './utils.js';

const EMPTY = '.';
const SPLITTER = '^';

const input = (await fs.readFile('./data07.txt', 'utf8')).trimEnd();

const startX = input.indexOf('S');

const grid = Grid.fromString(input, {
  [EMPTY]: EMPTY,
  [SPLITTER]: SPLITTER,
  S: EMPTY,
} as const);

const part1 = () => {
  let result = 0;

  const queue = [grid.at(startX, 0).unwrap()];
  const seen = new Set<number>();

  while (queue.length > 0) {
    const cell = queue.shift()!;

    if (seen.has(cell.getKey())) continue;

    seen.add(cell.getKey());

    // if (cell.isNone()) continue;
    const optionDown = cell.neighborDown();
    if (optionDown.isNone()) {
      continue;
    }

    const down = optionDown.unwrap();
    if (down.value === EMPTY) {
      queue.push(down);
      continue;
    }

    result += 1;

    const left = down.neightborLeft();

    if (left.isSome()) queue.push(left.unwrap());

    const right = down.neighborRight();

    if (right.isSome()) queue.push(right.unwrap());
  }

  return result;
};

const cache = new Map<number, number>();

const walk = (start: Cell<typeof EMPTY | typeof SPLITTER>) => {
  if (cache.has(start.getKey())) {
    return cache.get(start.getKey())!;
  }

  let location = start;

  while (location.value !== SPLITTER) {
    const downOption = location.neighborDown();
    if (downOption.isNone()) {
      return 1;
    }

    const down = downOption.unwrap();
    if (down.value === EMPTY) {
      location = down;
      continue;
    }

    location = down;
  }

  let result = 0;

  const leftOption = location.neightborLeft();
  if (leftOption.isSome()) {
    result += walk(leftOption.unwrap());
  }

  const rightOption = location.neighborRight();
  if (rightOption.isSome()) {
    result += walk(rightOption.unwrap());
  }

  cache.set(start.getKey(), result);

  return result;
};

console.log({ part1: part1(), part2: walk(grid.at(startX, 0).unwrap()) });

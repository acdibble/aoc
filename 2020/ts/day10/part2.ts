import { readFile } from "../utils.ts";

const numbers = (await readFile(import.meta.url)).split("\n").map(Number).sort((
  a,
  b,
) => a - b);

const map = [0, ...numbers].reverse().reduce((acc, current) => {
  acc[current] = [1, 2, 3].reduce(
    (count, offset) => count + (acc[current + offset] ?? 0),
    0,
  );
  return acc;
}, {
  [numbers[numbers.length - 1] + 3]: 1,
});

console.log(map[0]);

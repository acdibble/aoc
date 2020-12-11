import { readFile } from "../utils.ts";

const numbers = (await readFile(import.meta.url)).split("\n").map(Number).sort((
  a,
  b,
) => a - b);

const offsets = [1, 2, 3];

const result = [0, ...numbers].reverse().reduce((acc, current) => {
  acc[current] = offsets.reduce(
    (count, offset) => count + (acc[current + offset] ?? 0),
    0,
  );
  return acc;
}, {
  [numbers[numbers.length - 1] + 3]: 1,
})[0];

console.log(result);

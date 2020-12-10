import { readFile } from "../utils.ts";

const numbers = (await readFile(import.meta.url)).split("\n").map(Number).sort((
  a,
  b,
) => a - b);

let diff1 = 1;
let diff3 = 1;

numbers.reduce((previous, current) => {
  const diff = current - previous;
  if (diff === 3) diff3 += 1;
  if (diff === 1) diff1 += 1;
  return current;
});

console.log(diff1 * diff3);

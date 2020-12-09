import { readFile } from "../utils.ts";

const numbers = (await readFile(import.meta.url)).split("\n").map(Number);

let weakness = 0;

main:
for (let current = 25; current < numbers.length; current += 1) {
  for (let a = current - 1; a > current - 25; a -= 1) {
    const rem = numbers[current] - numbers[a];
    for (let b = current - 25; b < a; b += 1) {
      if (numbers[b] === rem) {
        continue main;
      }
    }
  }

  weakness = numbers[current];
  break;
}

for (let i = 0; i < numbers.length; i += 1) {
  let runningSum = numbers[i];
  let j = i + 1;

  for (j; runningSum < weakness; j += 1) {
    runningSum += numbers[j];
  }

  if (runningSum === weakness) {
    const contiguousNumbers = numbers.slice(i, j + 1);
    console.log(
      Math.min(...contiguousNumbers) + Math.max(...contiguousNumbers),
    );
    break;
  }
}

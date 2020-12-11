import { readFile } from "../utils.ts";

const numbers = (await readFile(import.meta.url)).split("\n").map(Number);

main:
for (let current = 25; current < numbers.length; current += 1) {
  for (let a = current - 1; a > current - 25; a -= 1) {
    const diff = numbers[current] - numbers[a];
    for (let b = current - 25; b < a; b += 1) {
      if (numbers[b] === diff) {
        continue main;
      }
    }
  }

  console.log(numbers[current]);
  break;
}

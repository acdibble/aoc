import * as fs from 'fs/promises';

const ranges = (await fs.readFile('data02.txt', 'utf8'))
  .trim()
  .split(',')
  .map((r) => {
    const [start, end] = r.split('-').map((n) => Number.parseInt(n));
    return { start, end };
  });

let part1 = 0;
let part2 = 0;

const regexps1: Record<number, RegExp> = {};
const regexps2: Record<number, RegExp> = {};

for (const { start, end } of ranges) {
  for (let current = start; current <= end; current++) {
    const str = current.toString();
    const half = str.toString().length / 2;

    regexps1[half] ??= new RegExp(String.raw`^(\d{${half}})\1$`);
    if (regexps1[half].test(str)) {
      part1 += current;
    }

    for (let i = 1; i <= half; i++) {
      if (str.length % i !== 0) continue;
      regexps2[i] ??= new RegExp(String.raw`^(\d{${i}})\1+$`);
      if (regexps2[i].test(str)) {
        part2 += current;
        break;
      }
    }
  }
}

console.log({ part1, part2 });

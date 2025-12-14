import * as fs from 'fs/promises';

const banks = (await fs.readFile('data03.txt', 'utf8'))
  .trim()
  .split('\n')
  .map((l) => l.split('').map(Number));

let part1 = 0;
let part2 = 0;

const cache: Record<string, number> = {};

const findLargestJoltage = (bank: number[], size: number): number => {
  const key = bank.join('') + size * 10;

  if (key in cache) return cache[key];

  let largest = 0;

  if (size === 1) {
    largest = Math.max(...bank);
  } else {
    for (let i = 0; i < bank.length; i++) {
      if (i + size > bank.length) break;

      const args = [bank.slice(i + 1), size - 1] as const;

      const largestFromRemaining = findLargestJoltage(...args);

      const candidate = bank[i] * 10 ** (size - 1) + largestFromRemaining;
      if (candidate > largest) {
        largest = candidate;
      }
    }
  }

  cache[key] = largest;
  return largest;
};

for (const bank of banks) {
  part1 += findLargestJoltage(bank, 2);
  part2 += findLargestJoltage(bank, 12);
}

console.log({ part1, part2 });

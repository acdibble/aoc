import { isNotNullish, Range } from './utils.ts';

const [rangeStrings, ingredients] = (await Deno.readTextFile('./data05.txt')).trim().split('\n\n');

const ranges = rangeStrings.split('\n').map((line) => {
  const [start, end] = line.split('-');
  return Range.inclusive(Number.parseInt(start), Number.parseInt(end));
});

const part1 = ingredients.split('\n').filter((ingredient) => {
  const num = Number.parseInt(ingredient);
  return ranges.some((range) => range.contains(num));
}).length;

const combineRanges = (ranges: Range[]): Range[] => {
  const combined: (Range | null)[] = [...ranges];
  let changed = false;

  for (let i = 0; i < combined.length; i++) {
    let a = combined[i];

    if (a === null) continue;

    for (let j = i + 1; j < combined.length; j++) {
      const b = combined[j];

      if (b === null) continue;

      const merged: Range | null = a.join(b);

      if (merged) {
        combined[i] = merged;
        combined[j] = null;
        a = merged;
        changed = true;
      }
    }
  }

  const compacted = combined.filter(isNotNullish);

  if (changed) return combineRanges(compacted);
  return compacted;
};

const part2 = combineRanges(ranges).reduce((sum, range) => sum + range.size, 0);

console.log({ part1, part2 });

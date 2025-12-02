const rotations = (await Deno.readTextFile('data01.txt'))
  .trim()
  .split('\n')
  .map((line) => Number.parseInt(line.replace('R', '').replace('L', '-')));

let dial = 50;
let part1 = 0;
let part2 = 0;

for (const rotation of rotations) {
  const change = Math.sign(rotation) * -1;

  for (let remaining = rotation; remaining !== 0; remaining += change) {
    dial += change;

    if (dial > 99) {
      dial = 0;
    } else if (dial < 0) {
      dial = 99;
    }

    if (dial === 0) part2 += 1;
  }

  if (dial === 0) part1 += 1;
}

console.log('part 1:', part1);
console.log('part 2:', part2);

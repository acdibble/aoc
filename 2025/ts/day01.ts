const rotations = (await Deno.readTextFile('data01.txt'))
  .trim()
  .split('\n')
  .map((line) => Number.parseInt(line.replace('R', '').replace('L', '-')));

const part1 = () => {
  let dial = 50;
  let result = 0;

  for (const step of rotations.map((n) => n % 100)) {
    if (dial + step > 99) {
      dial = dial + step - 100;
    } else if (dial + step < 0) {
      dial = 100 + (dial + step);
    } else {
      dial += step;
    }

    if (dial === 0) result += 1;
  }

  return result;
};

const part2 = () => {
  let dial = 50;
  let result = 0;

  for (const rotation of rotations) {
    const change = Math.sign(rotation) * -1;

    for (let remaining = rotation; remaining !== 0; remaining += change) {
      dial += change;

      if (dial > 99) {
        dial = 0;
      } else if (dial < 0) {
        dial = 99;
      }

      if (dial === 0) result += 1;
    }
  }

  return result;
};

console.log('part 1:', part1());
console.log('part 2:', part2());

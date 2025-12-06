const lines = (await Deno.readTextFile('./data06.txt')).trim().split('\n');

const part1 = () => {
  const problems = lines.map((line, y, arr) => {
    const split = line.trim().split(/\s+/);
    if (y === arr.length - 1) return split;
    return split.map((v) => Number.parseInt(v));
  });

  let result = 0;

  for (let x = 0; x < problems[0].length; x++) {
    const op = problems.at(-1)!.at(x)!;
    let total = op === '+' ? 0 : 1;
    for (let y = 0; y < problems.length - 1; y++) {
      switch (op) {
        case '+':
          total += problems[y][x] as number;
          break;
        case '*':
          total *= problems[y][x] as number;
          break;
        default:
          throw new Error(`Unknown operation: ${op}`);
      }
    }

    result += total;
  }

  return result;
};

const readNumber = (x: number): number | null => {
  let value = 0;

  let inNumber = false;

  for (let y = 0; y < lines.length - 1; y++) {
    const char = lines[y][x];

    if (inNumber && char === ' ') break;
    if (char === ' ') continue;

    inNumber = true;

    value *= 10;
    value += Number(char);
  }

  return inNumber ? value : null;
};

const part2 = () => {
  let result = 0;
  let x = 0;
  let total = 0;
  let sign;

  while (x < lines[0].length) {
    if (!sign) {
      sign = lines.at(-1)![x];
      if (sign === '*') total = 1;
    }

    const num = readNumber(x);

    if (num === null) {
      sign = undefined;
      result += total;
      total = 0;
    } else {
      switch (sign) {
        case '+':
          total += num;
          break;
        case '*':
          total *= num;
          break;
        default:
          throw new Error(`Unknown operation: ${sign}`);
      }
    }

    x += 1;
  }

  return result + total;
};

console.log({ part1: part1(), part2: part2() });

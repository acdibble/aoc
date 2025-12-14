import assert from 'assert';
import * as fs from 'fs/promises';
import { init, type Arith } from 'z3-solver';

const machines = (await fs.readFile('./data10.txt', 'utf8'))
  .trimEnd()
  .split('\n')
  .map((line) => {
    const [lightsString, ...rest] = line.split(' ');

    const lights = Number.parseInt(
      lightsString.slice(1, -1).replaceAll('.', '0').replaceAll('#', '1'),
      2,
    );

    const buttons: number[][] = [];
    const intButtons: number[] = [];

    for (const s of rest.slice(0, -1)) {
      const numbers = s
        .slice(1, -1)
        .split(',')
        .map((v) => Number(v));

      buttons.push(numbers);

      intButtons.push(
        Number.parseInt(
          lights
            .toString(2)
            .padStart(lightsString.length - 2, '0')
            .split('')
            .map((_, i) => (numbers.includes(i) ? 1 : 0))
            .join(''),
          2,
        ),
      );
    }

    const joltages = rest
      .at(-1)!
      .slice(1, -1)
      .split(',')
      .map((v) => Number(v));

    return { lights, intButtons, joltages, buttons };
  });

const part1 = () => {
  let queue: { state: number; presses: number }[];
  let result = 0;
  const seen = new Set<number>();

  for (const { lights, intButtons } of machines) {
    seen.clear();
    seen.add(lights);
    queue = intButtons.map((s) => ({ state: lights ^ s, presses: 1 }));
    queue.forEach(({ state }) => seen.add(state));

    while (!queue.some(({ state }) => state === 0)) {
      const existing = queue;
      queue = existing
        .flatMap(({ state, presses }) =>
          intButtons.map((s) => ({ state: state ^ s, presses: presses + 1 })),
        )
        .filter(({ state }) => !seen.has(state));
      queue.forEach(({ state }) => seen.add(state));
    }

    result += queue.find(({ state }) => state === 0)!.presses;
  }

  return result;
};

const part2 = async () => {
  const { Context } = await init();

  let result = 0;

  for (const machine of machines) {
    const { Optimize, Int } = Context('main');
    const optimizer = new Optimize();

    const variables = machine.buttons.map((_, i) => {
      const int = Int.const(`b${i}`);
      optimizer.add(int.ge(0));
      return int;
    });

    machine.joltages.forEach((joltage, jIndex) => {
      let condition: Arith<'main'> = Int.val(0);

      machine.buttons.forEach((button, bIndex) => {
        if (button.includes(jIndex)) {
          condition = condition.add(variables[bIndex]);
        }
      });

      optimizer.add(condition.eq(Int.val(joltage)));
    });

    const presses = variables.reduce((acc, v) => acc.add(v));
    optimizer.minimize(presses);

    assert((await optimizer.check()) === 'sat');

    result += Number(optimizer.model().eval(presses).toString());
  }

  return result;
};

console.log({ part1: part1(), part2: await part2() });

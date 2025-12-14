import * as fs from 'fs/promises';
import { Graph } from './utils.js';

const graph = (await fs.readFile('data11.txt', 'utf8'))
  .trimEnd()
  .split('\n')
  .reduce((acc, line) => {
    const it = line.matchAll(/\w\w\w/g);

    const [input] = it.next().value!;
    const node = acc.addNode(input);

    // console.log(input);
    for (const [output] of it) {
      // console.log(output);
      node.addChild(output);
    }
    // console.log();

    return acc;
  }, new Graph());

// console.log(lines);
const part1 = () => graph.getNode('you')!.countPaths('out');

const part2 = () => {
  const svr = graph.getNode('svr')!;
  const dac = graph.getNode('dac')!;
  const fft = graph.getNode('fft')!;

  return (
    svr.countPaths('dac') * dac.countPaths('fft') * fft.countPaths('out') +
    svr.countPaths('fft') * fft.countPaths('dac') * dac.countPaths('out')
  );
};

console.log({ part1: part1(), part2: part2() });

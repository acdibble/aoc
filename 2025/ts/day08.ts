import { Point3D } from './utils.ts';

const boxes = (await Deno.readTextFile('./data08.txt'))
  .trim()
  .split('\n')
  .map((line) => Point3D.fromString(line));

const distances = boxes
  .values()
  .flatMap((a, i) =>
    boxes
      .values()
      .drop(i + 1)
      .map((b) => [a, b, a.euclideanDistance(b)] as const),
  )
  .toArray()
  .sort(([, , distA], [, , distB]) => distA - distB);

const part1 = () => {
  const circuits = new Map(boxes.map((box) => [box, new Set([box])] as const));

  for (const [a, b] of distances.values().take(1000)) {
    const circuitA = circuits.get(a)!;
    if (circuitA.has(b)) {
      continue;
    }

    const combined = circuitA.union(circuits.get(b)!);

    for (const box of combined) {
      circuits.set(box, combined);
    }
  }

  return new Set(circuits.values())
    .values()
    .toArray()
    .sort((a, b) => b.size - a.size)
    .values()
    .take(3)
    .reduce((sum, circuit) => sum * circuit.size, 1);
};

const part2 = () => {
  const circuits = new Map(boxes.map((box) => [box, new Set([box])] as const));
  const uniqueCircuits = new Set(circuits.values());

  for (const [a, b] of distances) {
    const circuitA = circuits.get(a)!;
    if (circuitA.has(b)) {
      continue;
    }

    const circuitB = circuits.get(b)!;
    const combined = circuitA.union(circuitB);

    uniqueCircuits.delete(circuitA);
    uniqueCircuits.delete(circuitB);
    uniqueCircuits.add(combined);

    if (uniqueCircuits.size === 1) {
      return a.x * b.x;
    }

    for (const box of combined) {
      circuits.set(box, combined);
    }
  }

  throw new Error('unreachable');
};

console.log({ part1: part1(), part2: part2() });

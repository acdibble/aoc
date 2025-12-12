import { Point, Vector } from './utils.ts';

const points = (await Deno.readTextFile('./data09.txt'))
  .trimEnd()
  .split('\n')
  .map((l) => Point.fromString(l));

function* generateRectangles() {
  for (let i = 0; i < points.length; i++) {
    for (let j = i + 1; j < points.length; j++) {
      const a = points[i];
      const b = points[j];
      const width = Math.abs(a.x - b.x) + 1;
      const height = Math.abs(a.y - b.y) + 1;
      const area = width * height;

      yield { a, b, area };
    }
  }
}

const part1 = () => generateRectangles().reduce((acc, { area }) => (acc > area ? acc : area), 0);

const part2 = () => {
  const redAndGreenTiles = new Set<`${number},${number}`>();
  const borderTiles = new Set<`${number},${number}`>();

  const it = points.values();
  const it2 = [points[0]].values();

  for (
    let from = it.next().value, to = it.next().value;
    from && to;
    from = to, to = it.next().value ?? it2.next().value
  ) {
    let current = from;

    const direction = from.vectorTo(to);
    direction.x = Math.sign(direction.x);
    direction.y = Math.sign(direction.y);
    const outwards = direction.rotateCounterClockwise();

    while (!current.eq(to)) {
      const border = current.translate(outwards);
      if (!redAndGreenTiles.has(border.toString())) borderTiles.add(border.toString());
      redAndGreenTiles.add(current.toString());
      borderTiles.delete(current.toString());
      current = current.translate(direction);
    }

    borderTiles.add(current.translate(outwards).toString());
  }

  const vector = new Vector(1, 0);

  const vectors = [
    vector,
    vector.rotateClockwise(),
    vector.rotateClockwise().rotateClockwise(),
    vector.rotateClockwise().rotateClockwise().rotateClockwise(),
  ];

  // determine how far a corner can extend in each direction before leaving the
  // red and green tiles
  const maxLengths = new Map<`${number},${number}`, Record<`${number},${number}`, number>>();

  for (const point of points) {
    const lengths = {
      [vectors[0].toString()]: 0,
      [vectors[1].toString()]: 0,
      [vectors[2].toString()]: 0,
      [vectors[3].toString()]: 0,
    };
    maxLengths.set(point.toString(), lengths);
    for (const vector of vectors) {
      let length = 0;
      let current = point.translate(vector);

      while (!borderTiles.has(current.toString())) {
        length += 1;
        current = current.translate(vector);
      }

      lengths[vector.toString()] = length;
    }
  }

  return generateRectangles().reduce((acc, { a, b, area }) => {
    // skip rectangles that are lines
    if (a.x === b.x || a.y === b.y) return Math.max(area, acc);

    for (const { from, to } of [
      { from: a, to: b },
      { from: b, to: a },
    ]) {
      // calculate the distance and separate into x and y components
      const vector = from.vectorTo(to);
      const xVector = vector.zeroY();
      const yVector = vector.zeroX();
      const xStep = new Vector(Math.sign(xVector.x), 0);
      const yStep = new Vector(0, Math.sign(yVector.y));
      const lengths = maxLengths.get(from.toString())!;

      // check if we can extend far enough in each direction
      if (Math.abs(xVector.x) > lengths[xStep.toString()]) {
        return acc;
      }

      if (Math.abs(yVector.y) > lengths[yStep.toString()]) {
        return acc;
      }
    }

    return Math.max(acc, area);
  }, 0);
};

console.log({ part1: part1(), part2: part2() });

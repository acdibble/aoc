import { readFile } from "../utils.ts";
import {
  cloneCube,
  countCube,
  Cube,
  Hypercube,
  neighbors4D,
  padCube,
} from "./utils.ts";

const z0 = (await readFile(import.meta.url)).split("\n").map((line) =>
  line.split("").map((char) => char === "#" ? 1 : 0)
);

const cloneHypercube = (cube: Hypercube): Hypercube => cube.map(cloneCube);

const padHypercube = (cube: Hypercube): Hypercube => {
  const clone = cube.map(cloneCube).map(padCube);
  const blankNewCube = clone[0].map((cube) =>
    cube.map((line) => line.map(() => 0))
  ) as Cube;
  clone.unshift(blankNewCube);
  clone.push(blankNewCube);
  return clone;
};

let current = [[z0]];
for (let i = 0; i < 6; i++) {
  current = padHypercube(current);
  const next = cloneHypercube(current);
  for (let w = 0; w < next.length; w++) {
    const cube = current[w];
    for (let z = 0; z < next.length; z++) {
      const layer = cube[z];
      for (let y = 0; y < layer.length; y++) {
        const newLine: (0 | 1)[] = [];
        for (let x = 0; x < layer[y].length; x++) {
          let neighborCount = 0;
          for (const [xOffset, yOffset, zOffset, wOffset] of neighbors4D) {
            if (neighborCount > 3) break;
            neighborCount += current[w + wOffset]?.[z + zOffset]?.[y + yOffset]
              ?.[x + xOffset] ??
              0;
          }
          if (
            (neighborCount === 3 && layer[y][x] === 0) ||
            (neighborCount === 2 || neighborCount === 3) && layer[y][x] === 1
          ) {
            newLine.push(1);
          } else {
            newLine.push(0);
          }
        }
        next[w][z][y] = newLine;
      }
    }
  }

  current = next;
}

console.log(current.map(countCube).reduce((a, b) => a + b));

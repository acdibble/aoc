import { readFile } from "../utils.ts";
import { cloneCube, countCube, Cube, neighbors3D, padCube } from "./utils.ts";

const ACTIVE = "#";

const z0 = (await readFile(import.meta.url)).split("\n").map((line) =>
  line.split("").map((char) => char === ACTIVE ? 1 : 0)
);

let current = [z0];
for (let i = 0; i < 6; i++) {
  current = padCube(current);
  const next = cloneCube(current);
  for (let z = 0; z < next.length; z++) {
    const layer = current[z];
    for (let y = 0; y < layer.length; y++) {
      const newLine: (0 | 1)[] = [];
      for (let x = 0; x < layer[y].length; x++) {
        let neighborCount = 0;
        for (const [xOffset, yOffset, zOffset] of neighbors3D) {
          if (neighborCount > 3) break;
          neighborCount += current[z + zOffset]?.[y + yOffset]?.[x + xOffset] ??
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
      next[z][y] = newLine;
    }
  }
  current = next;
}

console.log(countCube(current));

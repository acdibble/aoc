import { Color, followDirections } from "./part1.ts";

type XYZ = [number, number, number];

const neighbors: XYZ[] = [
  [0, 1, -1], // NW
  [1, 0, -1], // NE
  [1, -1, 0], // E
  [0, -1, 1], // SE
  [-1, 0, 1], // SW
  [-1, 1, 0], // W
];

const getNeighbors = (key: string): string[] => {
  const [x, y, z] = key.split(",").map(Number) as XYZ;
  const output = [];
  for (let i = 0; i < 6; i++) {
    const [nx, ny, nz] = neighbors[i];
    output[i] = `${x + nx},${y + ny},${z + nz}`;
  }
  return output;
};

let tiles = await followDirections();

const neighborKeys: Record<string, string[]> = {};

for (let i = 0; i < 100; i++) {
  for (const key in tiles) {
    neighborKeys[key] ??= getNeighbors(key);
    for (const neighbor of neighborKeys[key]) {
      tiles[neighbor] ??= Color.White;
    }
  }

  const updated = { ...tiles };

  for (const key in updated) {
    neighborKeys[key] ??= getNeighbors(key);

    let blackNeighborCount = 0;
    for (const neighbor of neighborKeys[key]) {
      tiles[neighbor] ??= Color.White;
      blackNeighborCount += tiles[neighbor];
    }

    const current = tiles[key];
    if (current === Color.White && blackNeighborCount === 2) {
      updated[key] = Color.Black;
    } else if (
      current === Color.Black &&
      (blackNeighborCount === 0 || blackNeighborCount > 2)
    ) {
      updated[key] = Color.White;
    } else {
      updated[key] = current;
    }
  }

  tiles = updated;
}

console.log(Object.values(tiles).reduce((a, b) => a + b));

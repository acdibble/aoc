import { unreachable } from "https://deno.land/std@0.79.0/testing/asserts.ts";
import { readFile } from "../utils.ts";

type NorthSouth = "n" | "s";
type EastWest = "e" | "w";
type Direction = EastWest | `${NorthSouth}${EastWest}`;

export const loadDirections = async (): Promise<Direction[][]> =>
  (await readFile(import.meta.url)).split("\n").map((directions) =>
    directions.match(/([ns][ew]|[nsew])/g)! as Direction[]
  );

export const followDirections = async (): Promise<Record<string, Color>> => {
  const tiles: Record<string, Color> = {};
  const lines = await loadDirections();
  for (const directions of lines) {
    let x = 0;
    let y = 0;
    let z = 0;
    for (const direction of directions) {
      switch (direction) {
        case "e":
          x += 1;
          y -= 1;
          break;
        case "w":
          x -= 1;
          y += 1;
          break;
        case "nw":
          y += 1;
          z -= 1;
          break;
        case "se":
          y -= 1;
          z += 1;
          break;
        case "ne":
          x += 1;
          z -= 1;
          break;
        case "sw":
          x -= 1;
          z += 1;
          break;
        default:
          unreachable();
      }
    }

    const key = `${x},${y},${z}`;
    tiles[key] = ((tiles[key] ?? 0) + 1) % 2;
  }

  return tiles;
};

export enum Color {
  White,
  Black,
}

if (import.meta.main) {
  const tiles = await followDirections();

  console.log(Object.values(tiles).reduce((a, b) => a + b));
}

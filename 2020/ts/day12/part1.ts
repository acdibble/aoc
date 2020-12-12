import { readFile } from "../utils.ts";

type Action = "N" | "S" | "E" | "W" | "L" | "R" | "F";

const directions = (await readFile(import.meta.url)).split("\n").map((dir) => {
  const [, action, num] = /(\w)(\d+)/.exec(dir)!;

  return [action, Number.parseInt(num)];
}) as [Action, number][];

const EAST = 0;
const SOUTH = 90;
const NORTH = 270;

const shipState = {
  bearing: 0,
  northSouth: 0,
  eastWest: 0,
};

const moveForward = (units: number) => {
  const { bearing } = shipState;
  const mul = bearing === NORTH || bearing === EAST ? 1 : -1;
  const key = bearing === NORTH || bearing === SOUTH
    ? "northSouth"
    : "eastWest";
  shipState[key] += (units * mul);
};

const moveCardinal = (action: "N" | "S" | "E" | "W", units: number) => {
  const mul = action === "N" || action === "E" ? 1 : -1;
  const key = action === "N" || action === "S" ? "northSouth" : "eastWest";
  shipState[key] += (units * mul);
};

const turn = (action: "L" | "R", degrees: number) => {
  const mul = action === "R" ? 1 : -1;
  shipState.bearing = (360 + shipState.bearing + (degrees * mul)) % 360;
};

for (const [action, value] of directions) {
  if (action === "F") {
    moveForward(value);
  } else if (action === "R" || action === "L") {
    turn(action, value);
  } else {
    moveCardinal(action, value);
  }
}

console.log(Math.abs(shipState.northSouth) + Math.abs(shipState.eastWest));

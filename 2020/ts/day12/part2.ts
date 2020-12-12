import { readFile } from "../utils.ts";

type Action = "N" | "S" | "E" | "W" | "L" | "R" | "F";

const directions = (await readFile(import.meta.url)).split("\n").map((dir) => {
  const [, action, num] = /(\w)(\d+)/.exec(dir)!;

  return [action, Number.parseInt(num)];
}) as [Action, number][];

const shipState = {
  waypointEastWest: 10,
  waypointNorthSouth: 1,
  northSouth: 0,
  eastWest: 0,
};

const moveForward = (times: number) => {
  shipState.eastWest += times * shipState.waypointEastWest;
  shipState.northSouth += times * shipState.waypointNorthSouth;
};

const moveWaypoint = (action: "N" | "S" | "E" | "W", units: number) => {
  const mul = action === "N" || action === "E" ? 1 : -1;
  const key = action === "N" || action === "S"
    ? "waypointNorthSouth"
    : "waypointEastWest";
  shipState[key] += (units * mul);
};

const turn = (action: "L" | "R", degrees: number) => {
  let rightTurns = degrees / 90;
  if (action === "L") rightTurns = 4 - rightTurns;
  for (let i = 0; i < rightTurns; i += 1) {
    const temp = shipState.waypointNorthSouth;
    shipState.waypointNorthSouth = shipState.waypointEastWest * -1;
    shipState.waypointEastWest = temp;
  }
};

for (const [action, value] of directions) {
  if (action === "F") {
    moveForward(value);
  } else if (action === "R" || action === "L") {
    turn(action, value);
  } else {
    moveWaypoint(action, value);
  }
}

console.log(Math.abs(shipState.northSouth) + Math.abs(shipState.eastWest));

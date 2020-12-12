import { readFile } from "../utils.ts";

type Tile = "#" | "." | "L";

const seats = (await readFile(import.meta.url)).split("\n").map((line) =>
  line.split("")
) as Tile[][];

const cloneSeats = (tiles: Tile[][]): Tile[][] =>
  tiles.map((line) => [...line]);

const neighbors = [
  [-1, -1],
  [-1, 0],
  [-1, 1],
  [0, -1],
  [0, 1],
  [1, -1],
  [1, 0],
  [1, 1],
];

const getNewValue = (board: Tile[][], x: number, y: number): Tile => {
  const current = board[y][x];
  if (current === ".") return ".";

  const occupiedSeats = neighbors.reduce((acc, [xOffset, yOffset]) => {
    let adjustedX = x;
    let adjustedY = y;
    let neighbor: Tile;
    do {
      adjustedX += xOffset;
      adjustedY += yOffset;
      neighbor = board[adjustedY]?.[adjustedX];
    } while (neighbor === ".");
    return acc + Number(neighbor === "#");
  }, 0);

  if (current === "L" && occupiedSeats === 0) return "#";
  if (current === "#" && occupiedSeats >= 5) return "L";
  return current;
};

let aSeatChanged = true;
let currentSeats = cloneSeats(seats);
let iteration: number;

for (iteration = 0; aSeatChanged; iteration += 1) {
  aSeatChanged = false;
  const nextSeats = cloneSeats(currentSeats);

  for (let y = 0; y < currentSeats.length; y += 1) {
    for (let x = 0; x < currentSeats[y].length; x += 1) {
      nextSeats[y][x] = getNewValue(currentSeats, x, y);
      aSeatChanged ||= nextSeats[y][x] !== currentSeats[y][x];
    }
  }

  currentSeats = nextSeats;
}

console.log(
  currentSeats.map((line) => line.join("")).join("").match(/#/g)?.length,
);

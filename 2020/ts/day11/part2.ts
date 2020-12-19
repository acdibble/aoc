import { readFile } from "../utils.ts";

type Tile = "#" | "." | "L";

let seats = (await readFile(import.meta.url)).split("\n").map((line) =>
  line.split("")
) as Tile[][];

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
let result = 0;

for (let iteration = 0; aSeatChanged; iteration += 1) {
  result = 0;
  aSeatChanged = false;
  seats = seats.map((line, y) =>
    line.map((oldChar, x) => {
      const newChar = getNewValue(seats, x, y);
      aSeatChanged ||= newChar !== oldChar;
      result += Number(newChar === "#");
      return newChar;
    })
  );
}

console.log(result);

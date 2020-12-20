import { findCorners, linkTiles, rotateImage, Tile } from "./part1.ts";

const tiles = await linkTiles();

const corners = findCorners(tiles);

let rowStart: Tile | undefined = corners.find((c) => c.left === c.top)!;
let current: Tile | undefined = rowStart;

let picture: string[] = [];
let offset = 0;

while (current) {
  current.data.slice(1, -1).map((l) => l.slice(1, -1)).forEach((line, i) => {
    picture[offset + i] ||= "";
    picture[offset + i] += line;
  });
  const next = current.right ?? current.top?.right?.bottom ??
    current.bottom?.right?.top;
  if (!next) {
    offset += 8;
    rowStart = rowStart?.bottom;
    current = rowStart;
  } else {
    current = next;
  }
}

const re1 = /(?<=.{18})#(?=.)/g;
const re2 = /#....##....##....###/;
const re3 = /.#..#..#..#..#..#.../;

let matchesFound = 0;
for (let n = 0; n < 4; n++) {
  picture = rotateImage(picture);
  for (let k = 0; k < 2; k++) {
    if (k === 1) {
      picture = picture.map((l) => l.split("").reverse().join(""));
    }

    for (let i = 0; i < picture.length - 1; i++) {
      for (const match of picture[i].matchAll(re1)) {
        if (
          re2.test(picture[i + 1].slice(match.index! - 18, match.index! + 2)) &&
          re3.test(picture[i + 2].slice(match.index! - 18, match.index! + 2))
        ) {
          matchesFound += 1;
        }
      }
    }
  }
  if (matchesFound > 0) break;
}

console.log(picture.join("").replace(/[^#]/g, "").length - matchesFound * 15);

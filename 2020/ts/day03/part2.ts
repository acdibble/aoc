import { path } from "../deps.ts";

const TREE = "#";

const map = (await Deno.readTextFile(
  path.join(path.fromFileUrl(path.dirname(import.meta.url)), "data.txt"),
)).split("\n");

const totalTreeCount = [
  [1, 1],
  [3, 1],
  [5, 1],
  [7, 1],
  [1, 2],
].reduce((acc, [run, rise]) => {
  let treeCount = 0;

  for (let x = 0, y = 0; y < map.length; x += run, y += rise) {
    if (map[y][x] === undefined) {
      map[y] = map[y].repeat(Math.ceil((x + 1) / map[y].length));
    }

    treeCount += Number(map[y][x] === TREE);
  }

  return acc * treeCount;
}, 1);

console.log(totalTreeCount);

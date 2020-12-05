import { path } from "../deps.ts";

const RISE = 1;
const RUN = 3;
const TREE = "#";

const map = (await Deno.readTextFile(
  path.join(path.fromFileUrl(path.dirname(import.meta.url)), "data.txt"),
)).split("\n");

let treeCount = 0;

for (let x = 0, y = 0; y < map.length; x += RUN, y += RISE) {
  if (map[y][x] === undefined) {
    map[y] = map[y].repeat(Math.ceil((x + 1) / map[y].length));
  }

  treeCount += Number(map[y][x] === TREE);
}

console.log(treeCount);

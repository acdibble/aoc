import { readFile } from "../utils.ts";

const sum = (await readFile(import.meta.url)).split("\n\n").reduce(
  (acc, group) => acc + new Set(group.match(/[a-z]/g)).size,
  0,
);

console.log(sum);

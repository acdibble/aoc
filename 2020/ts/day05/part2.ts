import { path } from "../deps.ts";

export const calculateId = (seat: string): number =>
  Number.parseInt(
    seat.replace(/[BR]/g, "1").replace(/[FL]/g, "0"),
    2,
  );

const ids = (await Deno.readTextFile(
  path.join(path.fromFileUrl(path.dirname(import.meta.url)), "data.txt"),
))
  .split("\n")
  .map(calculateId);

const expectedSum = (Math.min.apply(null, ids) + Math.max.apply(null, ids)) *
  (ids.length + 1) / 2;

console.log(expectedSum - ids.reduce((a, b) => a + b));

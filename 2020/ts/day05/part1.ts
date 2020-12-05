import { path } from "../deps.ts";

export const calculateId = (seat: string): number =>
  Number.parseInt(
    seat.replace(/[BR]/g, "1").replace(/[FL]/g, "0"),
    2,
  );

const highest = Math.max.apply(
  null,
  (await Deno.readTextFile(
    path.join(path.fromFileUrl(path.dirname(import.meta.url)), "data.txt"),
  ))
    .split("\n")
    .map(calculateId),
);

console.log(highest);

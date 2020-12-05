import { unreachable } from "https://deno.land/std@0.79.0/testing/asserts.ts";
import { path } from "../deps.ts";

export const calculateId = (seat: string): number =>
  Number.parseInt(
    seat.replace(/[BR]/g, "1").replace(/[FL]/g, "0"),
    2,
  );

const tickets = (await Deno.readTextFile(
  path.join(path.fromFileUrl(path.dirname(import.meta.url)), "data.txt"),
))
  .split("\n")
  .sort((a, b) => {
    for (let i = 0; i < a.length; i += 1) {
      if (a[i] === "B" && b[i] === "F") return -1;
      if (b[i] === "B" && a[i] === "F") return 1;
      if (a[i] === "R" && b[i] === "L") return -1;
      if (b[i] === "R" && a[i] === "L") return 1;
    }
    unreachable();
  });

let previous = calculateId(tickets[0]);

for (let i = 1; i < tickets.length; i += 1) {
  const current = calculateId(tickets[i]);
  if (previous !== current + 1) {
    console.log(current + 1);
    break;
  }
  previous = current;
}

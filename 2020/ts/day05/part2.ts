import { path } from "../deps.ts";

export const calculateId = (seat: string): number =>
  Number.parseInt(
    seat.replace(/[BR]/g, "1").replace(/[FL]/g, "0"),
    2,
  );

const [actualSum, seatCount, lowestId, highestId] = (await Deno.readTextFile(
  path.join(path.fromFileUrl(path.dirname(import.meta.url)), "data.txt"),
))
  .split("\n")
  .reduce(
    ([total, count, min, max], seat) => {
      const id = calculateId(seat);
      return [total + id, count + 1, Math.min(min, id), Math.max(max, id)];
    },
    [0, 1, Infinity, -Infinity],
  );

const expectedSum = (lowestId + highestId) * (seatCount) / 2;

console.log(expectedSum - actualSum);

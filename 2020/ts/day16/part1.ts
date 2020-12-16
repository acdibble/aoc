import { readFile } from "../utils.ts";

const ticketInfo = await readFile(import.meta.url);

const createRange = (inputString: string) => {
  const [start, end] = inputString.split("-").map(Number);

  return Array.from({ length: end - start + 1 }, (_, i) => start + i);
};

const validNumbers = new Set(
  [...ticketInfo.matchAll(/\d+-\d+/g)].flatMap(([range]) => createRange(range)),
);

const invalidTickets = [...ticketInfo.matchAll(/\n([\d,]+)/g)]
  .flatMap(([, match]) => match.split(",").map(Number))
  .reduce((acc, num) => acc + (validNumbers.has(num) ? 0 : num), 0);

console.log(invalidTickets);

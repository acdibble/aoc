import { GoodSet, readFile } from "../utils.ts";

const ticketInfo = await readFile(import.meta.url);

const createRange = (inputString: string) => {
  const [start, end] = inputString.split("-").map(Number);

  return Array.from({ length: end - start + 1 }, (_, i) => start + i);
};

const typeSet = [...ticketInfo.matchAll((/([\w ]+): (.+?) or (.+?)\n/g))]
  .reduce<Record<string, Set<number>>>((acc, match) => {
    const [, label, range1, range2] = match;
    acc[label] = new Set([...createRange(range1), ...createRange(range2)]);
    return acc;
  }, {});

const validNumbers = new Set(Object.values(typeSet).flatMap((set) => [...set]));

const tickets: number[][] = [...ticketInfo.matchAll(/\n([\d,]+)/g)]
  .map(([, match]) => match.split(",").map(Number))
  .filter((ticket) => ticket.every((num) => validNumbers.has(num)));

const ticketSets = tickets.map((line) =>
  line.reduce<Record<number, GoodSet<string>>>((acc, number, i) => {
    acc[i] = Object.entries(typeSet).reduce((acc2, [key, set]) => {
      if (set.has(number)) acc2.add(key);
      return acc2;
    }, new GoodSet<string>());
    return acc;
  }, {})
);

const myTicket = tickets[0];

const labelSets = myTicket.map((_, i) =>
  ticketSets.reduce((acc, sets, j) => {
    if (j === 0) return sets[i];
    return sets[i].intersection(acc);
  }, new GoodSet<string>())
);

let run = true;
while (run) {
  run = false;
  for (const set of labelSets) {
    if (set.size === 1) {
      for (let i = 0; i < labelSets.length; i++) {
        if (labelSets[i].size !== 1) {
          labelSets[i] = labelSets[i].difference(set);
          run = true;
        }
      }
    }
  }
}

const result = labelSets.map((set) => [...set][0]).reduce(
  (acc, label, i) => acc * (/departure/.test(label) ? myTicket[i] : 1),
  1,
);

console.log(result);

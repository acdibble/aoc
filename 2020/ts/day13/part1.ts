import { readFile } from "../utils.ts";

const [departure, busNumbers] = (await readFile(import.meta.url)).split("\n");

const timestamp = Number(departure);

const busses = busNumbers.split(",").map(Number).filter(Number.isInteger);

const result = busses.reduce((acc, bus) => {
  const cycleNumber = Math.ceil(timestamp / bus);
  const diff = (cycleNumber * bus) - timestamp;
  if (acc[1] > diff) {
    return [bus, diff];
  }
  return acc;
}, [0, Infinity]);

console.log(result[0] * result[1]);

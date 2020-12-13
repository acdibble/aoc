import { readFile } from "../utils.ts";

const busses = (await readFile(import.meta.url)).split("\n")[1].split(",")
  .reduce<[bigint, bigint][]>((acc, b, i) => {
    if (b !== "x") acc.push([BigInt(b), BigInt(i)]);
    return acc;
  }, []);

const gcd = (a: bigint, b: bigint): bigint => (b === 0n ? a : gcd(b, a % b));

const lcm = (a: bigint, b: bigint): bigint => a * b / gcd(a, b);

const lcmAll = (numbers: bigint[]): bigint => numbers.reduce(lcm);

let bussesToInclude = 2;
let increment = 1n;
let timestamp = 0n;

main:
while (bussesToInclude <= busses.length) {
  timestamp += increment;

  for (let i = 0; i < bussesToInclude; i++) {
    const [bus, offset] = busses[i];
    if ((timestamp + offset) % bus !== 0n) continue main;
  }

  increment = lcmAll(busses.slice(0, bussesToInclude).map(([b]) => b));
  bussesToInclude += 1;
}

console.log(timestamp);

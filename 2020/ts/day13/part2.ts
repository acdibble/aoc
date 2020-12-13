import { readFile } from "../utils.ts";

const busses = (await readFile(import.meta.url)).split("\n")[1].split(",")
  .reduce<[bigint, bigint][]>((acc, b, i) => {
    if (b !== "x") acc.push([BigInt(b), BigInt(i)]);
    return acc;
  }, []);

const gcd = (a: bigint, b: bigint): bigint => (b === 0n ? a : gcd(b, a % b));

const lcm = (a: bigint, b: bigint): bigint => a * b / gcd(a, b);

let bussesToInclude = 2;
let increment = 1n;
let timestamp = 0n;

main:
while (bussesToInclude <= busses.length) {
  timestamp += increment;

  let newIncrement = 1n;
  for (let i = 0; i < bussesToInclude; i++) {
    const [bus, offset] = busses[i];
    if ((timestamp + offset) % bus !== 0n) continue main;
    newIncrement = lcm(bus, newIncrement);
  }

  increment = newIncrement;
  bussesToInclude += 1;
}

console.log(timestamp);

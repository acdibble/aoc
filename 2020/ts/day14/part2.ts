import { readFile } from "../utils.ts";

const program = (await readFile(import.meta.url)).split("\n");

const maskRegExp = /^mask = ([X01]+)/;
const memRegExp = /^mem\[(?<address>\d+)\] = (?<value>\d+)$/;

const memory: Record<string, number> = {};

let mask = "";

for (const line of program) {
  if (maskRegExp.test(line)) {
    const [, newMask] = maskRegExp.exec(line)!;
    mask = newMask;
  } else if (memRegExp.test(line)) {
    const { groups } = memRegExp.exec(line)!;
    const addresses = [BigInt(groups!.address)];
    for (let i = 35n; i >= 0n; i--) {
      const addressCount = addresses.length;
      for (let j = 0; j < addressCount; j++) {
        const bit = mask[35 - Number(i)];
        if (bit === "1") {
          addresses[j] |= (1n << i);
        } else if (bit === "X") {
          let newVal = addresses[j];
          addresses[j] &= ~(1n << i);
          newVal |= (1n << i);
          addresses.push(newVal);
        }
      }
    }
    const value = Number(groups!.value);
    for (const address of addresses) {
      memory[address.toString()] = value;
    }
  }
}

console.log(Object.values(memory).reduce((acc, current) => acc + current, 0));

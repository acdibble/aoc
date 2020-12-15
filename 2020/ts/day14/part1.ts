import { readFile } from "../utils.ts";

const program = (await readFile(import.meta.url)).split("\n");

const maskRegExp = /^mask = ([X01]+)/;
const memRegExp = /^mem\[(?<address>\d+)\] = (?<value>\d+)$/;

const memory: Record<string, bigint> = {};

let mask = "";

const bitsToBigInt = (number: number): bigint =>
  number.toString(2)
    .padStart(36, "0")
    .split("")
    .reduce((acc, char) => {
      acc <<= 1n;
      acc |= BigInt(char);
      return acc;
    }, 0n);

for (const line of program) {
  if (maskRegExp.test(line)) {
    const [, newMask] = maskRegExp.exec(line)!;
    mask = newMask;
  } else if (memRegExp.test(line)) {
    const { groups } = memRegExp.exec(line)!;
    let value = bitsToBigInt(Number(groups!.value));
    const { address } = groups!;
    for (let i = 0; i < mask.length; i++) {
      if (mask[i] === "1") {
        value |= (1n << BigInt(35 - i));
      } else if (mask[i] === "0") {
        value &= ~(1n << BigInt(35 - i));
      }
    }
    memory[address] = value;
  }
}

console.log(Object.values(memory).reduce((a, b) => a + b));

import { readFile } from "../utils.ts";

const program = (await readFile(import.meta.url)).split("\n");

const maskRegExp = /^mask = ([X01]+)/;
const memRegExp = /^mem\[(?<address>\d+)\] = (?<value>\d+)$/;

const memory: Record<string, string[]> = {};

let mask = "";

for (const line of program) {
  if (maskRegExp.test(line)) {
    const [, newMask] = maskRegExp.exec(line)!;
    mask = newMask;
  } else if (memRegExp.test(line)) {
    const { groups } = memRegExp.exec(line)!;
    const value = Number(groups!.value).toString(2).padStart(36, "0").split("");
    const { address } = groups!;
    for (let i = 0; i < mask.length; i++) {
      if (mask[i] !== "X") value[i] = mask[i];
    }
    memory[address] = value;
  }
}

console.log(
  Object.values(memory).reduce(
    (acc, current) => acc + Number.parseInt(current.join(""), 2),
    0,
  ),
);

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
    const addresses = [
      Number(groups!.address).toString(2).padStart(36, "0").split(""),
    ];
    for (let i = 0; i < mask.length; i++) {
      const addressCount = addresses.length;
      for (let j = 0; j < addressCount; j++) {
        if (mask[i] === "1") {
          addresses[j][i] = mask[i];
        } else if (mask[i] === "X") {
          const newVal = [...addresses[j]];
          addresses[j][i] = "0";
          newVal[i] = "1";
          addresses.push(newVal);
        }
      }
    }
    const value = Number(groups!.value);
    for (const address of addresses) {
      memory[address.join("")] = value;
    }
  }
}


console.log(
  Object.values(memory).reduce(
    (acc, current) => acc + current,
    0,
  ),
);

import { unreachable } from "https://deno.land/std@0.79.0/testing/asserts.ts";
import { readFile } from "../utils.ts";

const program: ["nop" | "jmp" | "acc", number][] =
  (await readFile(import.meta.url)).split("\n").map(
    (instruction) => {
      const [op, value] = instruction.split(" ");
      const int = Number.parseInt(value);
      if (Number.isNaN(int)) throw new Error("nan");
      return [op as "nop" | "jmp" | "acc", int];
    },
  );

let pc = 0;
let acc = 0;

const seenPcs = new Set<number>();

while (!seenPcs.has(pc)) {
  seenPcs.add(pc);
  const [op, int] = program[pc];
  switch (op) {
    // deno-lint-ignore no-fallthrough
    case "acc":
      acc += int;
    case "nop":
      pc += 1;
      break;
    case "jmp":
      pc += int;
      break;
    default:
      unreachable();
  }
}

console.log(acc);

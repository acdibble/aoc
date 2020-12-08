import { unreachable } from "https://deno.land/std@0.79.0/testing/asserts.ts";
import { readFile } from "../utils.ts";

type Instruction = "nop" | "jmp" | "acc";

const program: [Instruction, number][] = (await readFile(import.meta.url))
  .split("\n").map(
    (instruction) => {
      const [op, value] = instruction.split(" ");
      const int = Number.parseInt(value);
      if (Number.isNaN(int)) throw new Error("nan");
      return [op as Instruction, int];
    },
  );

for (let i = program.length - 1; i >= 0; i -= 1) {
  const old = program[i][0];

  if (program[i][0] === "jmp") {
    program[i][0] = "nop";
  } else if (program[i][0] === "nop") {
    program[i][0] = "jmp";
  } else {
    continue;
  }

  let pc = 0;
  let acc = 0;
  const seenPcs = new Set<number>();

  while (!seenPcs.has(pc) && pc !== program.length) {
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

  if (pc === program.length) {
    console.log(acc);
    break;
  }

  program[i][0] = old;
}

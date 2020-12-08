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

const runProgram = (instructions: [Instruction, number][]): number | null => {
  let pc = 0;
  let acc = 0;

  const seenPcs = new Set<number>();

  while (!seenPcs.has(pc) && pc !== instructions.length) {
    seenPcs.add(pc);
    const [op, int] = instructions[pc];
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

  return pc === instructions.length ? acc : null;
};

const runModified = (index: number, instr: Instruction): number | null => {
  const modified = program.map((op) => [...op] as [Instruction, number]);
  modified[index][0] = instr;
  return runProgram(modified);
};

for (let i = program.length - 1; i >= 0; i -= 1) {
  let result: number | null = null;

  if (program[i][0] === "jmp") {
    result = runModified(i, "nop");
  } else if (program[i][0] === "nop") {
    result = runModified(i, "jmp");
  }

  if (result !== null) {
    console.log(result);
    break;
  }
}

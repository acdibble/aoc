import { readFile } from "../utils.ts";

const inputs = (await readFile(import.meta.url)).split("\n");

const op = /(\d+ [*+] \d+)/;
const parens = /\(([^())]+)\)/;

const evalRe = (_: string, match: string): string => eval(match);

const evaluate = (input: string): number => {
  let current = input;

  while (parens.test(current)) {
    current = current.replace(parens, (_, m) => String(evaluate(m)));
  }

  while (op.test(current)) {
    current = current.replace(op, evalRe);
  }

  return Number(current);
};

console.log(inputs.reduce((acc, line) => acc + evaluate(line), 0));

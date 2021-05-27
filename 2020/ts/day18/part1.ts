import { readFile } from "../utils.ts";

const inputs = (await readFile(import.meta.url)).split("\n");

class Parser {
  static isDigit(char: string): boolean {
    return /\d/.test(char);
  }

  private current = 0;
  private readonly input: string;

  constructor(input: string) {
    this.input = input.replace(/ /g, "");
  }

  private isAtEnd(): boolean {
    return this.current === this.input.length;
  }

  private advance(): string {
    return this.input[this.current++];
  }

  private peek(): string {
    return this.input[this.current];
  }

  private match(char: string): boolean {
    if (this.peek() === char) {
      this.advance();
      return true;
    }

    return false;
  }

  private number(): number {
    if (Parser.isDigit(this.peek())) {
      return Number(this.advance());
    } else if (this.match("(")) {
      return this.evaluate();
    } else {
      throw new Error("unreachable");
    }
  }

  evaluate(): number {
    let total = this.number();

    while (!this.isAtEnd() && !this.match(")")) {
      const op = this.advance();
      const number = this.number();

      if (op === "+") {
        total += number;
      } else if (op === "*") {
        total *= number;
      } else {
        throw new Error("unreachable");
      }
    }

    return total;
  }
}

console.log(inputs.reduce((acc, line) => acc + new Parser(line).evaluate(), 0));

import { readFile } from "../utils.ts";

const inputs = (await readFile(import.meta.url)).split("\n");

class Parser {
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

  private consume(char: string): void {
    if (!this.match(char)) {
      throw new Error(`expected '${char}', got '${this.peek()}'.`);
    }
  }

  private check(arg0: RegExp | string): boolean {
    if (this.isAtEnd()) return false;
    return arg0 instanceof RegExp
      ? arg0.test(this.peek())
      : this.peek() === arg0;
  }

  private match(char: string): boolean {
    if (!this.isAtEnd() && this.peek() === char) {
      this.advance();
      return true;
    }

    return false;
  }

  private number(): number {
    let number: number;
    if (this.check(/\d/)) {
      number = Number(this.advance());
    } else if (this.match("(")) {
      number = this.evaluate();
      this.consume(")");
    } else {
      throw new Error("unreachable");
    }

    return number;
  }

  private sum(): number {
    let number = this.number();

    while (this.match("+")) number += this.number();

    return number;
  }

  private product(): number {
    let number = this.sum();

    while (this.match("*")) number *= this.sum();

    return number;
  }

  evaluate(): number {
    return this.product();
  }
}

console.log(inputs.reduce((acc, line) => acc + new Parser(line).evaluate(), 0));

import { path } from "../deps.ts";

const getProduct = (numbers: number[]) => {
  for (const number of numbers) {
    for (const number2 of numbers) {
      if (number + number2 === 2020) {
        return number * number2;
      }
    }
  }

  throw new Error("unable to find pair");
};

const numbers = (await Deno.readTextFile(
  path.join(path.fromFileUrl(path.dirname(import.meta.url)), "data.txt"),
))
  .split("\n").map(Number);

console.log(getProduct(numbers));

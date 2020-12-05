import { path } from "../deps.ts";

const getProduct = (numbers: number[]) => {
  for (const number of numbers) {
    for (const number2 of numbers) {
      for (const number3 of numbers) {
        if (number + number2 + number3 === 2020) {
          return number * number2 * number3;
        }
      }
    }
  }

  throw new Error("unable to find triple");
};

const numbers = (await Deno.readTextFile(
  path.join(path.fromFileUrl(path.dirname(import.meta.url)), "data.txt"),
))
  .split("\n").map(Number);

console.log(getProduct(numbers));

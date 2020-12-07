import { readFile } from "../utils.ts";

const rules = (await readFile(import.meta.url)).split("\n");

const getName = (rule: string) => /(^[\w\s]+)s contain/.exec(rule)![1];

const findBagsThatContain = (bag: string) =>
  rules.filter((rule) => new RegExp(String.raw`\d ${bag}`).test(rule)).map(
    getName,
  );

const bags = findBagsThatContain("shiny gold bag");

const goldBagHolders = new Set<string>();

while (bags.length) {
  const bag = bags.shift();
  if (!bag || goldBagHolders.has(bag)) continue;
  goldBagHolders.add(bag);
  bags.push(...findBagsThatContain(bag));
}

console.log(goldBagHolders.size);

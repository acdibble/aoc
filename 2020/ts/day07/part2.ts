import { readFile } from "../utils.ts";

const rules = (await readFile(import.meta.url)).split("\n");

const bagRegExp = /\d+[a-z ]+bags?/g;

const findBagsContainedBy = (bag: string) =>
  rules.filter((rule) => new RegExp(String.raw`^${bag}s contain`).test(rule))
    .flatMap((rule) => rule.match(bagRegExp) ?? []);

const getChildBagCount = (bag: string, initialValue = 0): number => {
  const children = findBagsContainedBy(bag);

  return children.reduce((acc, child) => {
    const [, amount, bag] = /(\d+) ([a-z ]+bag)s?/.exec(child)!;

    return acc + (Number(amount) * getChildBagCount(bag, 1));
  }, initialValue);
};

console.log(getChildBagCount("shiny gold bag"));

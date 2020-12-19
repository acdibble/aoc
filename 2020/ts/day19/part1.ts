import { readFile } from "../utils.ts";

const input = await readFile(import.meta.url);

const rules = [...input.matchAll(/(\d+): (.+)/g)]
  .reduce<Record<string, string>>((acc, [, name, rule]) => {
    acc[name] = rule;
    return acc;
  }, {});

const numRegExp = /(\d+)/;

const parseRule = (name: string): string => {
  let rule = rules[name];
  while (numRegExp.test(rule)) {
    rule = rule.replace(numRegExp, (_, m) => {
      const replacement = parseRule(m);
      return /\|/.test(replacement) ? `(${replacement})` : replacement;
    });
  }
  rule = rule.replace(/[" ]/g, "");
  return rule;
};

const messages = input.split("\n\n")[1].split("\n");

const regExp0 = new RegExp(`^${parseRule("0")}$`);

console.log(messages.filter((m) => regExp0.test(m)).length);

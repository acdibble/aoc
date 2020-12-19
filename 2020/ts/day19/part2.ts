import { readFile } from "../utils.ts";

const input = (await readFile(import.meta.url))
  .replace(/8: 42/, "8: 42 | 42 8")
  .replace(/11: 42 31/, "11: 42 31 | 42 11 31");

const rules = [...input.matchAll(/(\d+): (.+)/g)]
  .reduce<Record<string, string>>((acc, [, name, rule]) => {
    acc[name] = rule;
    return acc;
  }, {});

const numRegExp = /(\d+)/;

const parseRule = (name: string, depth = 0): string => {
  if (depth === 50) return "";
  let rule = rules[name];
  while (numRegExp.test(rule)) {
    rule = rule.replace(numRegExp, (_, m) => {
      const replacement = parseRule(m, depth + 1);
      return /\|/.test(replacement) ? `(${replacement})` : replacement;
    });
  }
  rule = rule.replace(/[" ]/g, "");
  return rule;
};

const messages = input.split("\n\n")[1].split("\n");

const regExp0 = new RegExp(`^${parseRule("0")}$`);

console.log(messages.filter((m) => regExp0.test(m)).length);

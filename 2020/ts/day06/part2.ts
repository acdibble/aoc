import { GoodSet, readFile } from "../utils.ts";

const sum = (await readFile(import.meta.url)).split("\n\n").reduce(
  (total, group) =>
    total +
    group.split("\n").reduce<GoodSet<string> | null>(
      (acc, member) =>
        acc === null
          ? new GoodSet(member)
          : new GoodSet(member).intersection(acc),
      null,
    )!.size,
  0,
);

console.log(sum);

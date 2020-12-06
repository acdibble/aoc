import { readFile } from "../utils.ts";

class GoodSet<T> extends Set<T> {
  intersection(other: GoodSet<T> | null) {
    if (other === null) return this;
    return new GoodSet<T>([...this].filter((val) => other.has(val)));
  }
}

const sum = (await readFile(import.meta.url)).split("\n\n").reduce(
  (total, group) =>
    total +
    group.split("\n").reduce<GoodSet<string> | null>((acc, member) =>
      new GoodSet(member).intersection(acc), null)!.size,
  0,
);

console.log(sum);

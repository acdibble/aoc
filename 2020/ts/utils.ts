import { path } from "./deps.ts";

export const readFile = (url: string): Promise<string> =>
  Deno.readTextFile(
    path.join(path.fromFileUrl(path.dirname(url)), "data.txt"),
  );

export class GoodSet<T> extends Set<T> {
  intersection(other: GoodSet<T>): GoodSet<T>;
  intersection(other: null): null;
  intersection(other: GoodSet<T> | null): GoodSet<T> | null {
    if (other === null) return null;
    return new GoodSet<T>([...this].filter((val) => other.has(val)));
  }

  difference(other: GoodSet<T> | null): GoodSet<T> {
    if (other === null) return this;
    return new GoodSet<T>([...this].filter((val) => !other.has(val)));
  }
}

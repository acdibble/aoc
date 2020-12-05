import { path } from "../deps.ts";

const validPasswordCount = (await Deno.readTextFile(
  path.join(path.fromFileUrl(path.dirname(import.meta.url)), "data.txt"),
)).split("\n").reduce((acc, line) => {
  const {
    min,
    max,
    letter,
    password,
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  } = /^(?<min>\d+)-(?<max>\d+) (?<letter>[a-z]): (?<password>[a-z]+)$/.exec(
    line,
  )!.groups!;

  const difference = password.match(new RegExp(`(${letter})`, "g"))?.length ??
    0;
  return acc + Number(difference >= Number(min) && difference <= Number(max));
}, 0);

console.log(validPasswordCount);

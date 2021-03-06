import { path } from "../deps.ts";

const validPasswordCount = (await Deno.readTextFile(
  path.join(path.fromFileUrl(path.dirname(import.meta.url)), "data.txt"),
)).split("\n").reduce((acc, line) => {
  const {
    min,
    max,
    letter,
    password,
  } = /^(?<min>\d+)-(?<max>\d+) (?<letter>[a-z]): (?<password>[a-z]+)$/.exec(
    line,
  )!.groups!;

  return acc +
    Number(
      (password[Number(min) - 1] === letter) !==
        (password[Number(max) - 1] === letter),
    );
}, 0);

console.log(validPasswordCount);

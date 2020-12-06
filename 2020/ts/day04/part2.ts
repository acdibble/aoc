import { path } from "../deps.ts";

const monster =
  /(?=(.\n?)*byr:(19[2-9][0-9]|200[012])\b)(?=(.\n?)*iyr:20(1[0-9]|20)\b)(?=(.\n?)*eyr:20(2[0-9]|30)\b)(?=(.\n?)*hgt:((59|6[0-9]|7[0-6])in|1(([5-8][0-9]|9[0-3])cm))\b)(?=(.\n?)*hcl:#[0-9a-f]{6}\b)(?=(.\n?)*ecl:(amb|blu|brn|gry|grn|hzl|oth)\b)(?=(.\n?)*pid:[0-9]{9}\b)([\w\W]+?)(\n\n|$)/g;

const count = (await Deno.readTextFile(
  path.join(path.fromFileUrl(path.dirname(import.meta.url)), "data.txt"),
)).match(monster)?.length;

console.log(count);

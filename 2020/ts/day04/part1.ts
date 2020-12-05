import { path } from "../deps.ts";

const passports = (await Deno.readTextFile(
  path.join(path.fromFileUrl(path.dirname(import.meta.url)), "data.txt"),
))
  .split("\n\n")
  .map((line) => line.replace(/\n/g, " "));

const birthYear = "byr";
const issueYear = "iyr";
const expirationYear = "eyr";
const height = "hgt";
const hairColor = "hcl";
const eyeColor = "ecl";
const passportID = "pid";
// const countryID = 'cid';

const requiredFields = [
  birthYear,
  issueYear,
  expirationYear,
  height,
  hairColor,
  eyeColor,
  passportID,
];

console.log(
  passports.filter((passport) =>
    requiredFields.every((field) => passport.includes(field))
  ).length,
);

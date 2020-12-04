import * as fs from 'fs';
import * as path from 'path';

const passports = fs.readFileSync(path.join(__dirname, 'data.txt'), 'utf8')
  .split('\n\n')
  .map((line) => line.replace(/\n/g, ' '));

const birthYear = 'byr';
const issueYear = 'iyr';
const expirationYear = 'eyr';
const height = 'hgt';
const hairColor = 'hcl';
const eyeColor = 'ecl';
const passportID = 'pid';
// const countryID = 'cid';

const regExps = [
  birthYear,
  issueYear,
  expirationYear,
  height,
  hairColor,
  eyeColor,
  passportID,
].map((string) => new RegExp(`(${string}):([^ ]+)`));

const checkHeight = (val: string): boolean => {
  const [, num, unit] = /^([0-9]+)(cm|in)$/.exec(val) ?? [];
  if (!num) return false;

  if (unit === 'cm') return num >= '150' && num <= '193';
  return num >= '59' && num <= '76';
};

const validators: Record<string, { test: (val: string) => boolean}> = {
  [birthYear]: { test: (val) => val >= '1920' && val <= '2002' },
  [issueYear]: { test: (val) => val >= '2010' && val <= '2020' },
  [expirationYear]: { test: (val) => val >= '2020' && val <= '2030' },
  [height]: { test: checkHeight },
  [hairColor]: /^#[0-9a-f]{6}$/,
  [eyeColor]: /^(amb|blu|brn|gry|grn|hzl|oth)$/,
  [passportID]: /^[0-9]{9}$/,
};

console.log(passports.filter((passport) => regExps.every((regExp) => {
  const [, name, value] = regExp.exec(passport) ?? [];
  if (!value) return false;
  const validator = validators[name];
  return validator.test(value);
})).length);

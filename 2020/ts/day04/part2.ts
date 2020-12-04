import * as fs from 'fs';
import * as path from 'path';

// eslint-disable-next-line max-len
const monster = /(?=(?:.|[^\n]\n)*byr:(?:19[2-9][0-9]|200[012])(?:\s|$))(?=(?:.|[^\n]\n)*iyr:20(?:1[0-9]|20)(?:\s|$))(?=(?:.|[^\n]\n)*eyr:20(?:2[0-9]|30)(?:\s|$))(?=(?:.|[^\n]\n)*hgt:(?:(?:59|6[0-9]|7[0-6])in|1(?:(?:[5-8][0-9]|9[0-3])cm))(?:\s|$))(?=(?:.|[^\n]\n)*hcl:#[0-9a-f]{6}(?:\s|$))(?=(?:.|[^\n]\n)*ecl:(?:amb|blu|brn|gry|grn|hzl|oth)(?:\s|$))(?=(?:.|[^\n]\n)*pid:[0-9]{9}(?:\s|$))([\w\W]+?)(?:\n\n|$)/g;

const count = fs.readFileSync(path.join(__dirname, 'data.txt'), 'utf8').match(monster)?.length;

console.log(count);

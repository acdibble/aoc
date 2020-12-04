import * as fs from 'fs';
import * as path from 'path';

const monster = new RegExp(
  '(?=.*byr:(19[2-9][0-9]|200[012])(\\s|$))'
  + '(?=.*iyr:20(1[0-9]|20)(\\s|$))'
  + '(?=.*eyr:20(2[0-9]|30)(\\s|$))'
  + '(?=.*hgt:((59|6[0-9]|7[0-6])in|1(([5-8][0-9]|9[0-3])cm))(\\s|$))'
  + '(?=.*hcl:#[0-9a-f]{6}(\\s|$))'
  + '(?=.*ecl:(amb|blu|brn|gry|grn|hzl|oth)(\\s|$))'
  + '(?=.*pid:[0-9]{9}(\\s|$))',
  's',
);

const count = fs.readFileSync(path.join(__dirname, 'data.txt'), 'utf8')
  .split('\n\n')
  .reduce((acc, line) => acc + Number(monster.test(line)), 0);

console.log(count);

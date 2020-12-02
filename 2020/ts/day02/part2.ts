import * as fs from 'fs';
import * as path from 'path';

const validPasswordCount = fs.readFileSync(path.join(__dirname, 'data.txt'), 'utf8').split('\n').reduce((acc, line) => {
  const {
    min,
    max,
    letter,
    password,
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  } = /^(?<min>\d+)-(?<max>\d+) (?<letter>[a-z]): (?<password>[a-z]+)$/.exec(line)!.groups!;

  const count = Number(password[Number(min) - 1] === letter) + Number(password[Number(max) - 1] === letter);
  return acc + Number(count === 1);
}, 0);

console.log(validPasswordCount);

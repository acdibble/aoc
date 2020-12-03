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

  return acc + Number((password[Number(min) - 1] === letter) !== (password[Number(max) - 1] === letter));
}, 0);

console.log(validPasswordCount);

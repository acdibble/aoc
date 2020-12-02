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

  const difference = password.length - password.replace(new RegExp(`(${letter})`, 'g'), '').length;
  return acc + Number(difference >= Number(min) && difference <= Number(max));
}, 0);

console.log(validPasswordCount);

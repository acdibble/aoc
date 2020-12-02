import * as fs from 'fs';
import * as path from 'path';
import { createInterface } from 'readline';

(async () => {
  const stream = fs.createReadStream(path.join(__dirname, 'data.txt'), 'utf8');
  const rl = createInterface(stream);
  stream.on('end', () => rl.close());

  let validCount = 0;
  for await (const line of rl) {
    const {
      min,
      max,
      letter,
      password,
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    } = /^(?<min>\d+)-(?<max>\d+) (?<letter>[a-z]): (?<password>[a-z]+)$/.exec(line)!.groups!;

    const difference = password.length - password.replace(new RegExp(`(${letter})`, 'g'), '').length;
    if (difference >= Number(min) && difference <= Number(max)) {
      validCount += 1;
    }
  }

  console.log(validCount);
})();

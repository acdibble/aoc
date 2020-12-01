import * as fs from 'fs';
import * as path from 'path';

const getProduct = (numbers: number[]) => {
  for (const number of numbers) {
    for (const number2 of numbers) {
      for (const number3 of numbers) {
        if (number + number2 + number3 === 2020) {
          return number * number2 * number3;
        }
      }
    }
  }

  throw new Error('unable to find triple');
};

(async () => {
  const numbers = (await fs.promises.readFile(path.join(__dirname, 'data.txt'), 'utf8')).split('\n').map(Number);

  console.log(getProduct(numbers));
})();

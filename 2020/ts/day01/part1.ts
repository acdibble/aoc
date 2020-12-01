import * as fs from 'fs';
import * as path from 'path';

const getProduct = (numbers: number[]) => {
  for (const number of numbers) {
    for (const number2 of numbers) {
      if (number + number2 === 2020) {
        return number * number2;
      }
    }
  }

  throw new Error('unable to find pair');
};

(async () => {
  const numbers = (await fs.promises.readFile(path.join(__dirname, 'data.txt'), 'utf8')).split('\n').map(Number);

  console.log(getProduct(numbers));
})();

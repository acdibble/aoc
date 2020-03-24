const fs = require('fs');

const output = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .trim()
  .split('\n')
  .reduce((acc, numberAsString) => acc + Math.floor(numberAsString / 3) - 2, 0);

console.log(output);

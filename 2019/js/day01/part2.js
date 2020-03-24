const fs = require('fs');

const output = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .trim()
  .split('\n')
  .reduce((acc, numberAsString) => {
    let requiredFuel = Math.floor(numberAsString / 3) - 2;
    let totalForModule = 0;
    do {
      totalForModule += requiredFuel;
      requiredFuel = Math.floor(requiredFuel / 3) - 2;
    } while (requiredFuel > 0);

    return acc + totalForModule;
  }, 0);

console.log(output);

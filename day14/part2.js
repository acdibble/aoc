const fs = require('fs');

const data = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .trim()
  .split('\n')
  .map((reaction) => reaction.split(' => ').reverse())
  .reduce((acc, [output, input]) => {
    acc[output] = input.split(', ').reduce((subAcc, countAndType) => {
      const [count, type] = countAndType.split(' ');
      subAcc[type] = Number(count);
      return subAcc;
    }, {});
    return acc;
  }, {});

const batchAmounts = Object.keys(data).reduce((acc, numAndType) => {
  const [min, type] = numAndType.split(' ');
  acc[type] = Number(min);
  return acc;
}, {});

const run = (num) => {
  const store = Object.keys(batchAmounts).reduce((acc, type) => {
    acc[type] = 0;
    return acc;
  }, {});

  const make = (typeToMake, requiredAmount) => {
    const batchAmount = batchAmounts[typeToMake];
    const batchesNeeded = Math.ceil(requiredAmount / batchAmount);
    let ore = 0;
    const requirements = data[`${batchAmount} ${typeToMake}`];
    for (const [type, amt] of Object.entries(requirements)) {
      const minAmountToMake = batchesNeeded * amt;
      if (type === 'ORE') {
        ore += minAmountToMake;
      } else {
        if (store[type] < minAmountToMake) {
          ore += make(type, minAmountToMake - store[type]);
        }

        store[type] -= minAmountToMake;
      }
    }

    store[typeToMake] += (batchAmount * batchesNeeded);

    return ore;
  };


  return make('FUEL', num);
};

const MAX_ORE = 1e12;

let low = MAX_ORE / run(1);
let high = 4 * low;

while (low + 1 < high) {
  const mid = Math.round((low + high) / 2);
  console.log(mid);
  const requiredFuel = run(mid);
  if (requiredFuel === MAX_ORE) {
    console.log(mid);
    break;
  } else if (requiredFuel > MAX_ORE) {
    high = mid;
  } else {
    low = mid;
  }
}

console.log(`${low}:`, run(low));
console.log(`${high}:`, run(high));

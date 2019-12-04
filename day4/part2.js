const fs = require('fs');

const input = fs.readFileSync(`${__dirname}/data.txt`, 'utf8').trim();

const numberPassesValidation = (number) => {
  const num = number.toString();

  for (let i = 1; i < num.length; i++) {
    if (num.charCodeAt(i) < num.charCodeAt(i - 1)) {
      return false;
    }
  }

  return [...num.matchAll(/([0-9])\1+/g)].some(([match]) => match.length === 2);
}

const range = input.split('-');

const start = Number(range[0]);
const end = Number(range[1]);

let possibilities = 0;

for (let i = start; i <= end; i++) {
  if (numberPassesValidation(i)) {
    possibilities += 1;
  }
}

console.log(possibilities);

const fs = require('fs');

const bugs = fs.readFileSync(`${__dirname}/data.txt`, 'utf8').split('\n').map((l) => l.split(''));

const checkNeighbors = (num, i) => {
  let total = 0;
  if (i - 5 >= 0) total += (num >> (i - 5)) & 1;
  if (i + 5 < 25) total += (num >> (i + 5)) & 1;
  if (i % 5 !== 0 && i - 1 >= 0) total += (num >> (i - 1)) & 1;
  if ((i + 1) % 5 !== 0 && i + 1 < 25) total += (num >> (i + 1)) & 1;
  return total;
};

const start = Number.parseInt(bugs.flat().reverse().map((b) => Number(b === '#')).join(''), 2);

const previousResults = new Set();
let repeat = 0;
let result = start;
while (!repeat) {
  if (previousResults.has(result)) {
    repeat = result;
  } else {
    previousResults.add(result);
    let out = 0;
    for (let i = 0; i < 25; i++) {
      const numBugs = checkNeighbors(result, i);
      if (numBugs === 1 || (numBugs === 2 && ((result >> i) & 1) === 0)) {
        out |= (1 << i);
      }
    }
    result = out;
  }
}

console.log(result);

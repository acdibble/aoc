const fs = require('fs');

const BASE_PATTERN = [0, 1, 0, -1];

function* getPattern(n) {
  let isFirstPass = true;
  let patternIndex = 0;
  while (true) {
    for (let j = 0 + isFirstPass; j < n + 1; j++) {
      yield BASE_PATTERN[patternIndex];
    }
    isFirstPass = false;
    patternIndex = (patternIndex + 1) % 4;
  }
}

const processSignal = (signal) => {
  let nums = signal.split('');
  const len = nums.length;
  const nextSignal = [];
  for (let step = 0; step < 100; step++) {
    for (let i = 0; i < len; i++) {
      const it = getPattern(i);
      nextSignal[i] = Math.abs(nums.reduce((acc, num) => acc + (num * it.next().value), 0)) % 10;
    }
    nums = nextSignal.slice();
  }
  console.log(nextSignal.join('').slice(0, 8));
};

processSignal(fs.readFileSync(`${__dirname}/data.txt`, 'utf8').trim());

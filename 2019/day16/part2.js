/**
 * A port of https://gist.github.com/bluepichu/bb1d4f9746bd4812caed8b6182eafd2d
 * I'm not a math guy :(
 */

const fs = require('fs');

const masterNums = fs.readFileSync(`${__dirname}/data.txt`, 'utf8').trim().split('').map(Number);

let answer = masterNums.join('').repeat(10000).split('').map(Number);

const offset = Number(answer.slice(0, 7).join(''));

answer = answer.slice(offset);

const doPhase2 = (nums) => {
  let sum = nums.reduce((a, b) => a + b);
  const out = [];
  for (let i = 0; i < nums.length; i++) {
    out.push(((sum % 10) + 10) % 10);
    sum -= nums[i];
  }

  return out;
};

for (let i = 0; i < 100; i++) {
  answer = doPhase2(answer);
}

console.log(answer.slice(0, 8).join(''));

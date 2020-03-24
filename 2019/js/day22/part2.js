const fs = require('fs');

const DECK_SIZE = 119315717514047n;
const ITERATIONS = 101741582076661n;

const CUT = 'cut ';
const DEAL_INTO_NEW_STACK = 'deal into new stack';
const DEAL_WITH_INCREMENT = 'deal with increment ';

const pow = (base, exponent, modulus) => {
  if (modulus === 1n) return 0n;
  let result = 1n;
  base %= modulus;
  while (exponent > 0n) {
    if (exponent % 2n === 1n) {
      result = (result * base) % modulus;
    }
    exponent >>= 1n;
    base = (base * base) % modulus;
  }
  return result;
};

const inv = (n) => pow(n, DECK_SIZE - 2n, DECK_SIZE);

let increment = 1n;
let offset = 0n;

fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .split('\n')
  .forEach((instruction) => {
    if (instruction === DEAL_INTO_NEW_STACK) {
      increment *= -1n;
      increment %= DECK_SIZE;
      offset += increment;
      offset %= DECK_SIZE;
    }

    if (instruction.startsWith(CUT)) {
      const num = BigInt(instruction.slice(CUT.length));
      offset += num * increment;
      offset %= DECK_SIZE;
    }

    const num = BigInt(instruction.slice(DEAL_WITH_INCREMENT.length));
    increment *= inv(num);
    increment %= DECK_SIZE;
  });

const inc = pow(increment, ITERATIONS, DECK_SIZE);
let off = offset * (1n - increment) * inv((1n - increment) % DECK_SIZE);
off %= DECK_SIZE;

console.log((off + 2020n * inc) % DECK_SIZE);

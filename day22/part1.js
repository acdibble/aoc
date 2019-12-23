const fs = require('fs');

const DECK_SIZE = 10007;

const CUT = 'cut ';
const DEAL_INTO_NEW_STACK = 'deal into new stack';
const DEAL_WITH_INCREMENT = 'deal with increment ';

const finalDeck = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .split('\n')
  .reduce((acc, instruction) => {
    if (instruction === DEAL_INTO_NEW_STACK) {
      return acc.reverse();
    }

    if (instruction.startsWith(CUT)) {
      const num = Number(instruction.slice(CUT.length));
      return acc.slice(num).concat(acc.slice(0, num));
    }

    const num = Number(instruction.slice(DEAL_WITH_INCREMENT.length));
    const blankDeck = Array.from({ length: DECK_SIZE });

    for (let i = 0; i < DECK_SIZE; i++) {
      blankDeck[(i * num) % DECK_SIZE] = acc[i];
    }

    return blankDeck;
  }, Array.from({ length: DECK_SIZE }, (v, i) => i));

console.log(finalDeck.indexOf(2019));

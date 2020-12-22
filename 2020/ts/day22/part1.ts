import { readFile } from "../utils.ts";

const [deck1, deck2] = (await readFile(import.meta.url)).split("\n\n")
  .map((deck) => deck.split("\n").slice(1).map(Number));

const calculateScore = (deck: number[]): number => {
  let result = 0;
  const { length } = deck;
  for (let i = 0; i < length; i++) {
    result += (deck[i] * (length - i));
  }
  return result;
};

while (deck1.length !== 0 && deck2.length !== 0) {
  const card1 = deck1.shift()!;
  const card2 = deck2.shift()!;
  if (card1 > card2) {
    deck1.push(card1, card2);
  } else {
    deck2.push(card2, card1);
  }
}

console.log(Math.max(calculateScore(deck1), calculateScore(deck2)));

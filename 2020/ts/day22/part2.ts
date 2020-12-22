import { readFile } from "../utils.ts";

type Deck = number[];

const calculateScore = (deck: Deck): number => {
  let result = 0;
  const { length } = deck;
  for (let i = 0; i < length; i++) {
    result += (deck[i] * (length - i));
  }
  return result;
};

// https://en.wikipedia.org/wiki/Pairing_function#Cantor_pairing_function
const calculateState = (deck1: Deck, deck2: Deck): number => {
  const state1 = calculateScore(deck1);
  const state2 = calculateScore(deck2);
  return ((state1 + state2) * (state1 + state2 + 1) / 2) + state2;
};

const playGame = (deck1: Deck, deck2: Deck): readonly [Deck, Deck] => {
  const deckStates = new Set<number>();

  while (deck1.length !== 0 && deck2.length !== 0) {
    const state = calculateState(deck1, deck2);
    if (deckStates.has(state)) {
      deck1 = deck1.concat(deck2);
      deck2 = [];
      break;
    }

    deckStates.add(state);

    const card1 = deck1.shift()!;
    const card2 = deck2.shift()!;
    let winner: 0 | 1;

    if (deck1.length >= card1 && deck2.length >= card2) {
      const results = playGame(deck1.slice(0, card1), deck2.slice(0, card2));
      winner = Number(results[0].length === 0) as 0 | 1;
    } else {
      winner = Number(card2 > card1) as 0 | 1;
    }

    if (winner === 0) {
      deck1.push(card1, card2);
    } else {
      deck2.push(card2, card1);
    }
  }

  return [deck1, deck2] as const;
};

const [deck1, deck2] = (await readFile(import.meta.url)).split("\n\n")
  .map((deck) => deck.split("\n").slice(1).map(Number));

console.log(Math.max(...playGame(deck1, deck2).map(calculateScore)));

import { GoodSet, readFile } from "../utils.ts";

const lines = (await readFile(import.meta.url)).split("\n");

const sets = lines
  .reduce<Record<string, GoodSet<string>>>((setMap, food) => {
    const [, ingredients, allergens] = /([\w\s]+?) \(contains ([\s\w,]+)\)/
      .exec(food)!;
    const ingredientList = ingredients.split(" ");
    const ingredientSet = new GoodSet(ingredientList);
    allergens.split(", ").forEach((allergen) => {
      setMap[allergen] = (setMap[allergen] ?? ingredientSet)
        .intersection(ingredientSet);
    });
    return setMap;
  }, {});

const tuples = Object.entries(sets);

let run = true;
while (run) {
  run = false;
  for (const [, set] of tuples) {
    if (set.size === 1) {
      for (let i = 0; i < tuples.length; i++) {
        if (tuples[i][1].size !== 1) {
          tuples[i][1] = tuples[i][1].difference(set);
          run = true;
        }
      }
    }
  }
}

console.log(
  tuples.sort(([a], [b]) => a < b ? -1 : 1).map(([, set]) => Array.from(set)[0])
    .join(","),
);

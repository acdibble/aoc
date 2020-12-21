import { GoodSet, readFile } from "../utils.ts";

const lines = (await readFile(import.meta.url)).split("\n");

const [ingredientCount, ingredients, sets] = lines
  .reduce<
  [Record<string, number>, GoodSet<string>, Record<string, GoodSet<string>>]
>(
  ([counts, allIngredients, setMap], food) => {
    const [, ingredients, allergens] = /([\w\s]+?) \(contains ([\s\w,]+)\)/
      .exec(food)!;
    const ingredientList = ingredients.split(" ");
    ingredientList.forEach((ingredient) => {
      counts[ingredient] ||= 0;
      counts[ingredient] += 1;
    });
    const ingredientSet = new GoodSet(ingredientList);
    allergens.split(", ").forEach((allergen) => {
      setMap[allergen] = (setMap[allergen] ?? ingredientSet)
        .intersection(ingredientSet);
    });
    return [counts, allIngredients.union(ingredientSet), setMap];
  },
  [{}, new GoodSet<string>(), {}],
);

const safeIngredientCount = Array.from(ingredients
  .difference(Object.values(sets).reduce((acc, set) => acc.union(set))))
  .reduce((acc, ingredient) => acc + ingredientCount[ingredient], 0);

console.log(safeIngredientCount);

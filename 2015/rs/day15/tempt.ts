const getRemainderPossibilities = (total: number, n = 0) => {
  const spaces = new Array(n * 4 + 1).join(" ");

  const possibilities: List[] = [];

  if (n === ingredientNames.length - 1) {
    return [[{ name: ingredientNames[n], amount: total }]];
  } else {
    for (let i = total; i >= 0; i--) {
      const item = { name: ingredientNames[n], amount: i };

      if (i !== total) {
        const remainder = getRemainderPossibilities(total - i, n + 1);
        if (!remainder.length) {
          console.log(spaces, "debg:", total - i, n + 1);
        }
        remainder.forEach(function (list) {
          if (i !== 0) {
            list.unshift(item);
          }
          possibilities.push(list);
        });
      } else {
        possibilities.push([item]);
      }
    }
  }

  return possibilities;
};

const str = `Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
Mint: capacity 2, durability 3, flavor -2, texture -1, calories 3`;

interface Ingredient {
  name: string;
  capacity: number;
  durability: number;
  flavor: number;
  texture: number;
  calories: number;
}

const ingredients: Record<string, Ingredient> = {};

str.split("\n").forEach(function (ingredient) {
  const match =
    /(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)/.exec(
      ingredient
    )!;
  ingredients[match[1]!] = {
    name: match[1]!,
    capacity: parseInt(match[2]!),
    durability: parseInt(match[3]!),
    flavor: parseInt(match[4]!),
    texture: parseInt(match[5]!),
    calories: parseInt(match[6]!),
  };
});

const ingredientNames = Object.keys(ingredients);

type List = { name: string; amount: number }[];

function score(list: List, requiredCalories?: number) {
  let capacity = 0;
  let durability = 0;
  let flavor = 0;
  let texture = 0;
  let calories = 0;

  for (const item of list) {
    capacity += ingredients[item.name].capacity * item.amount;
    durability += ingredients[item.name].durability * item.amount;
    flavor += ingredients[item.name].flavor * item.amount;
    texture += ingredients[item.name].texture * item.amount;
    calories += ingredients[item.name].calories * item.amount;
  }

  if (capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0) return 0;

  if (requiredCalories && calories !== requiredCalories) return 0;

  return capacity * durability * flavor * texture;
}

const possibilities = getRemainderPossibilities(100);
let partOne = 0;
let partTwo = 0;
possibilities.forEach(function (list) {
  partOne = Math.max(partOne, score(list));
  partTwo = Math.max(partTwo, score(list, 500));
});

console.log("Part One:", partOne);
console.log("Part Two:", partTwo);

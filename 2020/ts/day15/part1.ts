export const startingNumbers = [19, 20, 14, 0, 9, 1];

export const findNthNumber = (inputNumbers: number[], n: number): number => {
  const input = [...inputNumbers];
  let previousNumber = input.pop()!;

  const numberMap = new Map(input.map((n, i) => [n, i]));

  for (
    let currentIndex = input.length;
    currentIndex < n - 1;
    currentIndex++
  ) {
    const temp = numberMap.get(previousNumber);
    numberMap.set(previousNumber, currentIndex);
    previousNumber = temp === undefined ? 0 : currentIndex - temp;
  }

  return previousNumber;
};

if (import.meta.main) {
  console.log(findNthNumber(startingNumbers, 2020));
}

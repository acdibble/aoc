export const startingNumbers = [19, 20, 14, 0, 9, 1];

export const findNthNumber = (inputNumbers: number[], n: number): number => {
  const input = [...inputNumbers];
  let previousNumber = input.pop()!;

  const numberMap = new Int32Array(3e7).fill(-1);
  input.forEach((n, i) => {
    numberMap[Number(n)] = i;
  });

  for (
    let currentIndex = input.length;
    currentIndex < n - 1;
    currentIndex++
  ) {
    const temp = numberMap[previousNumber];
    numberMap[previousNumber] = currentIndex;
    previousNumber = temp === -1 ? 0 : currentIndex - temp;
  }

  return previousNumber;
};

if (import.meta.main) {
  console.log(findNthNumber(startingNumbers, 2020));
}

/* eslint-disable no-param-reassign */
const generate = (size, array, cb) => {
  if (size === 1) {
    cb(array.slice());
  } else {
    generate(size - 1, array, cb);

    for (let i = 0; i < size - 1; i++) {
      const index = size % 2 === 0 ? i : 0;
      const temp = array[index];
      array[index] = array[size - 1];
      array[size - 1] = temp;
      generate(size - 1, array, cb);
    }
  }
};

const permutate = (array) => {
  const copy = array.slice();
  const permutations = [];
  generate(array.length, copy, permutations.push.bind(permutations));
  return permutations;
};

module.exports = permutate;

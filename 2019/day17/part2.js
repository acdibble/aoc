const fs = require('fs');
const intcodeComputer = require('../lib/intcodeComputer');

const intcodes = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .replace(/\s/g, '')
  .split(',')
  .map(Number);

const mapTiles = [];

intcodeComputer(null, (val) => mapTiles.push(val))(intcodes);

const lineLength = mapTiles.indexOf('\n'.charCodeAt(0)) + 1;

const SCAFFOLD = '#'.charCodeAt(0);

const instructions = [];

const UP = -lineLength;
const RIGHT = 1;
const DOWN = lineLength;
const LEFT = -1;

const getDirection = (location, previousDirection) => {
  if (mapTiles[location + RIGHT] === SCAFFOLD && previousDirection !== LEFT) {
    return [RIGHT, previousDirection === UP ? 'R' : 'L'];
  }
  if (mapTiles[location + LEFT] === SCAFFOLD && previousDirection !== RIGHT) {
    return [LEFT, previousDirection === DOWN ? 'R' : 'L'];
  }
  if (mapTiles[location + DOWN] === SCAFFOLD && previousDirection !== UP) {
    return [DOWN, previousDirection === RIGHT ? 'R' : 'L'];
  }
  if (mapTiles[location + UP] === SCAFFOLD && previousDirection !== DOWN) {
    return [UP, previousDirection === LEFT ? 'R' : 'L'];
  }
  return [0, ''];
};

let location = mapTiles.indexOf('^'.charCodeAt(0));
let [direction, letter] = getDirection(location, UP);
while (direction && letter) {
  let count = 0;
  while (mapTiles[location + direction] === SCAFFOLD) {
    count += 1;
    location += direction;
  }

  if (!count) break;

  instructions.push(`${letter}${count}`);

  [direction, letter] = getDirection(location, direction);
}

// console.log(instructions.join(','));

const A = 'L,6,R,12,L,6';
const B = 'R,12,L,10,L,4,L,6';
const C = 'L,10,L,10,L,4,L,6';
const replaceA = 'A'.padStart(9);
const replaceB = 'B'.padStart(13);
const replaceC = 'C'.padStart(13);
const mainRoutine = instructions.join(',')
  .replace(new RegExp(A.replace(/(?<=[LR]),/g, ''), 'g'), replaceA)
  .replace(new RegExp(B.replace(/(?<=[LR]),/g, ''), 'g'), replaceB)
  .replace(new RegExp(C.replace(/(?<=[LR]),/g, ''), 'g'), replaceC);
// console.log(mainRoutine);
// console.log('A =', A);
// console.log('B =', B);
// console.log('C =', C);

const inputs = [mainRoutine.replace(/\s/g, ''), A, B, C, 'n']
  .flatMap((instr) => {
    const output = Array.from({ length: instr.length + 1 });
    output[output.length - 1] = 10;
    for (let i = 0; i < instr.length; i++) {
      output[i] = instr.charCodeAt(i);
    }
    return output;
  });

const vacuumOutput = [];
const inputsAlreadyFedIn = [];

const vacuumRobot = intcodeComputer(() => {
  const input = inputs.shift();
  if (vacuumOutput.length) console.log(vacuumOutput.join('').trim());
  vacuumOutput.splice(0, Infinity);
  inputsAlreadyFedIn.push(String.fromCharCode(input));
  return input;
}, (out) => {
  vacuumOutput.push(String.fromCharCode(out));
  if (Number(out) > 165) console.log('output', out);
  if (vacuumOutput.length === mapTiles.length) {
    console.log(vacuumOutput.join(''));
    vacuumOutput.splice(0, Infinity);
  }
  if (inputsAlreadyFedIn.length) console.log(inputsAlreadyFedIn.join(''));
  inputsAlreadyFedIn.splice(0, Infinity);
});
intcodes[0] = 2;

vacuumRobot(intcodes);

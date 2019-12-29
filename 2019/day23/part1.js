const fs = require('fs');
const asyncIntcodeComputer = require('../lib/asyncIntcodeComputer');

const intcodes = fs.readFileSync(`${__dirname}/data.txt`, 'utf8').split(',').map(Number);

const NUM_COMPUTERS = 50;

const inputs = Array.from({ length: NUM_COMPUTERS }, (_, i) => [i]);
const outputs = Array.from({ length: NUM_COMPUTERS }, () => []);

const computers = Array.from({ length: NUM_COMPUTERS }, (_, address) => asyncIntcodeComputer(
  () => {
    const packetValue = inputs[address].shift();
    return packetValue == null ? -1 : packetValue;
  }, (a) => {
    outputs[address].push(a);
    if (outputs[address].length === 3) {
      const [computer, x, y] = outputs[address].splice(0, 3);
      if (computer === 255) {
        console.log('THE Y VALUE', y);
      } else {
        inputs[computer].push(x);
        inputs[computer].push(y);
      }
    }
  },
));

(async () => {
  for (const computer of computers) {
    computer(intcodes);
  }
})();

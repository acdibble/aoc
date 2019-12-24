const fs = require('fs');
const asyncIntcodeComputer = require('../asyncIntcodeComputer');

const intcodes = fs.readFileSync(`${__dirname}/data.txt`, 'utf8').split(',').map(Number);

const NUM_COMPUTERS = 50;

const inputs = Array.from({ length: NUM_COMPUTERS }, (_, i) => [i]);
const outputs = Array.from({ length: NUM_COMPUTERS }, () => []);
const idleMonitor = Array.from({ length: NUM_COMPUTERS }, () => [0]);

const nat = { x: -1, y: -1 };

const computers = Array.from({ length: NUM_COMPUTERS }, (_, address) => asyncIntcodeComputer(
  () => {
    const packetValue = inputs[address].shift();
    const input = packetValue == null ? -1 : packetValue;
    if (input === -1) {
      idleMonitor[address] += 1;
      if (idleMonitor.every((m) => m > 5) && nat.x > -1 && nat.y > -1) {
        console.log(nat);
        inputs[0].push(nat.x);
        inputs[0].push(nat.y);
        nat.x = -1;
        nat.y = -1;
      }
    }
    return input;
  }, (a) => {
    idleMonitor[address] = 0;
    outputs[address].push(a);
    if (outputs[address].length === 3) {
      const [computer, x, y] = outputs[address].splice(0, 3);
      if (computer === 255) {
        nat.x = x;
        nat.y = y;
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

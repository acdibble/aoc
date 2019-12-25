const fs = require('fs');
const asyncIntcodeComputer = require('../lib/asyncIntcodeComputer');

const intcodes = fs.readFileSync(`${__dirname}/data.txt`, 'utf8').split(',').map(Number);

const getLine = () => new Promise((resolve) => {
  process.stdin.once('data', (data) => {
    resolve([...data.toString('utf8')].map((c) => c.charCodeAt(0)));
  });
});

const inputs = [];

const run = asyncIntcodeComputer(async () => {
  if (inputs.length) {
    return inputs.shift();
  }
  process.stdout.write('> ');
  inputs.push(...await getLine());
  return inputs.shift();
}, (c) => {
  process.stdout.write(String.fromCharCode(c));
});

(async () => {
  await run(intcodes);
})();

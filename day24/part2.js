const fs = require('fs');

const bugs = fs.readFileSync(`${__dirname}/data.txt`, 'utf8').split('\n').map((l) => l.split(''));

const start = Number.parseInt(bugs.flat().reverse().map((b) => Number(b === '#')).join(''), 2);

let layers = {};
for (let i = -100; i <= 100; i++) {
  layers[i] = i === 0 ? start : 0;
}

const checkNeighbors = (num, i, depth) => {
  let total = 0;
  if (i === 12) return total;
  const nextLayerDown = layers[depth + 1];
  const nextLayerUp = layers[depth - 1];

  let neighbor = i - 5;
  if (neighbor >= 0 && neighbor !== 12) {
    total += (num >> neighbor) & 1;
  } else if (neighbor === 12) {
    total += (nextLayerDown >> 24) & 1;
    total += (nextLayerDown >> 23) & 1;
    total += (nextLayerDown >> 22) & 1;
    total += (nextLayerDown >> 21) & 1;
    total += (nextLayerDown >> 20) & 1;
  } else if (neighbor >= -5) {
    total += (nextLayerUp >> 7) & 1;
  }

  neighbor = i + 5;
  if (neighbor < 25 && neighbor !== 12) {
    total += (num >> neighbor) & 1;
  } else if (neighbor === 12) {
    total += (nextLayerDown >> 4) & 1;
    total += (nextLayerDown >> 3) & 1;
    total += (nextLayerDown >> 2) & 1;
    total += (nextLayerDown >> 1) & 1;
    total += (nextLayerDown >> 0) & 1;
  } else if (neighbor < 30) {
    total += (nextLayerUp >> 17) & 1;
  }

  neighbor = i - 1;
  if (i % 5 !== 0 && neighbor >= 0 && neighbor !== 12) {
    total += (num >> neighbor) & 1;
  } else if (neighbor === 12) {
    total += (nextLayerDown >> 24) & 1;
    total += (nextLayerDown >> 19) & 1;
    total += (nextLayerDown >> 14) & 1;
    total += (nextLayerDown >> 9) & 1;
    total += (nextLayerDown >> 4) & 1;
  } else if (i % 5 === 0 && neighbor >= -1) {
    total += (nextLayerUp >> 11) & 1;
  }

  neighbor = i + 1;
  if (neighbor % 5 !== 0 && neighbor < 25 && neighbor !== 12) {
    total += (num >> neighbor) & 1;
  } else if (neighbor === 12) {
    total += (nextLayerDown >> 20) & 1;
    total += (nextLayerDown >> 15) & 1;
    total += (nextLayerDown >> 10) & 1;
    total += (nextLayerDown >> 5) & 1;
    total += (nextLayerDown >> 0) & 1;
  } else if (neighbor % 5 === 0 && neighbor <= 25) {
    total += (nextLayerUp >> 13) & 1;
  }
  return total;
};

const tempLayers = { ...layers };

for (let minutes = 0; minutes < 200; minutes++) {
  for (let layerNum = -100; layerNum <= 100; layerNum++) {
    const layer = layers[layerNum];
    let out = 0;
    if (layer > 0 || layers[layerNum + 1] > 0 || layers[layerNum - 1] > 0) {
      for (let i = 0; i < 25; i++) {
        const numBugs = checkNeighbors(layer, i, layerNum);
        if (numBugs === 1 || (numBugs === 2 && ((layer >> i) & 1) === 0)) {
          out |= (1 << i);
        }
      }
    }
    tempLayers[layerNum] = out;
  }

  layers = { ...tempLayers };
}

console.log(Object.entries(layers)
  .filter(([, n]) => n !== 0)
  .reduce((acc, [, n]) => acc + n.toString(2).replace(/0/g, '').length, 0));

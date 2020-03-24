const fs = require('fs');

const rawImageData = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .replace(/\s/g, '');

const splitImageIntoLayers = (imageData, width, height) => {
  const imageLength = imageData.length;
  const layerLength = width * height;
  const layers = [];
  for (let i = 0; i < imageLength; i += layerLength) {
    const layer = [];
    for (let j = 0; j < layerLength; j += width) {
      layer.push(...imageData.slice(i + j, i + j + width).split('').map(Number));
    }
    layers.push(layer);
  }

  return layers;
};


const layers = splitImageIntoLayers(rawImageData, 25, 6);

let layerData = { 0: Infinity, 1: 0, 2: 0 };

for (const layer of layers) {
  const currentLayerData = layer.reduce((acc, num) => {
    acc[num] += 1;
    return acc;
  }, { 0: 0, 1: 0, 2: 0 });
  console.log(currentLayerData);
  if (currentLayerData[0] < layerData[0]) {
    layerData = currentLayerData;
  }
}

console.log(layerData);
console.log(layerData[1] * layerData[2]);

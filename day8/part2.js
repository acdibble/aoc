const fs = require('fs');

const rawImageData = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .replace(/\s/g, '');

const IMAGE_WIDTH = 25;
const IMAGE_LENGTH = 6;

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


const layers = splitImageIntoLayers(rawImageData, IMAGE_WIDTH, IMAGE_LENGTH);

const finalImage = layers.reduce((acc, layer) => {
  for (let i = 0; i < acc.length; i++) {
    if (acc[i] === 2) {
      acc[i] = layer[i];
    }
  }

  return acc;
}, Array.from({ length: IMAGE_WIDTH * IMAGE_LENGTH }, () => 2));

console.log(Array.from({ length: finalImage.length / IMAGE_WIDTH }, (_, i) => (
  finalImage.slice(i * IMAGE_WIDTH, i * IMAGE_WIDTH + IMAGE_WIDTH).join('')
)).join('\n').replace(/1/g, '#').replace(/0/g, ' '));

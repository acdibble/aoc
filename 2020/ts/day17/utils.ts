export type Cube = (0 | 1)[][][];
export type Hypercube = Cube[];

export const countCube = (cube: Cube): number =>
  cube.reduce<number>(
    (layerAcc, layer) =>
      layerAcc + layer.reduce<number>((lineAcc, line) =>
        lineAcc + line.reduce<number>((a, b) =>
          (a + b) as number, 0), 0),
    0,
  );

export const cloneCube = (inputDimension: Cube): Cube =>
  inputDimension.map((layer) => {
    const lineLength = layer[0].length;
    const blankLine = Array.from({ length: lineLength }, () => 0) as (0 | 1)[];
    layer.unshift(blankLine);
    layer.push(blankLine);
    return layer.map((line) => [...line]);
  });

export const padCube = (inputDimension: Cube): Cube => {
  const newDimension = cloneCube(inputDimension).map(
    (layer) => layer.map((line) => [0, ...line, 0]),
  );
  const blankNewLayer: (0 | 1)[][] = newDimension[0].map((line) =>
    line.map(() => 0)
  );
  newDimension.push(blankNewLayer);
  newDimension.unshift(blankNewLayer);
  return newDimension as Cube;
};

export const neighbors3D = [
  [-1, -1, -1],
  [-1, -1, 0],
  [-1, -1, 1],
  [-1, 0, -1],
  [-1, 0, 0],
  [-1, 0, 1],
  [-1, 1, -1],
  [-1, 1, 0],
  [-1, 1, 1],
  [0, -1, -1],
  [0, -1, 0],
  [0, -1, 1],
  [0, 0, -1],
  [0, 0, 1],
  [0, 1, -1],
  [0, 1, 0],
  [0, 1, 1],
  [1, -1, -1],
  [1, -1, 0],
  [1, -1, 1],
  [1, 0, -1],
  [1, 0, 0],
  [1, 0, 1],
  [1, 1, -1],
  [1, 1, 0],
  [1, 1, 1],
];

export const neighbors4D = [
  [-1, -1, -1, -1],
  [-1, -1, -1, 0],
  [-1, -1, -1, 1],
  [-1, -1, 0, -1],
  [-1, -1, 0, 0],
  [-1, -1, 0, 1],
  [-1, -1, 1, -1],
  [-1, -1, 1, 0],
  [-1, -1, 1, 1],
  [-1, 0, -1, -1],
  [-1, 0, -1, 0],
  [-1, 0, -1, 1],
  [-1, 0, 0, -1],
  [-1, 0, 0, 0],
  [-1, 0, 0, 1],
  [-1, 0, 1, -1],
  [-1, 0, 1, 0],
  [-1, 0, 1, 1],
  [-1, 1, -1, -1],
  [-1, 1, -1, 0],
  [-1, 1, -1, 1],
  [-1, 1, 0, -1],
  [-1, 1, 0, 0],
  [-1, 1, 0, 1],
  [-1, 1, 1, -1],
  [-1, 1, 1, 0],
  [-1, 1, 1, 1],
  [0, -1, -1, -1],
  [0, -1, -1, 0],
  [0, -1, -1, 1],
  [0, -1, 0, -1],
  [0, -1, 0, 0],
  [0, -1, 0, 1],
  [0, -1, 1, -1],
  [0, -1, 1, 0],
  [0, -1, 1, 1],
  [0, 0, -1, -1],
  [0, 0, -1, 0],
  [0, 0, -1, 1],
  [0, 0, 0, -1],
  [0, 0, 0, 1],
  [0, 0, 1, -1],
  [0, 0, 1, 0],
  [0, 0, 1, 1],
  [0, 1, -1, -1],
  [0, 1, -1, 0],
  [0, 1, -1, 1],
  [0, 1, 0, -1],
  [0, 1, 0, 0],
  [0, 1, 0, 1],
  [0, 1, 1, -1],
  [0, 1, 1, 0],
  [0, 1, 1, 1],
  [1, -1, -1, -1],
  [1, -1, -1, 0],
  [1, -1, -1, 1],
  [1, -1, 0, -1],
  [1, -1, 0, 0],
  [1, -1, 0, 1],
  [1, -1, 1, -1],
  [1, -1, 1, 0],
  [1, -1, 1, 1],
  [1, 0, -1, -1],
  [1, 0, -1, 0],
  [1, 0, -1, 1],
  [1, 0, 0, -1],
  [1, 0, 0, 0],
  [1, 0, 0, 1],
  [1, 0, 1, -1],
  [1, 0, 1, 0],
  [1, 0, 1, 1],
  [1, 1, -1, -1],
  [1, 1, -1, 0],
  [1, 1, -1, 1],
  [1, 1, 0, -1],
  [1, 1, 0, 0],
  [1, 1, 0, 1],
  [1, 1, 1, -1],
  [1, 1, 1, 0],
  [1, 1, 1, 1],
];

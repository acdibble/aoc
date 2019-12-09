const fs = require('fs');

const data = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .trim()
  .split('\n')
  .map((orbit) => orbit.split(')'))
  .reduce((acc, [body, satellite]) => {
    acc[body] = (acc[body] || []).concat([satellite]);
    return acc;
  }, {});

const buildTree = (name) => {
  const node = { name, children: null };
  const children = data[name];
  node.children = children ? children.map(buildTree) : [];
  return node;
};

const tree = buildTree('COM');

const traverseTree = (node, depth) => node.children
  .reduce((acc, child) => acc + traverseTree(child, depth + 1), depth);

console.log(traverseTree(tree, 0));

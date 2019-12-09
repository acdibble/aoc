const fs = require('fs');

const data = fs.readFileSync(`${__dirname}/data.txt`, 'utf8')
  .trim()
  .split('\n')
  .map((orbit) => orbit.split(')'))
  .reduce((acc, [body, satellite]) => {
    acc[body] = (acc[body] || []).concat([satellite]);
    return acc;
  }, {});

const buildTree = (name, parent = null) => {
  const node = { name, parent, children: null };
  const children = data[name];
  node.children = children
    ? children.map(((childName) => buildTree(childName, node)))
    : [];
  return node;
};

const tree = buildTree('COM');

const findDistanceToNode = (node, name, distance) => {
  if (node.name === name) {
    return distance;
  }

  if (node.children.length) {
    return node.children
      .reduce((acc, child) => (
        acc == null ? findDistanceToNode(child, name, distance + 1) : acc
      ), null);
  }

  return null;
};

const getPathBetween = (node, a, b) => {
  const flatTree = [node];

  for (let i = 0; i < flatTree.length; i++) {
    flatTree.push(...flatTree[i].children);
  }

  return flatTree.reduce((acc, body) => {
    const distToA = findDistanceToNode(body, a, -1);
    const distToB = findDistanceToNode(body, b, -1);

    return ((distToA && distToB) && (distToA + distToB) < acc)
      ? distToA + distToB
      : acc;
  }, Infinity);
};
console.log(getPathBetween(tree, 'YOU', 'SAN'));

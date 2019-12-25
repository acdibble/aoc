const fs = require('fs');
const asyncIntcodeComputer = require('../lib/asyncIntcodeComputer');

const intcodes = fs.readFileSync(`${__dirname}/data.txt`, 'utf8').split(',').map(Number);

const GOOD_ITEMS = new Set(['coin', 'mouse', 'hypercube', 'cake', 'pointer', 'tambourine', 'mug', 'monolith']);
const DIRECTIONS = ['north', 'south', 'east', 'west'];

/**
 * @typedef Room
 * @property {string} name
 * @property {string} description
 * @property {object} doors
 * @property {string} [doors.north]
 * @property {string} [doors.south]
 * @property {string} [doors.east]
 * @property {string} [doors.west]
 * @property {string[]} items
 */

/** @type {Room[]} */
const rooms = JSON.parse(fs.readFileSync(`${__dirname}/map.txt`, 'utf8'))
  .map((room, i, arr) => {
    for (const dir of DIRECTIONS) {
      if (room.doors[dir] != null) {
        room.doors[dir] = arr.find((r) => r.name === room.doors[dir]);
      }
    }
    return room;
  });

const visitedRooms = new Set(rooms.map(({ name }) => name));
let previousRoom = null;

/* eslint-disable comma-style */
const parseRoom = (chars) => {
  const [
    roomName,
    description,
    , // doors here lead
    ...rest
  ] = chars.join('').replace(/\n+/g, '\n').trim().split('\n');

  const matches = roomName.match(/== ([a-z\s]+) ==/i);

  if (matches === null) {
    return previousRoom;
  }

  if (visitedRooms.has(matches[1])) {
    return rooms.find(({ name }) => name === matches[1]);
  }

  const doors = {};

  while (rest[0] !== 'Command?' && rest[0] !== 'Items here:') {
    const [, direction] = rest.shift().split(' ');
    doors[direction] = null;
  }

  const items = [];
  if (rest[0] === 'Items here:') {
    rest.shift();
    while (rest[0] !== 'Command?') {
      items.push(rest.shift().slice(2));
    }
  }

  return {
    name: matches[1],
    description,
    doors,
    items,
  };
};
/* eslint-enable comma-style */

const OPPOSITE_DIRECTION = {
  south: 'north',
  north: 'south',
  west: 'east',
  east: 'west',
};

const inputs = [];
const outputs = [];
let previousCommand = 'start';

process.on('SIGINT', () => {
  fs.writeFileSync(`${__dirname}/map.txt`, JSON.stringify(rooms, (key, value) => {
    if (OPPOSITE_DIRECTION[key] !== undefined && value !== null) {
      return value.name;
    }
    return value;
  }, 2), 'utf8');
  process.exit(0);
});

const findPathToAllItems = (start) => {
  const queue = [{
    room: start,
    collectedItems: new Set(),
    moves: [],
  }];

  const cache = new Set();

  while (true) {
    const { room, collectedItems, moves } = queue.shift();
    const cacheKey = moves.join('');
    if (cache.has(cacheKey)) continue;
    const newCollectedItems = new Set(collectedItems);
    cache.add(cacheKey);
    for (const item of room.items) {
      if (GOOD_ITEMS.has(item) && !newCollectedItems.has(item)) {
        newCollectedItems.add(item);
        moves.push(`take ${item}`);
      }
    }
    if (newCollectedItems.size === GOOD_ITEMS.size) {
      console.log(queue.length);
      return { room, collectedItems: newCollectedItems, moves };
    }
    for (const dir of DIRECTIONS) {
      if (room.doors[dir]) {
        queue.push({
          room: room.doors[dir],
          collectedItems: newCollectedItems,
          moves: OPPOSITE_DIRECTION[moves[moves.length - 1]] === dir
            ? moves.slice(0, -1)
            : [...moves, dir],
        });
      }
    }
  }
};

const {
  room: currentRoom,
  moves: movesToCollectItems,
} = findPathToAllItems(rooms.find(({ name }) => name === 'Hull Breach'));

movesToCollectItems.forEach((move) => {
  inputs.push(...[...move].map((c) => c.charCodeAt(0)), 10);
});

const navigateTo = (source, dest) => {
  const queue = [{
    room: source,
    moves: [],
  }];

  const cache = new Set();

  while (true) {
    const { room, moves } = queue.shift();
    if (room === dest) return { room, moves };
    const cacheKey = moves.join('');
    if (cache.has(cacheKey)) continue;
    cache.add(cacheKey);
    for (const dir of DIRECTIONS) {
      if (room.doors[dir]) {
        queue.push({
          room: room.doors[dir],
          moves: OPPOSITE_DIRECTION[moves[moves.length - 1]] === dir
            ? moves.slice(0, -1)
            : [...moves, dir],
        });
      }
    }
  }
};

const securityCheckpoint = rooms.find((room) => room.name === 'Security Checkpoint');

const { moves: movesToDestination } = navigateTo(currentRoom, securityCheckpoint);

movesToDestination.forEach((move) => {
  inputs.push(...[...move].map((c) => c.charCodeAt(0)), 10);
});

GOOD_ITEMS.forEach((item) => {
  inputs.push(...[...`drop ${item}`].map((c) => c.charCodeAt(0)), 10);
});

let tooLight = true;

function* iterateItems(current, rest) {
  for (const item of rest) {
    current.push(item);
    yield `take ${item}\n`;
    yield 'north\n';
    if (tooLight) {
      yield* iterateItems(current, rest.filter((i) => !current.includes(i)));
    }
    yield `drop ${current.pop()}\n`;
  }
}

const iterator = iterateItems([], [...GOOD_ITEMS]);

const getLine = () => new Promise((resolve) => {
  if (securityCheckpoint.doors.north == null) {
    const nextMove = iterator.next().value;
    resolve(nextMove);
  } else {
    process.stdout.write('> ');
    process.stdin.on('data', (data) => {
      resolve(data.toString('utf8'));
    });
  }
});

(async () => {
  await asyncIntcodeComputer(async () => {
    if (outputs.length && /^(start|north|south|east|west)$/.test(previousCommand)) {
      // console.log(previousCommand, outputs);
      const currentOutputs = outputs.splice(0, Infinity);
      const room = parseRoom(currentOutputs);
      if (room !== previousRoom) {
        if (!visitedRooms.has(room.name)) {
          visitedRooms.add(room.name);
          rooms.push(room);
        }
        if (previousRoom && previousCommand !== 'start') {
          previousRoom.doors[previousCommand] = room;
          room.doors[OPPOSITE_DIRECTION[previousCommand]] = previousRoom;
        }
        previousRoom = room;
      } else {
        const outputAsString = currentOutputs.join('');
        if (outputAsString.includes('lighter')) {
          tooLight = false;
        } else if (outputAsString.includes('heavier')) {
          tooLight = true;
        }
      }
    }
    if (inputs.length) {
      return inputs.shift();
    }
    // eslint-disable-next-line guard-for-in
    // for (const dir in previousRoom.doors) {
    //   console.log(dir, '=>', previousRoom.doors[dir] && previousRoom.doors[dir].name);
    // }
    const command = await getLine();
    previousCommand = command.trim();
    inputs.push(...[...command].map((c) => c.charCodeAt(0)));
    return inputs.shift();
  }, (c) => {
    const char = String.fromCharCode(c);
    outputs.push(char);
    process.stdout.write(char);
  })(intcodes);
})();

import { cups, doNMoves, LinkedList } from "./part1.ts";

const highestValue = 1e6;

const linkedList = new LinkedList<number>(
  Array.from({ length: highestValue }, (_, i) => cups[i] ?? i + 1),
);

doNMoves({ linkedList, n: 10e6, highestValue });

const node = linkedList.map.get(1)!;

console.log(node.next.value * node.next.next.value);

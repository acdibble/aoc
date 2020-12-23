import { cups, LinkedList } from "./part1.ts";

const highestValue = 1e6;

const linkedList = new LinkedList(
  Array.from({ length: highestValue }, (_, i) => cups[i] ?? i + 1),
);

for (let i = 0; i < 10e6; i++) {
  linkedList.doMove();
}

const node = linkedList.map.get(1)!;

console.log(node.next.value * node.next.next.value);

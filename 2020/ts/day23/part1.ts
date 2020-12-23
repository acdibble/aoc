const cupLabelString = "167248359";

class Node {
  constructor(
    readonly value: number,
    public next: Node,
  ) {}
}

export class LinkedList {
  pointer: Node;
  map: Map<number, Node> = new Map();
  highestValue = 0;

  constructor(values: number[]) {
    // deno-lint-ignore no-explicit-any
    const initial = new Node(values[values.length - 1] as number, null as any);
    this.map.set(initial.value, initial);
    this.pointer = initial;
    let highestValue = initial.value;
    for (let i = values.length - 2; i >= 0; i--) {
      this.pointer = new Node(values[i], this.pointer);
      this.map.set(values[i], this.pointer);
      if (highestValue < values[i]) highestValue = values[i];
    }
    initial.next = this.pointer;
    this.highestValue = highestValue;
  }

  insertAfter(value: number, destination: number) {
    const node = this.map.get(destination)!;
    const newNode = new Node(value, node.next);
    this.map.set(value, newNode);
    node.next = newNode;
  }

  removeAfterPointer(): number {
    const afterPointer = this.pointer.next;
    this.pointer.next = afterPointer.next;
    this.map.delete(afterPointer.value);
    return afterPointer.value;
  }

  advancePointer() {
    this.pointer = this.pointer!.next;
  }

  doMove(): void {
    const removedValues = [
      this.removeAfterPointer(),
      this.removeAfterPointer(),
      this.removeAfterPointer(),
    ];

    let destination = this.pointer!.value - 1;
    while (!this.map.has(destination)) {
      destination -= 1;
      if (destination <= 0) destination = this.highestValue;
    }

    let previousValue = destination;

    for (let i = 0; i < 3; i++) {
      const value = removedValues[i];
      this.insertAfter(value, previousValue);
      previousValue = value;
    }
    this.advancePointer();
  }
}

export const cups = cupLabelString.split("").map(Number);

if (import.meta.main) {
  const linkedList = new LinkedList(cups);

  for (let i = 0; i < 100; i++) {
    linkedList.doMove();
  }

  let result = "";
  for (
    let node = linkedList.map.get(1)!.next;
    node.value !== 1;
    node = node.next
  ) {
    result += node.value;
  }

  console.log(result);
}

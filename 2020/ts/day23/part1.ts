const cupLabelString = "167248359";

class Node<T> {
  constructor(
    readonly value: T,
    public next: Node<T>,
    // public previous: Node | null = null,
  ) {}

  toString(): string {
    return `(${this.value}) => ${this.next?.toString() ?? null}`;
  }
}

export class LinkedList<T> {
  pointer: Node<T>;
  map: Map<T, Node<T>> = new Map();

  constructor(values: T[]) {
    // deno-lint-ignore no-explicit-any
    const initial = new Node(values[values.length - 1] as T, null as any);
    this.map.set(initial.value, initial);
    this.pointer = initial;
    for (let i = values.length - 2; i >= 0; i--) {
      this.pointer = new Node(values[i], this.pointer);
      this.map.set(values[i], this.pointer);
    }
    initial.next = this.pointer;
  }

  has(value: T): boolean {
    return this.map.has(value);
  }

  insertAfter(value: T, destination: T) {
    const node = this.map.get(destination)!;
    const newNode = new Node(value, node.next);
    this.map.set(value, newNode);
    node.next = newNode;
  }

  removeAfterPointer(): T {
    const afterPointer = this.pointer.next;
    this.pointer.next = afterPointer.next;
    this.map.delete(afterPointer.value);
    return afterPointer.value;
  }

  advancePointer() {
    this.pointer = this.pointer!.next;
  }

  *[Symbol.iterator](): IterableIterator<Node<T>> {
    let current = this.pointer;
    do {
      yield current;
      current = current.next;
    } while (current !== this.pointer);
  }

  toString(): string {
    const output = [];
    for (const node of this) {
      if (this.pointer === node) {
        output.push(`(${node.value})`);
      } else {
        output.push(String(node.value));
      }
    }
    return output.join(" ");
  }
}

type DoMovesOpts = {
  linkedList: LinkedList<number>;
  n: number;
  highestValue: number;
};

export const doNMoves = ({ linkedList, n, highestValue }: DoMovesOpts) => {
  for (let i = 0; i < n; i++) {
    let destination = linkedList.pointer!.value - 1;
    if (destination <= 0) destination = highestValue;

    const removedValues = [
      linkedList.removeAfterPointer(),
      linkedList.removeAfterPointer(),
      linkedList.removeAfterPointer(),
    ];

    while (!linkedList.has(destination)) {
      destination -= 1;
      if (destination <= 0) destination = highestValue;
    }

    let previousValue = destination;

    for (let i = 0; i < 3; i++) {
      const value = removedValues[i];
      linkedList.insertAfter(value, previousValue);
      previousValue = value;
    }
    linkedList.advancePointer();
  }
};

export const cups = cupLabelString.split("").map(Number);

if (import.meta.main) {
  const highestValue = Math.max.apply(null, cups);

  const linkedList = new LinkedList<number>(cups);

  doNMoves({ linkedList, n: 100, highestValue });

  const result = linkedList.toString();

  console.log(result.split("1").reverse().join("").replace(/\D+/g, ""));
}

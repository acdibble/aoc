const NONE = Symbol('none');

class Option<T> {
  private constructor(private readonly value: T | typeof NONE) {}

  static some<T>(value: T): Option<T> {
    return new Option(value);
  }

  static none<T>(): Option<T> {
    return new Option<T>(NONE);
  }

  map<U>(fn: (value: T) => U): Option<U> {
    if (this.value === NONE) {
      return Option.none<U>();
    }
    return Option.some(fn(this.value));
  }

  unwrap(): T {
    if (this.value === NONE) {
      throw new Error('Called unwrap on a None value');
    }

    return this.value;
  }

  isSome(): boolean {
    return this.value !== NONE;
  }
}

const NEIGHBORS = [
  [-1, -1],
  [0, -1],
  [1, -1],
  [-1, 0],
  [1, 0],
  [-1, 1],
  [0, 1],
  [1, 1],
];

class Cell<T> {
  constructor(
    public value: T,
    public readonly x: number,
    public readonly y: number,
    private readonly grid: Grid<T>,
  ) {}

  *neighbors(): IterableIterator<Cell<T>> {
    for (const [dx, dy] of NEIGHBORS) {
      const neighbor = this.grid.at(this.x + dx, this.y + dy);
      if (neighbor.isSome()) {
        yield new Cell(neighbor.unwrap(), this.x + dx, this.y + dy, this.grid);
      }
    }
  }

  set(newValue: T): void {
    this.grid.setUnchecked(this.x, this.y, newValue);
    this.value = newValue;
  }
}

export class Grid<T> {
  static fromString<U>(data: string, rowSep = '\n', colSep = ''): Grid<U> {
    const rows = data.split(rowSep).map((row) => row.split(colSep));
    return new Grid<U>(rows as U[][]);
  }

  constructor(private readonly data: T[][]) {}

  at(x: number, y: number): Option<T> {
    if (y < 0 || y >= this.data.length) return Option.none();

    const row = this.data[y];
    if (x < 0 || x >= row.length) return Option.none();

    return Option.some(row[x]);
  }

  setUnchecked(x: number, y: number, value: T): void {
    this.data[y][x] = value;
  }

  *[Symbol.iterator](): IterableIterator<Cell<T>> {
    for (let y = 0; y < this.data.length; y++) {
      for (let x = 0; x < this.data[y].length; x++) {
        yield new Cell(this.data[y][x], x, y, this);
      }
    }
  }

  print() {
    for (const row of this.data) {
      console.log(row.join(''));
    }
  }
}

export class Range {
  static inclusive(start: number, end: number): Range {
    return new Range(start, end + 1);
  }

  constructor(public readonly start: number, public readonly end: number) {}

  contains(value: number): boolean {
    return value >= this.start && value < this.end;
  }

  join(other: Range): Range | null {
    if (
      this.contains(other.start) ||
      this.contains(other.end) ||
      other.contains(this.start) ||
      other.contains(this.end)
    ) {
      return new Range(Math.min(this.start, other.start), Math.max(this.end, other.end));
    }

    return null;
  }

  get size(): number {
    return this.end - this.start;
  }
}

export const inspect = <T>(value: T): T => {
  console.log(value);
  return value;
};

export const isNotNullish = <T>(value: T | null | undefined): value is T => value != null;

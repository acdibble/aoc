import assert from 'assert';

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

  isNone(): boolean {
    return this.value === NONE;
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

export class Cell<T> {
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
        yield neighbor.unwrap();
      }
    }
  }

  neighborDown(): Option<Cell<T>> {
    return this.grid.at(this.x, this.y + 1);
  }

  neightborLeft(): Option<Cell<T>> {
    return this.grid.at(this.x - 1, this.y);
  }

  neighborRight(): Option<Cell<T>> {
    return this.grid.at(this.x + 1, this.y);
  }

  set(newValue: T): void {
    this.grid.setUnchecked(this.x, this.y, newValue);
    this.value = newValue;
  }

  getKey(): number {
    return this.y * this.grid.width + this.x;
  }
}

export class Grid<T> {
  static fromString<U>(
    data: string,
    mapping: Record<string, U>,
    { rowSep = '\n', colSep = '' }: { rowSep?: string; colSep?: string } = {},
  ): Grid<U> {
    const rows = data.split(rowSep).map((row) =>
      row.split(colSep).map((cell) => {
        const mapped = mapping[cell];
        assert(mapped !== undefined, `No mapping found for cell value: ${cell}`);
        return mapped;
      }),
    );
    return new Grid<U>(rows as U[][]);
  }

  constructor(private readonly data: T[][]) {}

  get width(): number {
    return this.data[0].length;
  }

  get height(): number {
    return this.data.length;
  }

  at(x: number, y: number): Option<Cell<T>> {
    if (y < 0 || y >= this.data.length) return Option.none();

    const row = this.data[y];
    if (x < 0 || x >= row.length) return Option.none();

    return Option.some(new Cell(row[x], x, y, this));
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

export class Point {
  static fromString(input: string): Point {
    const [, x, y] = input.match(/(-?\d+),(-?\d+)/)!;
    return new Point(Number(x), Number(y));
  }

  readonly key: `${number},${number}`;

  constructor(public x: number, public y: number) {
    this.key = `${x},${y}`;
  }

  eq(other: Point): boolean {
    return this.x === other.x && this.y === other.y;
  }

  translate(vector: Vector): Point {
    return new Point(this.x + vector.x, this.y + vector.y);
  }

  vectorTo(other: Point): Vector {
    return new Vector(other.x - this.x, other.y - this.y);
  }

  toString(): `${number},${number}` {
    return this.key;
  }
}

export class Vector {
  readonly key: `${number},${number}`;

  constructor(public x: number, public y: number) {
    this.key = `${x},${y}`;
  }

  zeroX(): Vector {
    return new Vector(0, this.y);
  }

  zeroY(): Vector {
    return new Vector(this.x, 0);
  }

  rotateClockwise(): Vector {
    return new Vector(-this.y, this.x);
  }

  rotateCounterClockwise(): Vector {
    return new Vector(this.y, -this.x);
  }

  toString(): `${number},${number}` {
    return this.key;
  }
}

export class Point3D {
  static fromString(input: string): Point3D {
    const [, x, y, z] = input.match(/(-?\d+),(-?\d+),(-?\d+)/)!;
    return new Point3D(Number(x), Number(y), Number(z));
  }

  constructor(public x: number, public y: number, public z: number) {}

  euclideanDistance(other: Point3D): number {
    return Math.sqrt((this.x - other.x) ** 2 + (this.y - other.y) ** 2 + (this.z - other.z) ** 2);
  }
}

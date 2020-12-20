import { readFile } from "../utils.ts";

enum Direction {
  Top,
  Right,
  Bottom,
  Left,
  None,
}

const isOpposite = (a: Direction, b: Direction) => (a + 2) % 4 === b;
const aRightOfB = (a: Direction, b: Direction) => (a + 3) % 4 === b;
const aLeftOfB = (a: Direction, b: Direction) => (a + 1) % 4 === b;
const neighbors = ["top", "right", "bottom", "left"] as const;

export const rotateImage = (data: string[]): string[] => {
  const newData = data.map(() => "");
  for (let i = data.length - 1; i >= 0; i--) {
    for (let j = 0; j < data[i].length; j++) {
      newData[j] += data[i][j];
    }
  }
  return newData;
};

export class Tile {
  readonly name: string;
  data: string[];
  private readonly edges: string[] = new Array(4);
  private readonly invertedEdges: string[] = new Array(4);

  top: Tile | undefined = undefined;
  right: Tile | undefined = undefined;
  bottom: Tile | undefined = undefined;
  left: Tile | undefined = undefined;

  constructor(input: string) {
    const [, num] = /Tile (\d+):/.exec(input)!;
    this.name = num;
    const data = input.slice(input.indexOf("\n") + 1);
    this.data = data.split("\n");

    this.setEdges();
  }

  neighborCount(): number {
    return neighbors.reduce((acc, n) => acc + Number(this[n] != null), 0);
  }

  private setEdges(): void {
    let left = "";
    let right = "";
    for (let i = 0; i < this.data.length; i++) {
      left += this.data[i][0];
      right += this.data[i][this.data[i].length - 1];
    }

    this.edges[Direction.Top] = this.data[0];
    this.edges[Direction.Right] = right;
    this.edges[Direction.Bottom] = this.data[this.data.length - 1];
    this.edges[Direction.Left] = left;
    this.invertEdges();
  }

  private invertEdges(): void {
    for (let i = 0; i < 4; i++) {
      this.invertedEdges[i] = this.edges[i].split("").reverse().join("");
    }
  }

  private rotateData(): void {
    this.data = rotateImage(this.data);
  }

  private rotate(): Tile {
    this.rotateData();
    this.setEdges();
    return this;
  }

  private flip(type: "vertical" | "horizontal"): Tile {
    if (type === "vertical") {
      this.data = this.data.map((line) => line.split("").reverse().join(""));
    } else {
      this.data.reverse();
    }
    this.setEdges();
    return this;
  }

  someMatch(other: Tile): boolean {
    return this.edges.some((edge) =>
      other.edges.includes(edge) || other.invertedEdges.includes(edge)
    );
  }

  private setLeftRight(other: Tile, dir: "left" | "right"): void {
    const otherDir = dir === "right" ? "left" : "right";
    this[dir] = other;
    if (this.top?.[dir]) {
      this.top[dir]!.bottom = other;
      other.top = this.top[dir];
    }
    if (this.bottom?.[dir]) {
      this.bottom[dir]!.top = other;
      other.bottom = this.bottom[dir];
    }
    other[otherDir] = this;
  }

  private setTopBottom(other: Tile, dir: "top" | "bottom"): void {
    const otherDir = dir === "bottom" ? "top" : "bottom";
    this[dir] = other;
    if (this.right?.[dir]) {
      this.right[dir]!.left = other;
      other.right = this.right[dir];
    }
    if (this.left?.[dir]) {
      this.left[dir]!.right = other;
      other.left = this.left[dir];
    }
    other[otherDir] = this;
  }

  private set(other: Tile, direction: Direction): void {
    if (direction === Direction.Top) return this.setTopBottom(other, "top");
    if (direction === Direction.Bottom) {
      return this.setTopBottom(other, "bottom");
    }
    if (direction === Direction.Left) return this.setLeftRight(other, "left");
    if (direction === Direction.Right) return this.setLeftRight(other, "right");
  }

  makeFit(other: Tile): void {
    const thisSide: Direction = this.edges.findIndex((edge) =>
      other.edges.includes(edge) || other.invertedEdges.includes(edge)
    );
    let otherSide: Direction | -1 = other.edges
      .findIndex((edge) => edge === this.edges[thisSide]);

    if (otherSide !== -1) {
      if (thisSide === otherSide) {
        const type = thisSide === Direction.Top || thisSide === Direction.Bottom
          ? "horizontal"
          : "vertical";
        other.flip(type);
      } else if (aLeftOfB(otherSide, thisSide)) {
        other.rotate().rotate().rotate();
      } else if (aRightOfB(otherSide, thisSide)) {
        other.rotate();
      }
    } else {
      otherSide = other.invertedEdges
        .findIndex((edge) => edge === this.edges[thisSide]);

      if (thisSide === otherSide) {
        other.rotate().rotate();
      } else if (isOpposite(thisSide, otherSide)) {
        const type = thisSide === Direction.Top || thisSide === Direction.Bottom
          ? "vertical"
          : "horizontal";
        other.flip(type);
      } else if (thisSide === Direction.Right || thisSide === Direction.Left) {
        if (aRightOfB(otherSide, thisSide)) {
          other.rotate().flip("horizontal");
        } else if (aLeftOfB(otherSide, thisSide)) {
          other.rotate().rotate().rotate();
        }
      } else if (thisSide === Direction.Top || thisSide === Direction.Bottom) {
        if (aLeftOfB(otherSide, thisSide)) {
          other.flip("vertical").rotate();
        } else if (aRightOfB(otherSide, thisSide)) {
          other.rotate();
        }
      }
    }

    return this.set(other, thisSide);
  }
}

export const linkTiles = async () => {
  const unplacedTiles = (await readFile(import.meta.url)).split("\n\n")
    .map((tile) => new Tile(tile));

  const placedTiles: Tile[] = [unplacedTiles.shift()!];

  for (
    let unplaced = unplacedTiles.shift();
    unplaced;
    unplaced = unplacedTiles.shift()
  ) {
    const matches = placedTiles.filter((tile) => tile.someMatch(unplaced!));
    if (!matches.length) {
      unplacedTiles.push(unplaced);
    } else {
      matches.forEach((match) => match.makeFit(unplaced!));
      placedTiles.push(unplaced);
    }
  }

  return placedTiles;
};

export const findCorners = (tiles: Tile[]): Tile[] =>
  tiles.filter((t) => {
    return t.neighborCount() === 2 &&
      // some bug :shrug:
      neighbors.every((n) => (t[n]?.neighborCount() ?? 3) === 3);
  });

if (import.meta.main) {
  const tiles = await linkTiles();
  console.log(
    findCorners(tiles).map((t) => t.name).reduce(
      (acc, num) => acc * Number(num),
      1,
    ),
  );
}

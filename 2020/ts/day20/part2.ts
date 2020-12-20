import { unreachable } from "https://deno.land/std@0.79.0/testing/asserts.ts";
import { readFile } from "../utils.ts";

enum Direction {
  Top,
  Right,
  Bottom,
  Left,
  None,
}

const rotateImage = (data: string[]): string[] => {
  const newData = data.map(() => "");
  for (let i = data.length - 1; i >= 0; i--) {
    for (let j = 0; j < data[i].length; j++) {
      newData[j] += data[i][j];
    }
  }
  return newData;
};

class Tile {
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
    const neighbors = ["top", "right", "bottom", "left"] as const;
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

  private setLeft(other: Tile): void {
    return this.setLeftRight(other, "left");
  }

  private setRight(other: Tile): void {
    return this.setLeftRight(other, "right");
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

  private setTop(other: Tile): void {
    return this.setTopBottom(other, "top");
  }

  private setBottom(other: Tile): void {
    return this.setTopBottom(other, "bottom");
  }

  makeFit(other: Tile): void {
    const thisSide: Direction = this.edges.findIndex((edge) =>
      other.edges.includes(edge) || other.invertedEdges.includes(edge)
    );
    let otherSide: Direction | -1 = other.edges
      .findIndex((edge) => edge === this.edges[thisSide]);

    if (otherSide !== -1) {
      if (thisSide === Direction.Top) {
        if (otherSide === Direction.Top) {
          other.flip("horizontal");
        } else if (otherSide === Direction.Left) {
          other.rotate().rotate().rotate();
        } else if (otherSide === Direction.Right) {
          unreachable();
        }
        return this.setTop(other);
      }

      if (thisSide === Direction.Bottom) {
        if (otherSide === Direction.Right) {
          other.rotate().rotate().rotate();
        } else if (otherSide === Direction.Bottom) {
          other.flip("horizontal");
        } else if (otherSide === Direction.Left) {
          unreachable();
        }
        return this.setBottom(other);
      }

      if (thisSide === Direction.Left) {
        if (otherSide === Direction.Top) {
          other.rotate();
        } else if (otherSide === Direction.Left) {
          other.flip("vertical");
        } else if (otherSide === Direction.Bottom) {
          unreachable();
        }
        return this.setLeft(other);
      }

      if (thisSide === Direction.Right) {
        if (otherSide === Direction.Bottom) {
          other.rotate();
        } else if (otherSide === Direction.Right) {
          other.flip("vertical");
        } else if (otherSide === Direction.Top) {
          unreachable();
        }
        return this.setRight(other);
      }

      unreachable();
    }

    otherSide = other.invertedEdges
      .findIndex((edge) => edge === this.edges[thisSide]);

    if (thisSide === Direction.Right) {
      if (otherSide === Direction.Right) {
        other.rotate().rotate();
      } else if (otherSide === Direction.Left) {
        other.flip("horizontal");
      } else if (otherSide === Direction.Bottom) {
        other.rotate().flip("horizontal");
      } else if (otherSide === Direction.Top) {
        other.rotate().rotate().rotate();
      }
      return this.setRight(other);
    }

    if (thisSide === Direction.Top) {
      if (otherSide === Direction.Left) {
        other.rotate().rotate().rotate().flip("vertical");
      } else if (otherSide === Direction.Bottom) {
        other.flip("vertical");
      } else if (otherSide === Direction.Right) {
        other.rotate();
      } else if (otherSide === Direction.Top) {
        other.rotate().rotate();
      }
      return this.setTop(other);
    }

    if (thisSide === Direction.Bottom) {
      if (otherSide === Direction.Bottom) {
        other.rotate().rotate();
      } else if (otherSide === Direction.Left) {
        other.rotate();
      } else if (otherSide === Direction.Top) {
        other.flip("vertical");
      } else if (otherSide === Direction.Right) {
        other.flip("vertical").rotate();
      }
      return this.setBottom(other);
    }

    if (thisSide === Direction.Left) {
      if (otherSide === Direction.Top) {
        other.rotate().flip("horizontal");
      } else if (otherSide === Direction.Left) {
        other.rotate().rotate();
      } else if (otherSide === Direction.Bottom) {
        other.rotate().rotate().rotate();
      } else if (otherSide === Direction.Right) {
        other.flip("horizontal");
      }
      return this.setLeft(other);
    }

    unreachable();
  }
}

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
    continue;
  }
  matches.forEach((match) => match.makeFit(unplaced!));
  placedTiles.push(unplaced);
}

const corners = placedTiles.filter((t) => {
  const dirs = ["top", "bottom", "left", "right"] as const;
  return t.neighborCount() === 2 &&
    dirs.every((dir) => (t[dir]?.neighborCount() ?? 3) === 3);
});

let rowStart: Tile | undefined = corners.find((c) => c.left === c.top)!;
let current: Tile | undefined = rowStart;

let picture: string[] = [];
let offset = 0;

while (current) {
  current.data.slice(1, -1).map((l) => l.slice(1, -1)).forEach((line, i) => {
    picture[offset + i] ||= "";
    picture[offset + i] += line;
  });
  const next = current.right ?? current.top?.right?.bottom ??
    current.bottom?.right?.top;
  if (!next) {
    offset += 8;
    rowStart = rowStart?.bottom;
    current = rowStart;
  } else {
    current = next;
  }
}

const re1 = /(?<=.{18})#(?=.)/g;
const re2 = /#....##....##....###/;
const re3 = /.#..#..#..#..#..#.../;

let matchesFound = 0;
for (let n = 0; n < 4; n++) {
  picture = rotateImage(picture);
  for (let k = 0; k < 2; k++) {
    if (k === 1) {
      picture = picture.map((l) => l.split("").reverse().join(""));
    }

    for (let i = 0; i < picture.length - 1; i++) {
      for (const match of picture[i].matchAll(re1)) {
        if (
          re2.test(picture[i + 1].slice(match.index! - 18, match.index! + 2)) &&
          re3.test(picture[i + 2].slice(match.index! - 18, match.index! + 2))
        ) {
          matchesFound += 1;
        }
      }
    }
  }
  if (matchesFound > 0) break;
}

console.log(picture.join("").replace(/[^#]/g, "").length - matchesFound * 15);

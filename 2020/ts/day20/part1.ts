import { unreachable } from "https://deno.land/std@0.79.0/testing/asserts.ts";
import { readFile } from "../utils.ts";

enum Direction {
  Top,
  Right,
  Bottom,
  Left,
  None,
}

class Tile {
  readonly name: string;
  private data: string[];
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
    const newData = this.data.map(() => "");
    for (let i = this.data.length - 1; i >= 0; i--) {
      for (let j = 0; j < this.data[i].length; j++) {
        newData[j] += this.data[i][j];
      }
    }
    this.data = newData;
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

  private printData(): void {
    console.log(this.data.join("\n"));
    console.log();
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

  private debug(other: Tile): never {
    this.printData();
    other.printData();
    unreachable();
  }

  makeFit(other: Tile): void {
    if (this.name === "2099") {
      console.log(other.name);
    }
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
  } else {
    matches.forEach((match) => match.makeFit(unplaced!));
    placedTiles.push(unplaced);
  }
}

console.log(
  placedTiles.filter((t) => {
    const dirs = ["top", "bottom", "left", "right"] as const;
    return t.neighborCount() === 2 &&
      // some bug :shrug:
      dirs.every((dir) => (t[dir]?.neighborCount() ?? 3) === 3);
  }).map((t) => t.name).reduce((acc, num) => acc * Number(num), 1),
);

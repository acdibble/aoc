type color = Red | Green | Blue

type set = {
  red: int,
  green: int,
  blue: int,
}

type game = {
  id: int,
  sets: list<set>,
}

let integer =
  P.many1(P.digit)
  ->P.map(v => List.toArray(v)->Array.join("")->Int.fromString->Option.getExn)
  ->P.label("integer")

let color =
  P.choice([
    P.string("red")->P.map(_ => Red),
    P.string("green")->P.map(_ => Green),
    P.string("blue")->P.map(_ => Blue),
  ])->P.label("color")

let cube = P.keepLeft(integer, P.char(" "))->P.andThen(color)

let cubeSet = P.sepBy1(cube, P.string(", "))->P.map(v => List.toArray(v))

let findColor = (cubes, sought) =>
  Array.findMap(cubes, ((count, color)) =>
    if color == sought {
      Some(count)
    } else {
      None
    }
  )->Option.getOr(0)

let gameParser =
  P.between(P.string("Game "), integer, P.string(": "))
  ->P.andThen(P.sepBy1(cubeSet, P.string("; ")))
  ->P.map(((id, sets)) => {
    id,
    sets: List.map(sets, cubes => {
      red: findColor(cubes, Red),
      green: findColor(cubes, Green),
      blue: findColor(cubes, Blue),
    }),
  })

let solve = async (filterFn, mapFn) => {
  (await Utils.readInput(%raw("import.meta.dirname")))
  ->String.split("\n")
  ->Array.map(l =>
    switch P.run(gameParser, l) {
    | Ok((value, _)) => value
    | Error(_) => failwith("Failed to parse game")
    }
  )
  ->Array.filterMap(g =>
    if List.every(g.sets, filterFn) {
      Some(mapFn(g))
    } else {
      None
    }
  )
  ->Array.reduce(0, (a, b) => a + b)
}

let part1 = () => solve(set => set.red <= 12 && set.green <= 13 && set.blue <= 14, g => g.id)

part1()->Promise.thenResolve(Js.log2("Part 1:", _))->ignore

let part2 = () =>
  solve(
    _ => true,
    g => {
      let findMaxForColor = color =>
        List.reduce(g.sets, 0, (acc, c) =>
          max(
            acc,
            switch color {
            | Red => c.red
            | Green => c.green
            | Blue => c.blue
            },
          )
        )
      let red = findMaxForColor(Red)
      let green = findMaxForColor(Green)
      let blue = findMaxForColor(Blue)
      red * green * blue
    },
  )

part2()->Promise.thenResolve(Js.log2("Part 2:", _))->ignore

let anything = P.satisfy(_ => true)->P.map(_ => None)

let solve = async parser =>
  (await Utils.readInput(%raw("import.meta.dirname")))
  ->String.split("\n")
  ->Array.map(l =>
    switch parser
    ->P.orElse(anything)
    ->P.many
    ->P.map(v => v->List.toArray->Array.keepSome)
    ->P.run(l) {
    | Ok((ints, _)) => ints
    | Error(err) => failwith(err)
    }
  )
  ->Array.reduce(0, (acc, ints) => {
    let first = ints[0]->Option.getExn

    let last = Array.last(ints)->Option.getOr(first)

    acc + (first * 10 + last)
  })

let part1 = () => solve(P.digit->P.map(v => Int.fromString(v)))

part1()->Promise.thenResolve(Js.log2("part 1:", _))->ignore

let nongreedy = parser => P.Parser(
  input => P.run(parser, input)->Result.map(((v, _)) => (v, String.sliceToEnd(input, ~start=1))),
)

let part2 = () =>
  solve(
    [
      ("one", "1"),
      ("two", "2"),
      ("three", "3"),
      ("four", "4"),
      ("five", "5"),
      ("six", "6"),
      ("seven", "7"),
      ("eight", "8"),
      ("nine", "9"),
    ]
    ->Array.map(((word, digit)) =>
      P.string(word)->nongreedy->P.orElse(P.char(digit))->P.map(_ => Int.fromString(digit))
    )
    ->P.choice,
  )

part2()->Promise.thenResolve(Js.log2("part 2:", _))->ignore

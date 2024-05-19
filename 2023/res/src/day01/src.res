let solve = async (cb: string => array<int>) =>
  (await Utils.readInput(%raw("import.meta.dirname")))
  ->String.split("\n")
  ->Array.map(cb)
  ->Array.reduce(0, (acc, ints) => {
    let first = ints[0]->Option.getExn

    let last = Array.last(ints)->Option.getOr(first)

    acc + (first * 10 + last)
  })

let part1 = () =>
  solve(v => {
    String.split(v, "")
    ->Array.map(v => Int.fromString(v))
    ->Array.keepSome
  })

part1()
->Promise.thenResolve(v => Js.log(`part 1: ${Int.toString(v)}`))
->ignore

let part2 = () => {
  let re = %re("/^(\d|nine|eight|seven|six|five|four|three|two|one){1}/")

  let exec = RegExp.exec(re, _)

  solve(v => {
    Belt.Array.range(0, String.length(v) - 1)
    ->Array.map(i => String.sliceToEnd(v, ~start=i)->exec->Option.flatMap(arr => arr[0]))
    ->Array.keepSome
    ->Array.map(v => {
      switch v {
      | "one" => 1
      | "two" => 2
      | "three" => 3
      | "four" => 4
      | "five" => 5
      | "six" => 6
      | "seven" => 7
      | "eight" => 8
      | "nine" => 9
      | _ => Int.fromString(v)->Option.getExn
      }
    })
  })
}

part2()
->Promise.thenResolve(v => Js.log(`part 2: ${Int.toString(v)}`))
->ignore

@module("fs/promises")
external readFile: (
  string,
  [
    | #utf8
  ],
) => promise<string> = "readFile"

let readInput = (dirname: string): promise<string> =>
  dirname
  ->Path.join("data.txt")
  ->readFile(#utf8)

let inspect = v => {
  Console.log(v)
  v
}

let identity = v => v

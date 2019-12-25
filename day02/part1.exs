Code.require_file("lib/IntcodeComputer.exs")

IntcodeComputer.start("day02/data.txt", %{ 1 => 12, 2 => 2 })
|> IO.inspect()

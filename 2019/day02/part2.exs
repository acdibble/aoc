Code.require_file("lib/IntcodeComputer.exs")

intcodes = IntcodeComputer.read_file("day02/data.txt")

for noun <- 0..99 do
  for verb <- 0..99 do
    IntcodeComputer.start(intcodes, %{ 1 => noun, 2 => verb })
    |> case do
      { :ok, 19690720 } ->
        (noun * 100) + verb
      _ ->
        nil
    end
  end
end
|> List.flatten()
|> Enum.reject(&is_nil/1)
|> List.first()
|> IO.inspect()

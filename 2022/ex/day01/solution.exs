elves =
  File.stream!("day01/data.txt")
  |> Stream.map(
    &case Integer.parse(&1) do
      {value, _} -> value
      :error -> nil
    end
  )
  |> Enum.reduce([0], fn value, [head | tail] ->
    case value do
      nil -> [0, head | tail]
      _ -> [head + value | tail]
    end
  end)
  |> Enum.sort(:desc)

List.first(elves) |> IO.inspect()

Enum.take(elves, 3) |> Enum.sum() |> IO.inspect()

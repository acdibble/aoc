defmodule Part1 do
  def score(["A", "X"], current), do: current + 1 + 3
  def score(["A", "Y"], current), do: current + 2 + 6
  def score(["A", "Z"], current), do: current + 3
  def score(["B", "X"], current), do: current + 1
  def score(["B", "Y"], current), do: current + 2 + 3
  def score(["B", "Z"], current), do: current + 3 + 6
  def score(["C", "X"], current), do: current + 1 + 6
  def score(["C", "Y"], current), do: current + 2
  def score(["C", "Z"], current), do: current + 3 + 3
end

defmodule Part2 do
  def score(["A", "X"], current), do: current + 3
  def score(["A", "Y"], current), do: current + 1 + 3
  def score(["A", "Z"], current), do: current + 2 + 6
  def score(["B", "X"], current), do: current + 1
  def score(["B", "Y"], current), do: current + 2 + 3
  def score(["B", "Z"], current), do: current + 3 + 6
  def score(["C", "X"], current), do: current + 2
  def score(["C", "Y"], current), do: current + 3 + 3
  def score(["C", "Z"], current), do: current + 1 + 6
end

games =
  File.stream!("day02/data.txt")
  |> Stream.map(&String.trim/1)
  |> Stream.map(&String.split/1)
  |> Enum.to_list()

games
|> Enum.reduce(0, &Part1.score/2)
|> IO.inspect(label: "part one")

games
|> Enum.reduce(0, &Part2.score/2)
|> IO.inspect(label: "part two")

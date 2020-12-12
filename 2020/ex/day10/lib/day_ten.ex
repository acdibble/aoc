defmodule DayTen do
  defp load_file(path) do
    __ENV__.file
    |> Path.dirname()
    |> Path.join(path)
    |> File.stream!()
    |> Stream.map(&String.trim/1)
    |> Stream.map(&String.to_integer/1)
  end

  def part_one do
    load_file("../data.txt")
    |> Enum.into([])
    |> Enum.sort()
    |> count_diffs()
  end

  def part_two do
    load_file("../data.txt")
    |> Enum.into([])
    |> Enum.concat([0])
    |> Enum.sort(:desc)
    |> get_path_count()
  end

  defp count_diffs(adapters, ones \\ 1, threes \\ 1, previous_diff \\ nil)

  defp count_diffs(adapters, ones, threes, 1), do: count_diffs(adapters, ones + 1, threes, nil)
  defp count_diffs(adapters, ones, threes, 3), do: count_diffs(adapters, ones, threes + 1, nil)
  defp count_diffs([], ones, threes, nil), do: ones * threes

  defp count_diffs([first, second | []], ones, threes, nil),
    do: count_diffs([], ones, threes, second - first)

  defp count_diffs([first, second | rest], ones, threes, nil),
    do: count_diffs([second | rest], ones, threes, second - first)

  defp get_path_count(nums) do
    Enum.reduce(nums, %{(Enum.at(nums, 0) + 3) => 1}, fn current, acc ->
      Range.new(current + 1, current + 3)
      |> Enum.map(&Map.get(acc, &1, 0))
      |> Enum.sum()
      |> (&Map.put(acc, current, &1)).()
    end)
    |> Map.get(0)
  end
end

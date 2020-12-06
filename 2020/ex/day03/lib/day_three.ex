defmodule DayThree do
  defp load_file(path) do
    __ENV__.file
    |> Path.dirname()
    |> Path.join(path)
    |> File.stream!()
    |> Stream.map(&String.trim/1)
  end

  def part_one do
    load_file("../data.txt")
    |> Enum.into([])
    |> count_trees()
  end

  def part_two do
    lines = load_file("../data.txt") |> Enum.into([])

    [{1, 1}, {3, 1}, {5, 1}, {7, 1}, {1, 2}]
    |> Enum.reduce(1, fn {run, rise}, acc ->
      Enum.take_every(lines, rise)
      |> count_trees(run)
      |> Kernel.*(acc)
    end)
  end

  defp count_trees(lines, run \\ 3, x \\ 0, count \\ 0)

  defp count_trees([], _run, _x, count), do: count

  defp count_trees([line | lines], run, x, count) do
    count_trees(
      lines,
      run,
      rem(x + run, String.length(line)),
      count + get_val(String.at(line, x))
    )
  end

  defp get_val("."), do: 0
  defp get_val("#"), do: 1
end

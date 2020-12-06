defmodule DaySix do
  defp load_file(path) do
    __ENV__.file
    |> Path.dirname()
    |> Path.join(path)
    |> File.read!()
  end

  def part_one do
    load_sets("../data.txt")
    |> combine_sets(&MapSet.union/2)
  end

  def part_two do
    load_sets("../data.txt")
    |> combine_sets(&MapSet.intersection/2)
  end

  defp load_sets(path) do
    load_file(path)
    |> String.split("\n\n")
    |> Enum.map(fn group ->
      String.split(group, "\n")
      |> Enum.map(&(String.trim(&1) |> String.split("", trim: true) |> MapSet.new()))
    end)
  end

  defp combine_sets(all_sets, method) do
    all_sets
    |> Enum.reduce([], fn sets, acc ->
      Enum.reduce(sets, &method.(&1, &2))
      |> (&[&1 | acc]).()
    end)
    |> Enum.reduce(0, &(MapSet.size(&1) + &2))
  end
end

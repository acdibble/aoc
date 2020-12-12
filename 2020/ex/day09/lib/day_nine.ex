defmodule DayNine do
  defp load_file(path) do
    __ENV__.file
    |> Path.dirname()
    |> Path.join(path)
    |> File.stream!()
    |> Stream.map(&String.trim/1)
    |> Stream.map(&String.to_integer/1)
    |> Stream.with_index()
    |> Enum.into(%{}, &{elem(&1, 1), elem(&1, 0)})
  end

  def part_one do
    load_file("../data.txt")
    |> find_number()
  end

  def part_two do
    nums = load_file("../data.txt")

    nums
    |> find_number()
    |> find_weakness(nums)
  end

  defp find_number(map, index \\ 25, cont \\ true)
  defp find_number(map, index, false), do: map[index - 1]

  defp find_number(map, index, true) do
    previous_nums =
      Range.new(index - 25, index - 1)
      |> Enum.map(&map[&1])

    find_number(
      map,
      index + 1,
      Enum.any?(previous_nums, &((map[index] - &1) in previous_nums))
    )
  end

  defp find_weakness(value, map) do
    Range.new(25, map_size(map) - 1)
    |> Enum.find_value(&sum(value, map, &1))
  end

  defp sum(target, map, index, nums \\ [], total \\ 0)
  defp sum(target, _map, _index, _nums, total) when total > target, do: nil

  defp sum(target, _map, _index, nums, total) when total == target,
    do: Enum.max(nums) + Enum.min(nums)

  defp sum(target, map, index, nums, total) do
    next = map[index]
    sum(target, map, index + 1, [next | nums], total + next)
  end
end

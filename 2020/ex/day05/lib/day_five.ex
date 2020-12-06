defmodule DayFive do
  defp load_file(path) do
    __ENV__.file
    |> Path.dirname()
    |> Path.join(path)
    |> File.stream!()
    |> Stream.map(&String.trim/1)
  end

  def part_one do
    load_file("../data.txt")
    |> Stream.map(&calculate_id/1)
    |> Enum.max()
  end

  def part_two do
    ids =
      load_file("../data.txt")
      |> Stream.map(&calculate_id/1)

    expected_sum =
      Enum.min(ids)..Enum.max(ids)
      |> Enum.sum()

    expected_sum - Enum.sum(ids)
  end

  defp calculate_id(seat, total \\ 0)

  defp calculate_id("", total), do: total

  defp calculate_id(<<char, rest::binary>>, total) when char === ?B or char === ?R,
    do: calculate_id(rest, shift_with(total, 1))

  defp calculate_id(<<_char, rest::binary>>, total), do: calculate_id(rest, shift_with(total))

  defp shift_with(num, bit \\ 0), do: Bitwise.<<<(num, 1) |> Bitwise.|||(bit)
end

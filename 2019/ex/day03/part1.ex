defmodule DayThree.PartOne do
  use FileReader

  def run do
    # wire_1 = HashSet.new()
    # wire_2 = HashSet.new()
    data =
      read_file!()
      |> String.split("\n")
      |> Enum.map(&String.split(&1, ","))
      |> Enum.at(0)
      |> lay_wire()
  end

  defp lay_wire(segments, set \\ HashSet.new(), x \\ 0, y \\ 0)

  defp lay_wire([head | tail], set, x, y) do
    updated =
      if String.starts_with?(head, ["U", "D"]) do
        nil
      end

    lay_wire(tail)
  end

  defp lay_wire([], set, x, y), do: set
end

defmodule DayTwo do
  import FileLoader

  def calculate_needs(type, opts \\ []) do
    path = Keyword.get(opts, :path)
    Keyword.get(opts, :data, read_file(path))
    |> String.split("\n")
    |> crunch_numbers(0, type)
  end

  defp crunch_numbers([current | rest], total_area, type) do
    crunch_numbers(rest, calculate_current(current, type) + total_area, type)
  end

  defp crunch_numbers([], total_area, _type) do
    total_area
  end

  defp calculate_current(dimensions, type) do
    [small, middle, big] = dimensions
    |> String.split("x")
    |> Enum.map(&String.to_integer/1)
    |> Enum.sort()

    case type do
      :paper ->
        (small * middle) + (2 * small * middle) + (2 * middle * big) + (2 * big * small)
      :ribbon ->
        ((2 * small) + (2 * middle)) + (small * middle * big)
    end
  end
end

defmodule DayOne do
  import FileLoader
  def count_floors(opts \\ []) do
    path = Keyword.get(opts, :path, "#{__DIR__}/../input.txt")
    data = Keyword.get(opts, :data, read_file(path))
    find_first_basement = Keyword.get(opts, :find_first_basement, false)

    count_floor(data, 0, 0, find_first_basement)
  end

  defp count_floor(_, -1, counter, true) do
    counter
  end

  defp count_floor(<<"(" , rest::binary>>, acc, counter, find_first_basement) do
    count_floor(rest, acc + 1, counter + 1, find_first_basement)
  end

  defp count_floor(<<")", rest::binary>>, acc, counter, find_first_basement) do
    count_floor(rest, acc - 1, counter + 1, find_first_basement)
  end

  defp count_floor("", acc, _counter, _find_first_basement) do
    acc
  end
end

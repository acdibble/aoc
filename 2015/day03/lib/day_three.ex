defmodule DayThree do
  import FileLoader

  def start_count(opts \\ []) do
    path = Keyword.get(opts, :path)
    data = Keyword.get(opts, :data, read_file(path))
    santa_count = Keyword.get(opts, :santa_count, 1)

    count_houses(
      MapSet.new(["0,0"]),
      data,
      for _ <- 0..(santa_count - 1) do %{ x: 0, y: 0 } end,
      0,
      santa_count
    )
  end

  defp count_houses(set, <<char, rest::binary>>, coords, ptr, santa_count) do
    %{ x: x, y: y } = Enum.fetch!(coords, ptr)

    updated_coords = case char do
      ?^ ->
        %{ x: x, y: y + 1 }
      ?> ->
        %{ x: x + 1, y: y }
      ?v ->
        %{ x: x, y: y - 1 }
      ?< ->
        %{ x: x - 1, y: y }
    end
    new_set = MapSet.put(set, "#{Map.fetch!(updated_coords, :x)},#{Map.fetch!(updated_coords, :y)}")

    count_houses(
      new_set,
      rest,
      for n <- 0..(santa_count - 1) do if (n == ptr), do: updated_coords, else: Enum.fetch!(coords, n) end,
      rem(ptr + 1, santa_count),
      santa_count
    )
  end

  defp count_houses(set, "", coords, ptr, _santa_count) do
    %{ x: x, y: y } = Enum.fetch!(coords, ptr)
    MapSet.put(set, "#{x},#{y}")
    |> MapSet.size()
  end
end

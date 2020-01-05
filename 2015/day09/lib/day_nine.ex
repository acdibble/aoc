defmodule DayNine do
  import FileLoader

  def get_route(opts \\ []) do
    route_type = Keyword.get(opts, :route_type, :shortest)

    {comparator, starting_val} =
      if route_type == :shortest, do: {&min/2, :infinity}, else: {&max/2, 0}

    dict =
      read_file(opts)
      |> String.split("\n")
      |> build_dict()

    places = Map.keys(dict)

    Enum.reduce(places, starting_val, fn current, current_dist ->
      walk(dict, current, starting_val, comparator, Enum.reject(places, &(&1 == current)))
      |> comparator.(current_dist)
    end)
  end

  @regex ~r/^(?<src>[A-Za-z]+) to (?<dest>[A-Za-z]+) = (?<dist>[0-9]+)$/

  defp build_dict(routes) do
    Enum.map(routes, fn route -> Regex.named_captures(@regex, route) end)
    |> Enum.reduce(%{}, fn %{"src" => src, "dest" => dest, "dist" => dist}, dict ->
      int_distance = String.to_integer(dist)

      Map.update(dict, src, %{dest => int_distance}, &Map.put_new(&1, dest, int_distance))
      |> Map.update(dest, %{src => int_distance}, &Map.put_new(&1, src, int_distance))
    end)
  end

  defp walk(_dict, _current, _start, _comp, available) when length(available) == 0, do: 0

  defp walk(dict, current, start, comp, available) do
    Map.get(dict, current)
    |> Map.keys()
    |> Enum.filter(&(&1 in available))
    |> Enum.reduce(start, fn dest, acc ->
      Map.get(dict, current)
      |> Map.get(dest)
      |> Kernel.+(walk(dict, dest, start, comp, Enum.reject(available, &(&1 == dest))))
      |> comp.(acc)
    end)
  end
end

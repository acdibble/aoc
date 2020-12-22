defmodule DayTwelve do
  @bearing_to_letter_map %{north: "N", east: "E", south: "S", west: "W"}
  @letter_to_direction_map %{"R" => :right, "L" => :left}
  @right_map %{north: :east, east: :south, south: :west, west: :north}
  @left_map %{north: :west, east: :north, south: :east, west: :south}
  @direction_map %{right: @right_map, left: @left_map}

  defp load_file(path \\ "../data.txt") do
    __ENV__.file
    |> Path.dirname()
    |> Path.join(path)
    |> File.stream!()
    |> Stream.map(&String.trim/1)
  end

  def part_one do
    load_file()
    |> Enum.into([])
    |> move_ship()
  end

  defp move_ship(movements, ship_state \\ {:east, 0, 0})

  defp move_ship([], {_bearing, northing, easting}), do: abs(northing) + abs(easting)

  defp move_ship(["F" <> amount | tail], {bearing, northing, easting}) do
    move_ship([@bearing_to_letter_map[bearing] <> amount | tail], {bearing, northing, easting})
  end

  defp move_ship(["N" <> amount | tail], {bearing, northing, easting}),
    do: move_ship(tail, {bearing, northing + String.to_integer(amount), easting})

  defp move_ship(["E" <> amount | tail], {bearing, northing, easting}),
    do: move_ship(tail, {bearing, northing, easting + String.to_integer(amount)})

  defp move_ship(["S" <> amount | tail], {bearing, northing, easting}),
    do: move_ship(tail, {bearing, northing - String.to_integer(amount), easting})

  defp move_ship(["W" <> amount | tail], {bearing, northing, easting}),
    do: move_ship(tail, {bearing, northing, easting - String.to_integer(amount)})

  defp move_ship([<<letter::bytes-size(1)>> <> amount | tail], {bearing, northing, easting}) do
    direction = @letter_to_direction_map[letter]

    move_ship(tail, {rotate(bearing, direction, String.to_integer(amount)), northing, easting})
  end

  defp rotate(bearing, _direction, 0), do: bearing
  defp rotate(bearing, dir, amount), do: rotate(@direction_map[dir][bearing], dir, amount - 90)
end

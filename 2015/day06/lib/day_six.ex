defmodule DaySix do
  import FileLoader

  @command_regex ~r/(?<cmd>.+?) (?<x1>\d+),(?<y1>\d+).+?(?<x2>\d+),(?<y2>\d+)/

  def process_instructions(opts \\ []) do
    path = Keyword.get(opts, :path)
    method = Keyword.get(opts, :method)
    Keyword.get(opts, :data, read_file(path))
    |> String.split("\n")
    |> Enum.map(fn string -> parse_instruction(string, method) end)
    |> run_lights(%{}, method)
  end

  defp parse_instruction(string, method) do
    %{
      "cmd" => cmd,
      "x1" => x1,
      "y1" => y1,
      "x2" => x2,
      "y2" => y2,
    } = Regex.named_captures(@command_regex, string)

    fun = get_fun(cmd, method)

    [low_x, high_x] = Enum.sort([String.to_integer(x1), String.to_integer(x2)])
    [low_y, high_y] = Enum.sort([String.to_integer(y1), String.to_integer(y2)])

    coords = Stream.flat_map(low_x..high_x, fn x ->
      Stream.map(low_y..high_y, fn y ->
        "#{x},#{y}"
      end)
    end)

    %{ fun: fun, coords: coords }
  end

  defp get_fun(cmd, :incorrect) do
    case cmd do
      "turn on" ->
        &turn_on/1
      "turn off" ->
        &turn_off/1
      "toggle" ->
        &toggle/1
    end
  end

  defp get_fun(cmd, :correct) do
    case cmd do
      "turn on" ->
        &increment/1
      "turn off" ->
        &decrement/1
      "toggle" ->
        &increment_two/1
    end
  end

  defp turn_on(_current), do: true
  defp turn_off(_current), do: false
  defp toggle(current), do: !current
  defp increment(current), do: current + 1
  defp decrement(current), do: if current - 1 < 0, do: 0, else: current - 1
  defp increment_two(current), do: current + 2

  defp run_lights([%{ fun: fun, coords: coords } | rest], state, method) do
    default = if method == :incorrect, do: false, else: 0
    new_state = Enum.reduce(coords, state, fn coord, acc ->
      Map.put(acc, coord, fun.(Map.get(acc, coord, default)))
    end)
    run_lights(rest, new_state, method)
  end

  defp run_lights([], state, method) do
    Map.values(state) |> calculate_output(method)
  end

  defp calculate_output(values, :incorrect), do: Enum.count(values, fn el -> el end)
  defp calculate_output(values, :correct), do: Enum.sum(values)
end

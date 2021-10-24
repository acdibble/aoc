defmodule DaySeven do
  import FileLoader
  import Bitwise

  def construct_circuitboard(opts \\ []) when is_list(opts) do
    wire_to_output = Keyword.get(opts, :wire_to_output, "a")
    run_part_two = Keyword.get(opts, :run_part_two, false)

    parsed_circuits =
      read_file(opts)
      |> String.split("\n")
      |> Enum.map(&parse_circuit/1)

    first_run = process_circuits(parsed_circuits)

    if run_part_two do
      process_circuits(parsed_circuits, %{"b" => Map.get(first_run, "a")})
    else
      first_run
    end
    |> Map.get(wire_to_output)
  end

  @regex1 ~r/^(?<right>[a-z0-9]+) -> (?<out>[a-z]+)$/
  @regex2 ~r/^(?<left>[0-9a-z]+)? ?(?<op>[A-Z]+)? ?(?<right>[0-9a-z]+) -> (?<out>[a-z]+)$/

  defp parse_circuit(circuit) do
    case(Regex.named_captures(@regex1, circuit)) do
      nil -> Regex.named_captures(@regex2, circuit)
      match -> match |> Map.merge(%{"left" => "", "op" => ""})
    end
  end

  defp process_circuits({a, b}) when is_list(a) and is_map(b), do: process_circuits(a, b)

  defp process_circuits(matches, wire_map \\ %{})

  defp process_circuits([current | rest], wire_map) do
    case handle_match(current, wire_map) do
      {:ok, map} -> {rest, map}
      {:no, map} -> {rest ++ [current], map}
    end
    |> process_circuits()
  end

  defp process_circuits([], wire_map) do
    wire_map
  end

  defp parse_value(wire_map, string) do
    case Integer.parse(string) do
      {num, ""} -> num
      _ -> Map.get(wire_map, string)
    end
  end

  defp handle_match(%{"left" => "", "op" => "NOT", "out" => out, "right" => right}, wire_map) do
    parse_value(wire_map, right)
    |> case do
      nil ->
        {:no, wire_map}

      num ->
        {:ok, Map.put_new(wire_map, out, bnot(num) |> band(65535))}
    end
  end

  defp handle_match(%{"left" => "", "op" => "", "out" => out, "right" => right}, wire_map) do
    parse_value(wire_map, right)
    |> case do
      nil -> {:no, wire_map}
      num -> {:ok, Map.put_new(wire_map, out, num)}
    end
  end

  defp handle_match(%{"left" => left, "op" => op, "out" => out, "right" => right}, wire_map) do
    case op do
      "AND" -> &band/2
      "OR" -> &bor/2
      "RSHIFT" -> &bsr/2
      "LSHIFT" -> &bsl/2
    end
    |> handle_dual_wires(left, right, out, wire_map)
  end

  defp handle_dual_wires(op, left, right, out, wire_map) do
    case {parse_value(wire_map, left), parse_value(wire_map, right)} do
      {l, r} when not is_nil(l) and not is_nil(r) ->
        {:ok, Map.put_new(wire_map, out, op.(l, r))}

      _ ->
        {:no, wire_map}
    end
  end
end

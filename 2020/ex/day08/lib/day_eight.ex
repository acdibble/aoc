defmodule DayEight do
  defp load_codes(path) do
    __ENV__.file
    |> Path.dirname()
    |> Path.join(path)
    |> File.stream!()
    |> Stream.map(&String.trim/1)
    |> Stream.map(&String.split(&1, " ", trim: true))
    |> Stream.with_index()
    |> Enum.into(%{}, fn {[cmd, num], index} ->
      {index, {String.to_atom(cmd), String.to_integer(num)}}
    end)
  end

  def part_one do
    load_codes("../data.txt")
    |> run(:acc)
  end

  def part_two do
    codes = load_codes("../data.txt")

    codes
    |> Map.keys()
    |> Enum.reduce_while(nil, fn current, _acc ->
      case codes[current] do
        {:acc, _value} -> nil
        {op, value} -> run(%{codes | current => {swap(op), value}}, :fix)
      end
      |> case do
        nil -> {:cont, nil}
        result -> {:halt, result}
      end
    end)
  end

  defp run(codes, flag, visited_addresses \\ [], index \\ 0, acc \\ 0)

  defp run(codes, flag, visited_addresses, index, acc) do
    if index in visited_addresses do
      handle(flag, acc)
    else
      case process_command(codes[index], index, acc) do
        {new_index, new_acc} ->
          run(codes, flag, [index | visited_addresses], new_index, new_acc)

        result ->
          result
      end
    end
  end

  defp handle(:acc, num), do: num
  defp handle(:fix, _num), do: nil

  defp process_command({:jmp, value}, index, acc), do: {index + value, acc}
  defp process_command({:nop, _value}, index, acc), do: {index + 1, acc}
  defp process_command({:acc, value}, index, acc), do: {index + 1, acc + value}
  defp process_command(nil, _index, acc), do: acc

  defp swap(:jmp), do: :nop
  defp swap(:nop), do: :jmp
end

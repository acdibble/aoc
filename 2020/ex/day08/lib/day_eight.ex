defmodule DayEight do
  defp load_code(path) do
    __ENV__.file
    |> Path.dirname()
    |> Path.join(path)
    |> File.stream!()
    |> Stream.map(&(String.trim(&1) |> String.split(" ", trim: true)))
    |> Stream.with_index()
    |> Enum.into(%{}, fn {[cmd, num], index} ->
      {index, {String.to_atom(cmd), String.to_integer(num)}}
    end)
  end

  def part_one do
    load_code("../data.txt")
    |> run(:acc)
  end

  def part_two do
    code = load_code("../data.txt")
    Enum.find_value(code, &replace(&1, code))
  end

  defp run(code, flag, addresses \\ [], index_acc_tuple \\ {0, 0}, stop \\ false)

  defp run(_code, :fix, _addresses, {nil, acc}, _stop), do: acc
  defp run(_code, :acc, _addresses, {_index, acc}, true), do: acc
  defp run(_code, :fix, _addresses, {_index, _acc}, true), do: nil

  defp run(code, flag, addresses, {index, acc}, false),
    do: run(code, flag, [index | addresses], process(code[index], index, acc), index in addresses)

  defp process({:jmp, value}, index, acc), do: {index + value, acc}
  defp process({:nop, _value}, index, acc), do: {index + 1, acc}
  defp process({:acc, value}, index, acc), do: {index + 1, acc + value}
  defp process(nil, _index, acc), do: {nil, acc}

  defp replace({_index, {:acc, _value}}, _code), do: nil
  defp replace({index, {op, value}}, code), do: run(%{code | index => {swap(op), value}}, :fix)

  defp swap(:jmp), do: :nop
  defp swap(:nop), do: :jmp
end

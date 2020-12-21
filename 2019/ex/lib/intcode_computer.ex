defmodule IntcodeComputer do
  def start(a, replacements \\ %{})

  def start(path, replacements) when is_binary(path) do
    read_file(path)
    |> start(replacements)
  end

  def start(intcodes, replacements) when is_map(intcodes) do
    intcodes
    |> Map.merge(replacements)
    |> run()
  end

  def read_file(path) do
    File.read!(path)
    |> String.split(",")
    |> Stream.map(&String.to_integer/1)
    |> Stream.with_index()
    |> Stream.map(fn {code, index} -> {index, code} end)
    |> Map.new()
  end

  defp run({intcodes, pointer}) when is_map(intcodes) do
    Map.fetch!(intcodes, pointer)
    |> process(intcodes, pointer)
    |> run()
  end

  defp run(result) when is_tuple(result), do: result
  defp run(intcodes) when is_map(intcodes), do: run({intcodes, 0})

  defp process(1, intcodes, pointer), do: do_op(intcodes, pointer, &Kernel.+/2, 4)
  defp process(2, intcodes, pointer), do: do_op(intcodes, pointer, &Kernel.*/2, 4)
  defp process(99, intcodes, _pointer), do: {:ok, Map.fetch!(intcodes, 0)}
  defp process(_opcode, _intcodes, _pointer), do: {:error, :bad_opcode}

  defp do_op(intcodes, pointer, op, increment) do
    [operand1, operand2, output] = 1..3 |> Enum.map(&Map.fetch!(intcodes, pointer + &1))

    {Map.put(
       intcodes,
       output,
       op.(Map.fetch!(intcodes, operand1), Map.fetch!(intcodes, operand2))
     ), pointer + increment}
  end
end

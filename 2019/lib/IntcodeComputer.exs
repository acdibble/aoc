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
    |> Stream.map(fn { code, index } -> { index, code } end)
    |> Map.new()
  end

  defp run(intcodes, pointer \\ 0) do
    case Map.fetch!(intcodes, pointer) do
      1 ->
        add(intcodes, pointer)
      2 ->
        mult(intcodes, pointer)
      99 ->
        { :ok, Map.fetch!(intcodes, 0) }
      _ ->
        { :error, :bad_opcode }
    end
  end

  defp add(intcodes, pointer) do
    do_op(intcodes, pointer, &Kernel.+/2, 4)
  end

  defp mult(intcodes, pointer) do
    do_op(intcodes, pointer, &Kernel.*/2, 4)
  end

  defp do_op(intcodes, pointer, op, 4) do
    operand1 = pointer + 1
    operand2 = pointer + 2
    operand3 = pointer + 3
    %{ ^operand1 => param1, ^operand2 => param2, ^operand3 => output } = intcodes
    Map.put(intcodes, output, op.(Map.fetch!(intcodes, param1), Map.fetch!(intcodes, param2)))
    |> run(pointer + 4)
  end
end

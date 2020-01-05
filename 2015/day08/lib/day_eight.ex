defmodule DayEight do
  import FileLoader

  def calculate_characters(path) when is_binary(path), do: calculate_characters(path: path)

  def calculate_characters(opts \\ []) do
    mode = Keyword.get(opts, :mode, :decode)

    read_file(opts)
    |> String.split("\n")
    |> Enum.reduce([0, []], fn line, [num, lines] ->
      [num + String.length(line), [line | lines]]
    end)
    |> calculate_total(mode)
  end

  def calculate_total(acc, mode) do
    Enum.reduce(acc, fn a, b ->
      b -
        Enum.reduce(a, 0, fn line, acc ->
          acc + get_length(line, mode)
        end)
    end)
    |> finalize(mode)
  end

  def get_length(string, :decode), do: Code.eval_string(string) |> elem(0) |> String.length()

  def get_length(string, :encode) do
    ~s("#{
      String.replace(string, "\\", "\\\\", global: true)
      |> String.replace("\"", "\\\"", global: true)
    }")
    |> String.length()
  end

  def finalize(result, mode) do
    if mode == :decode, do: result, else: abs(result)
  end
end

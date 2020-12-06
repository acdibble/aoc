defmodule Rule do
  defstruct min: 0, max: 0, letter: '', password: ""

  def validate(%Rule{password: password, min: min, max: max, letter: letter} = rule, type) do
    occurrences = count_occurrences(rule, password, 0)

    case type do
      :bounds ->
        occurrences >= min and occurrences <= max

      :indices ->
        String.at(password, min - 1) == <<letter>> != (String.at(password, max - 1) == <<letter>>)
    end
    |> bool_to_num()
  end

  defp count_occurrences(_rule, "", count), do: count

  defp count_occurrences(%Rule{letter: letter} = rule, <<char, rest::binary>>, count) do
    new_count =
      (char == letter)
      |> bool_to_num()
      |> Kernel.+(count)

    count_occurrences(rule, rest, new_count)
  end

  def from_string(string), do: String.trim(string) |> from_string(%Rule{}, "")

  defp from_string("", rule, string), do: %{rule | password: string}

  defp from_string(<<"-", rest::binary>>, rule, string) do
    from_string(rest, %{rule | min: String.to_integer(string)}, "")
  end

  defp from_string(<<":", rest::binary>>, rule, <<char, "">>) do
    from_string(rest, %{rule | letter: char}, "")
  end

  defp from_string(<<" ", rest::binary>>, %Rule{letter: letter} = rule, string) do
    update =
      case letter do
        '' -> %{rule | max: String.to_integer(string)}
        _ -> rule
      end

    from_string(rest, update, "")
  end

  defp from_string(<<char, rest::binary>>, rule, string) do
    from_string(rest, rule, string <> <<char>>)
  end

  defp bool_to_num(true), do: 1
  defp bool_to_num(false), do: 0
end

defmodule DayTwo do
  defp load_file(path) do
    __ENV__.file
    |> Path.dirname()
    |> Path.join(path)
    |> File.stream!()
  end

  def part_one do
    load_file("../data.txt")
    |> Stream.map(&Rule.from_string/1)
    |> Stream.map(&Rule.validate(&1, :bounds))
    |> Enum.sum()
  end

  def part_two do
    load_file("../data.txt")
    |> Stream.map(&Rule.from_string/1)
    |> Stream.map(&Rule.validate(&1, :indices))
    |> Enum.sum()
  end
end

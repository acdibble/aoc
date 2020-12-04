defmodule Rule do
  defstruct min: 0, max: 0, letter: '', password: ""

  def validate(%Rule{password: password, min: min, max: max, letter: letter} = rule, type) do
    occurrences = count_occurrences(rule, password, 0)

    case type do
      :bounds ->
        occurrences >= min and occurrences <= max

      _ ->
        String.at(password, min - 1) == <<letter>> != (String.at(password, max - 1) == <<letter>>)
    end
    |> case do
      true -> 1
      _ -> 0
    end
  end

  defp count_occurrences(rule, "", count) do
    IO.inspect([rule, count])
    count
  end

  defp count_occurrences(%Rule{letter: letter} = rule, <<char, rest::binary>>, count) do
    new_count =
      case char == letter do
        true -> 1
        _ -> 0
      end
      |> Kernel.+(count)

    count_occurrences(rule, rest, new_count)
  end

  def parse_string(string), do: String.trim(string) |> parse_rule(%Rule{}, "")

  defp parse_rule("", rule, string), do: %{rule | password: string}

  defp parse_rule(<<"-", rest::binary>>, rule, string) do
    parse_rule(rest, %{rule | min: String.to_integer(string)}, "")
  end

  defp parse_rule(<<":", rest::binary>>, rule, <<char, "">>) do
    parse_rule(rest, %{rule | letter: char}, "")
  end

  defp parse_rule(<<" ", rest::binary>>, %Rule{letter: letter} = rule, string) do
    update =
      case letter do
        '' -> %{rule | max: String.to_integer(string)}
        _ -> rule
      end

    parse_rule(rest, update, "")
  end

  defp parse_rule(<<char, rest::binary>>, rule, string) do
    parse_rule(rest, rule, string <> <<char>>)
  end
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
    |> Stream.map(&Rule.parse_string/1)
    |> Stream.map(&Rule.validate(&1, :bounds))
    |> Enum.reduce(0, &Kernel.+/2)
  end

  def part_two do
    load_file("../data.txt")
    |> Stream.map(&Rule.parse_string/1)
    |> Stream.map(&Rule.validate(&1, :indices))
    |> Enum.reduce(0, &Kernel.+/2)
  end
end

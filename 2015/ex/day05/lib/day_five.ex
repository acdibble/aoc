defmodule DayFive do
  import FileLoader

  def count_nice_words(opts \\ []) do
    path = Keyword.get(opts, :path)
    ruleset = Keyword.get(opts, :ruleset, :old)
    Keyword.get(opts, :data, read_file(path))
    |> case do
      words when is_list(words) ->
        words
      words when is_binary(words) ->
        String.split(words, "\n")
    end
    |> Enum.reduce(0, fn word, acc ->
      case is_naughty_or_nice(word, ruleset) do
        :nice ->
          acc + 1
        :naughty ->
          acc
      end
    end)
  end

  defp is_naughty_or_nice(word, :old) do
    has_at_least_three_vowels?(word)
    |> contains_duplicate?(word)
    |> contains_naughty_substring?(word)
  end

  defp is_naughty_or_nice(word, :new) do
    if Regex.match?(~r/(.)[^\1]\1/, word) and Regex.match?(~r/(..).*\1/, word) do
      IO.inspect([:nice, word])
      :nice
    else
      IO.inspect([:naughty, word])
      :naughty
    end
  end

  defp has_at_least_three_vowels?(word) do
    String.graphemes(word)
    |> Enum.reduce(0, fn char, acc ->
      acc + if Regex.match?(~r/[aeiou]/, char), do: 1, else: 0
    end)
    |> case do
      x when x >=3 ->
        :nice
      _ ->
        :naughty
    end
  end

  defp contains_duplicate?(:naughty, _word), do: :naughty
  defp contains_duplicate?(:nice, word) do
    if Regex.match?(~r/(.)\1/, word), do: :nice, else: :naughty
  end

  defp contains_naughty_substring?(:naughty, _word), do: :naughty
  defp contains_naughty_substring?(:nice, word) do
    if Enum.any?(["ab", "cd", "pq", "xy"], fn substring -> String.contains?(word, substring) end) do
      :naughty
    else
      :nice
    end
  end
end

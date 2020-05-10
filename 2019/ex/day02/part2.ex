defmodule DayTwo.PartTwo do
  def run do
    intcodes = IntcodeComputer.read_file("day02/data.txt")

    run(intcodes, 0, 0)
  end

  defp run(intcodes, noun, 100), do: run(intcodes, noun + 1, 0)
  defp run(intcodes, 100, verb), do: run(intcodes, 0, verb + 1)

  defp run(intcodes, noun, verb) do
    case IntcodeComputer.start(intcodes, %{1 => noun, 2 => verb}) do
      {:ok, 19_690_720} ->
        noun * 100 + verb

      _ ->
        run(intcodes, noun + 1, verb)
    end
  end
end

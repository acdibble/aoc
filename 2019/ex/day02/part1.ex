defmodule DayTwo.PartOne do
  use FileReader

  def run do
    IntcodeComputer.start(get_input_filename(), %{1 => 12, 2 => 2})
  end
end

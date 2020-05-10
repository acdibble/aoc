defmodule DayTwo.PartOne do
  def run do
    IntcodeComputer.start("day02/data.txt", %{1 => 12, 2 => 2})
  end
end

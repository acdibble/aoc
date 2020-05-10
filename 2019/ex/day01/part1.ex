defmodule DayOne.PartOne do
  def run do
    File.read!("day01/data.txt")
    |> String.split()
    |> Enum.reduce(0, fn mass, acc ->
      mass
      |> String.to_integer()
      |> Integer.floor_div(3)
      |> Kernel.-(2)
      |> Kernel.+(acc)
    end)
  end
end

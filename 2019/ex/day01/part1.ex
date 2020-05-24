defmodule DayOne.PartOne do
  use FileReader

  def run do
    read_file!()
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

defmodule DayOne.PartTwo do
  use FileReader

  def run do
    stream_file!()
    |> Stream.map(&String.trim/1)
    |> Stream.map(&String.to_integer/1)
    |> Enum.reduce(0, &get_requirement/2)
  end

  def get_requirement(mass, total) do
    mass
    |> Integer.floor_div(3)
    |> Kernel.-(2)
    |> case do
      result when result > 0 ->
        get_requirement(result, total + result)

      _ ->
        total
    end
  end
end

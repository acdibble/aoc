defmodule Fuel do
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

File.read!("day01/data.txt")
|> String.split()
|> Enum.map(&String.to_integer/1)
|> Enum.reduce(0, &Fuel.get_requirement/2)
|> IO.inspect()

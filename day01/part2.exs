defmodule Fuel do
  def get_requirement(mass) when mass <= 0, do: 0
  def get_requirement(mass) do
    mass
    |> Integer.floor_div(3)
    |> Kernel.-(2)
    |> (fn (a) -> a + get_requirement(a) end).()
    |> case do
      result when result > 0 ->
        result
      _ ->
        0
    end
  end
end

File.read!("day01/data.txt")
|> String.split()
|> Enum.map(&String.to_integer/1)
|> Enum.reduce(0, fn module, acc -> acc + Fuel.get_requirement(module) end)
|> IO.inspect()

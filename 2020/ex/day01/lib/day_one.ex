defmodule DayOne do
  def get_numbers(path) do
    Path.dirname(__ENV__.file)
    |> Path.join(path)
    |> File.stream!()
    |> Stream.map(&parse_line/1)
  end

  def part_one do
    numbers = get_numbers("../data.txt")

    for i <- numbers, j <- numbers, i + j == 2020, do: throw(i * j)
  catch
    x -> x
  end

  def part_two do
    numbers = get_numbers("../data.txt")

    for i <- numbers, j <- numbers, k <- numbers, i + j + k == 2020, do: throw(i * j * k)
  catch
    x -> x
  end

  defp parse_line(line), do: String.trim(line) |> String.to_integer()
end

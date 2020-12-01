defmodule DayOne do
  def get_numbers(path) do
    Path.dirname(__ENV__.file)
    |> Path.join(path)
    |> File.stream!()
    |> Stream.map(&parse_line/1)
  end

  def part_one do
    numbers = get_numbers("../data.txt")

    for i <- numbers, j <- numbers do
      case i + j do
        2020 -> throw(i * j)
        _ -> nil
      end
    end
  catch
    x -> x
  end

  def part_two do
    numbers = get_numbers("../data.txt")

    for i <- numbers, j <- numbers, k <- numbers do
      case i + j + k do
        2020 -> throw(i * j * k)
        _ -> nil
      end
    end
  catch
    x -> x
  end

  defp parse_line(line), do: String.trim(line) |> String.to_integer()
end

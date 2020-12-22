defmodule DayEleven do
  use Agent

  defp load_file(path \\ "../data.txt") do
    __ENV__.file
    |> Path.dirname()
    |> Path.join(path)
    |> File.stream!()
    |> Stream.map(&String.trim/1)
    |> Stream.map(&String.split(&1, "", trim: true))
    |> Stream.with_index()
    |> Enum.into(%{}, &{elem(&1, 1), elem(&1, 0)})
  end

  def start_agent do
    Agent.start_link(fn -> nil end, name: __MODULE__)
  end

  def get_cutoff do
    Agent.get(__MODULE__, & &1)
  end

  def set_cutoff(value) do
    Agent.update(__MODULE__, fn _state -> value end)
  end

  def part_one do
    start_agent()
    set_cutoff(4)

    load_file()
    |> iterate()
  end

  def part_two do
    start_agent()
    set_cutoff(5)

    load_file()
    |> iterate()
    |> elem(1)
    |> Kernel.-(1)
  end

  defp iterate(board, iterations \\ 0) do
    transform_rows(board)
    |> diff_boards(board)
    |> case do
      {:stop, new_board} -> {iterations, count_occupied_seats(new_board)}
      {:cont, new_board} -> iterate(new_board, iterations + 1)
    end
  end

  defp count_occupied_seats(board, row \\ 0, total \\ 0)
  defp count_occupied_seats(board, row, total) when row == map_size(board), do: total

  defp count_occupied_seats(board, row, total) do
    Map.get(board, row)
    |> Enum.count(&(&1 == "#"))
    |> (&count_occupied_seats(board, row + 1, total + &1)).()
  end

  defp transform_rows(current_board, new_board \\ %{}, y \\ 0) do
    case Map.get(current_board, y) do
      nil ->
        new_board

      row ->
        new_row =
          Enum.with_index(row)
          |> Enum.map(fn {char, x} ->
            get_new_value(current_board, char, x, y)
          end)

        transform_rows(current_board, Map.put(new_board, y, new_row), y + 1)
    end
  end

  defp get_new_value(_board, ".", _x, _y), do: "."

  defp get_new_value(board, char, x, y),
    do: count_neighbors(0, board, x, y, -1, -1) |> convert_char(char, get_cutoff())

  defp count_neighbors(total, _board, _x, _y, 1, 2), do: total
  defp count_neighbors(total, board, x, y, 0, 0), do: count_neighbors(total, board, x, y, 0, 1)

  defp count_neighbors(total, board, x, y, xOffset, 2),
    do: count_neighbors(total, board, x, y, xOffset + 1, -1)

  defp count_neighbors(total, board, x, y, xOffset, yOffset) do
    get_neighbor_value(board, x, y, xOffset, yOffset)
    |> convert_neighbor()
    |> Kernel.+(total)
    |> count_neighbors(board, x, y, xOffset, yOffset + 1)
  end

  defp get_neighbor_value(_board, x, _y, xOffset, _yOffset) when x + xOffset < 0, do: nil
  defp get_neighbor_value(_board, _x, y, _xOffset, yOffset) when y + yOffset < 0, do: nil

  defp get_neighbor_value(board, x, _y, xOffset, _yOffset) when x + xOffset > map_size(board),
    do: nil

  defp get_neighbor_value(board, _x, y, _xOffset, yOffset) when y + yOffset > map_size(board),
    do: nil

  defp get_neighbor_value(board, x, y, xOffset, yOffset) do
    case get_cutoff() do
      4 ->
        Map.get(board, y + yOffset, [])
        |> Enum.at(x + xOffset)

      5 ->
        Enum.reduce_while(1..map_size(board), nil, fn mul, _acc ->
          case Map.get(board, y + mul * yOffset, []) |> Enum.at(x + mul * xOffset) do
            "." -> {:cont, nil}
            val -> {:halt, val}
          end
        end)
    end
  end

  defp convert_neighbor(nil), do: 0
  defp convert_neighbor("."), do: 0
  defp convert_neighbor("L"), do: 0
  defp convert_neighbor("#"), do: 1

  defp convert_char(total, "#", cutoff) when total >= cutoff, do: "L"
  defp convert_char(total, "L", _cutoff) when total == 0, do: "#"
  defp convert_char(_total, char, _cutoff), do: char

  defp diff_boards(board1, board2, row \\ 0)
  defp diff_boards(board1, _board2, row) when row == map_size(board1), do: {:stop, board1}

  defp diff_boards(board1, board2, row) when row < map_size(board1) do
    Enum.zip(Map.get(board1, row, []), Map.get(board2, row, []))
    |> Enum.find_value(false, fn {a, b} -> a != b end)
    |> if do
      {:cont, board1}
    else
      diff_boards(board1, board2, row + 1)
    end
  end
end

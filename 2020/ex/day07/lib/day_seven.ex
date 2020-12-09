defmodule DaySeven do
  defp load_file(path) do
    __ENV__.file
    |> Path.dirname()
    |> Path.join(path)
    |> File.stream!()
    |> Stream.map(&String.trim/1)
  end

  defp parse_bags do
    load_file("../data.txt")
    |> Stream.map(&parse_bag/1)
  end

  def part_one do
    parse_bags()
    |> Enum.reduce(%{}, fn item, acc ->
      item.children
      |> Enum.into(%{}, &{&1.name, [item.name]})
      |> Map.merge(acc, fn _k, v1, v2 ->
        v1 ++ v2
      end)
    end)
    |> find_all_bags_that_contain("shiny gold bag")
  end

  def part_two do
    parse_bags()
    |> Enum.into(%{}, &{&1.name, &1.children})
    |> count_bags_contained_by("shiny gold bag")
  end

  defp count_bags_contained_by(bag_map, bag),
    do: count_bags_contained_by(bag_map, Map.get(bag_map, bag), 0)

  defp count_bags_contained_by(_bag_map, [], total), do: total

  defp count_bags_contained_by(bag_map, bags, total) do
    new_bags =
      Enum.flat_map(bags, fn bag ->
        Map.get(bag_map, bag.name, [])
        |> Enum.map(&%{&1 | amount: &1.amount * bag.amount})
      end)

    count_bags_contained_by(
      bag_map,
      new_bags,
      total + Enum.reduce(bags, 0, &(&1.amount + &2))
    )
  end

  defp find_all_bags_that_contain(bag_map, bag) do
    [next | rest] = Map.fetch!(bag_map, bag)
    find_all_bags_that_contain(bag_map, next, [], rest)
  end

  defp find_all_bags_that_contain(_bag_map, _bag, containers, []),
    do: Enum.uniq(containers) |> length()

  defp find_all_bags_that_contain(bag_map, bag, containers, [next | rest]) do
    find_all_bags_that_contain(
      bag_map,
      next,
      [bag | containers],
      rest ++ Map.get(bag_map, bag, [])
    )
  end

  defp parse_bag(line, bag \\ %{name: "", children: []}, acc \\ "")

  defp parse_bag(".", bag, _acc), do: bag

  defp parse_bag(" bags contain " <> rest, bag, acc),
    do: parse_bag(rest, %{bag | name: acc <> " bag"}, "")

  defp parse_bag(", " <> rest, bag, _acc), do: parse_bag(rest, bag, "")

  defp parse_bag("bag" <> rest, bag, acc),
    do:
      parse_bag(
        rest,
        %{bag | children: parse_child(acc <> "bag", bag.children)},
        ""
      )

  defp parse_bag(<<char::bytes-size(1)>> <> rest, bag, acc),
    do: parse_bag(rest, bag, acc <> char)

  defp parse_child(data, children) do
    case String.split(data, " ", parts: 2, trim: true) do
      ["no", _] -> children
      [amount, name] -> [%{amount: String.to_integer(amount), name: name} | children]
    end
  end
end

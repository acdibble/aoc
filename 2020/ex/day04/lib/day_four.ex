defmodule DayFour do
  defp load_file(path) do
    __ENV__.file
    |> Path.dirname()
    |> Path.join(path)
    |> File.read!()
  end

  def part_one do
    load_file("../data.txt")
    |> String.split(~r/[ \n]/)
    |> count_valid_passports(:presence)
  end

  def part_two do
    load_file("../data.txt")
    |> String.split(~r/[ \n]/)
    |> count_valid_passports(:validity)
  end

  defp count_valid_passports(lines, type, valid_fields \\ 0, count \\ 0)

  defp count_valid_passports([], _type, valid_fields, count),
    do: count + convert_valid_fields(valid_fields)

  defp count_valid_passports(["" | lines], type, valid_fields, count),
    do: count_valid_passports(lines, type, 0, count + convert_valid_fields(valid_fields))

  defp count_valid_passports(["byr:" <> rest | lines], type, valid_fields, count),
    do: count_valid_passports(lines, type, valid_fields + validate(:byr, rest, type), count)

  defp count_valid_passports(["iyr:" <> rest | lines], type, valid_fields, count),
    do: count_valid_passports(lines, type, valid_fields + validate(:iyr, rest, type), count)

  defp count_valid_passports(["eyr:" <> rest | lines], type, valid_fields, count),
    do: count_valid_passports(lines, type, valid_fields + validate(:eyr, rest, type), count)

  defp count_valid_passports(["hgt:" <> rest | lines], type, valid_fields, count),
    do: count_valid_passports(lines, type, valid_fields + validate(:hgt, rest, type), count)

  defp count_valid_passports(["hcl:" <> rest | lines], type, valid_fields, count),
    do: count_valid_passports(lines, type, valid_fields + validate(:hcl, rest, type), count)

  defp count_valid_passports(["ecl:" <> rest | lines], type, valid_fields, count),
    do: count_valid_passports(lines, type, valid_fields + validate(:ecl, rest, type), count)

  defp count_valid_passports(["pid:" <> rest | lines], type, valid_fields, count),
    do: count_valid_passports(lines, type, valid_fields + validate(:pid, rest, type), count)

  defp count_valid_passports([_line | lines], type, valid_fields, count),
    do: count_valid_passports(lines, type, valid_fields, count)

  defp convert_valid_fields(7), do: 1
  defp convert_valid_fields(_), do: 0

  defp validate(_field, _data, :presence), do: 1

  defp validate(field, data, :validity), do: validate(field, data) |> bool_to_num()

  # byr (Birth Year) - four digits; at least 1920 and at most 2002.
  defp validate(:byr, data), do: Enum.member?(1920..2002, String.to_integer(data))

  # iyr (Issue Year) - four digits; at least 2010 and at most 2020.
  defp validate(:iyr, data), do: Enum.member?(2010..2020, String.to_integer(data))

  # eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
  defp validate(:eyr, data), do: Enum.member?(2020..2030, String.to_integer(data))

  # hgt (Height) - a number followed by either cm or in:
  # If cm, the number must be at least 150 and at most 193.
  # If in, the number must be at least 59 and at most 76.
  defp validate(:hgt, <<size::bytes-size(3)>> <> "cm"),
    do: Enum.member?(150..193, String.to_integer(size))

  defp validate(:hgt, <<size::bytes-size(2)>> <> "in"),
    do: Enum.member?(59..76, String.to_integer(size))

  defp validate(:hgt, _data), do: false

  # hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
  defp validate(:hcl, "#" <> data), do: Enum.member?(0x0..0xFFFFFF, String.to_integer(data, 16))
  defp validate(:hcl, _data), do: false

  # ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
  defp validate(:ecl, "amb"), do: true
  defp validate(:ecl, "blu"), do: true
  defp validate(:ecl, "brn"), do: true
  defp validate(:ecl, "gry"), do: true
  defp validate(:ecl, "grn"), do: true
  defp validate(:ecl, "hzl"), do: true
  defp validate(:ecl, "oth"), do: true
  defp validate(:ecl, _data), do: false

  # pid (Passport ID) - a nine-digit number, including leading zeroes.
  defp validate(:pid, <<data::bytes-size(9)>> <> ""), do: String.match?(data, ~r/[0-9]{9}/)
  defp validate(:pid, _data), do: false

  defp bool_to_num(true), do: 1
  defp bool_to_num(false), do: 0
end

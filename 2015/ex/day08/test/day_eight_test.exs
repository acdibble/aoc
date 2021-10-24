defmodule DayEightTest do
  use ExUnit.Case
  doctest DayEight

  @data ~s(""\n"abc"\n"aaa\\"aaa"\n"\\x27")

  test "calculates in decoding mode" do
    assert DayEight.calculate_characters(data: @data, mode: :decode) == 12
  end

  test "calculates in encoding mode" do
    assert DayEight.calculate_characters(data: @data, mode: :encode) == 19
  end
end

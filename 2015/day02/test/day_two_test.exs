defmodule DayTwoTest do
  use ExUnit.Case
  doctest DayTwo

  test "calculates total needed area (1)" do
    assert DayTwo.calculate_needs(:paper, data: "2x3x4") == 58
  end

  test "calculates total needed area (2)" do
    assert DayTwo.calculate_needs(:paper, data: "1x1x10") == 43
  end

  test "calculates total needed area (3)" do
    assert DayTwo.calculate_needs(:ribbon, data: "2x3x4") == 34
  end

  test "calculates total needed area (4)" do
    assert DayTwo.calculate_needs(:ribbon, data: "1x1x10") == 14
  end
end

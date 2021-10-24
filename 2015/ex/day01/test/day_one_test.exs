defmodule DayOneTest do
  use ExUnit.Case
  doctest DayOne

  test "counts floors (1)" do
    assert DayOne.count_floors(data: "(())") == 0
  end

  test "counts floors (2)" do
    assert DayOne.count_floors(data: "()()") == 0
  end

  test "counts floors (3)" do
    assert DayOne.count_floors(data: "(((") == 3
  end

  test "counts floors (4)" do
    assert DayOne.count_floors(data: "(()(()(") == 3
  end

  test "counts floors (5)" do
    assert DayOne.count_floors(data: "))(((((") == 3
  end

  test "counts floors (6)" do
    assert DayOne.count_floors(data: "())") == -1
  end

  test "counts floors (7)" do
    assert DayOne.count_floors(data: "))(") == -1
  end

  test "counts floors (8)" do
    assert DayOne.count_floors(data: ")))") == -3
  end

  test "counts floors (9)" do
    assert DayOne.count_floors(data: ")())())") == -3
  end

  test "counts floors (10)" do
    assert DayOne.count_floors(data: ")", find_first_basement: true) == 1
  end

  test "counts floors (11)" do
    assert DayOne.count_floors(data: "()())", find_first_basement: true) == 5
  end
end

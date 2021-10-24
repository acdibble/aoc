defmodule DayThreeTest do
  use ExUnit.Case
  doctest DayThree

  test "counts the houses (1)" do
    assert DayThree.start_count(data: ">") == 2
  end

  test "counts the houses (2)" do
    assert DayThree.start_count(data: "^>v<") == 4
  end

  test "counts the houses (3)" do
    assert DayThree.start_count(data: "^v^v^v^v^v") == 2
  end

  test "counts the houses (4)" do
    assert DayThree.start_count(data: "^v", santa_count: 2) == 3
  end

  test "counts the houses (5)" do
    assert DayThree.start_count(data: "^>v<", santa_count: 2) == 3
  end

  test "counts the houses (6)" do
    assert DayThree.start_count(data: "^v^v^v^v^v", santa_count: 2) == 11
  end
end

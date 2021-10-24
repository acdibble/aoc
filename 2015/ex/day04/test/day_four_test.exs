defmodule DayFourTest do
  use ExUnit.Case
  doctest DayFour

  test "finds the lowest nonce (1)" do
    assert DayFour.find_lowest_nonce("abcdef", 5) == 609043
  end

  test "finds the lowest nonce (2)" do
    assert DayFour.find_lowest_nonce("pqrstuv", 5) == 1048970
  end
end

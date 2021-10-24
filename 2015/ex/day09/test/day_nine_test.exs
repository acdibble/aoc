defmodule DayNineTest do
  use ExUnit.Case
  doctest DayNine

  @data "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141"

  test "finds the shortest route" do
    assert DayNine.get_route(data: @data, route_type: :shortest) == 605
  end

  test "finds the longest route" do
    assert DayNine.get_route(data: @data, route_type: :longest) == 982
  end
end

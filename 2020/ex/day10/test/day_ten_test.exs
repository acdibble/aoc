defmodule DayTenTest do
  use ExUnit.Case
  doctest DayTen

  test "greets the world" do
    assert DayTen.hello() == :world
  end
end

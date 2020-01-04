defmodule DaySevenTest do
  use ExUnit.Case
  doctest DaySeven

  test "greets the world" do
    assert DaySeven.hello() == :world
  end
end

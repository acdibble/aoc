defmodule DaySixTest do
  use ExUnit.Case
  doctest DaySix

  @turn_on "turn on 0,0 through 2,2"
  @toggle "toggle 0,0 through 2,2"
  @turn_off "turn off 0,0 through 2,2"

  describe "part 1" do
    test "calculates the number of lights on (1)" do
      assert DaySix.process_instructions(data: @turn_on, method: :incorrect) == 9
    end

    test "calculates the number of lights on (2)" do
      assert DaySix.process_instructions(data: @toggle, method: :incorrect) == 9
    end

    test "calculates the number of lights on (3)" do
      assert DaySix.process_instructions(data: @turn_off, method: :incorrect) == 0
    end

    test "calculates the number of lights on (4)" do
      data = Enum.join([@turn_on, @turn_off], "\n")
      assert DaySix.process_instructions(data: data, method: :incorrect) == 0
    end

    test "calculates the number of lights on (5)" do
      data = Enum.join([@toggle, @turn_on], "\n")
      assert DaySix.process_instructions(data: data, method: :incorrect) == 9
    end
  end

  describe "part 2" do
    test "calculates the light intensity (1)" do
      assert DaySix.process_instructions(data: @turn_on, method: :correct) == 9
    end

    test "calculates the light intensity (2)" do
      assert DaySix.process_instructions(data: @toggle, method: :correct) == 18
    end

    test "calculates the light intensity (3)" do
      assert DaySix.process_instructions(data: @turn_off, method: :correct) == 0
    end

    test "calculates the light intensity (4)" do
      data = Enum.join([@turn_on, @turn_off], "\n")
      assert DaySix.process_instructions(data: data, method: :correct) == 0
    end

    test "calculates the light intensity (5)" do
      data = Enum.join([@toggle, @turn_on], "\n")
      assert DaySix.process_instructions(data: data, method: :correct) == 27
    end
  end
end

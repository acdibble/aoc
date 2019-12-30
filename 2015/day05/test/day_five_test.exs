defmodule DayFiveTest do
  use ExUnit.Case
  doctest DayFive

  test "counts nice words (old)" do
    words = ["ugknbfddgicrmopn", "aaa", "jchzalrnumimnmhp", "haegwjzuvuyypxyu", "dvszwmarrgswjxmb"]
    assert DayFive.count_nice_words(data: words) == 2
  end

  test "counts nice words (new)" do
    words = ["qjhvhtzxzqqjkmpb", "xxyxx", "uurcxstgmygtbstg", "ieodomkazucvgmuy", "yzsmlbnftftgwadz"]
    assert DayFive.count_nice_words(data: words, ruleset: :new) == 3
  end
end

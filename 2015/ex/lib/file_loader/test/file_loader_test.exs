defmodule FileLoaderTest do
  use ExUnit.Case
  doctest FileLoader

  test "reads a file into a string" do
    assert FileLoader.read_file("#{__DIR__}/test.txt") == "Hello world!"
  end
end

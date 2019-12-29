defmodule FileLoader do
  def read_file(path) do
    File.read!(path)
  end
end

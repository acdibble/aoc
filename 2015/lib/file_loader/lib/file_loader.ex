defmodule FileLoader do
  def read_file(nil) do
    ""
  end

  def read_file(path) do
    File.read!(path)
  end
end

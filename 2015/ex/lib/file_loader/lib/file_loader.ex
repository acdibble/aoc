defmodule FileLoader do
  def read_file(nil) do
    ""
  end

  def read_file(path) when is_binary(path) do
    File.read!(path)
  end

  def read_file(opts \\ []) when is_list(opts) do
    path = Keyword.get(opts, :path)
    method = Keyword.get(opts, :method)
    Keyword.get(opts, :data, read_file(path))
  end
end

defmodule FileReader do
  defmacro __using__(_) do
    quote do
      def stream_file!, do: get_input_filename() |> File.stream!()
      def read_file!, do: get_input_filename() |> File.read!()

      def get_input_filename, do: Path.join(__DIR__, "data.txt")
    end
  end
end

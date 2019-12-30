defmodule DayFour do
  def find_lowest_nonce(secret_key, leading_zeros, num \\ 0) do
    :crypto.hash(:md5, "#{secret_key}#{num}")
    |> Base.encode16()
    |> String.starts_with?(String.duplicate("0", leading_zeros))
    |> case do
      true ->
        num
      false ->
        find_lowest_nonce(secret_key, leading_zeros, num + 1)
    end
  end
end

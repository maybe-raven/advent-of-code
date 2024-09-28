defmodule Adventofelixir do
  @moduledoc """
  Documentation for `Adventofelixir`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> Adventofelixir.hello()
      :world

  """
  def match_digit(""), do: :done
  def match_digit(<<d, rest::binary>>) when d in ?0..?9, do: {:ok, d - ?0, rest}
  def match_digit("zero" <> rest), do: {:ok, 0, rest}
  def match_digit("one" <> rest), do: {:ok, 1, rest}
  def match_digit("two" <> rest), do: {:ok, 2, rest}
  def match_digit("three" <> rest), do: {:ok, 3, rest}
  def match_digit("four" <> rest), do: {:ok, 4, rest}
  def match_digit("five" <> rest), do: {:ok, 5, rest}
  def match_digit("six" <> rest), do: {:ok, 6, rest}
  def match_digit("seven" <> rest), do: {:ok, 7, rest}
  def match_digit("eight" <> rest), do: {:ok, 8, rest}
  def match_digit("nine" <> rest), do: {:ok, 9, rest}
  def match_digit(<<_, rest::binary>>), do: {:none, rest}

  def process_line(:done, first_digit, last_digit)
      when is_integer(first_digit) and is_integer(last_digit),
      do: first_digit * 10 + last_digit

  def process_line(:done, first_digit)
      when is_integer(first_digit),
      do: first_digit * 10 + first_digit

  def process_line(:done, first_digit)
      when is_integer(first_digit),
      do: first_digit * 10 + first_digit
end

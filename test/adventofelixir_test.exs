defmodule AdventofelixirTest do
  use ExUnit.Case
  doctest Adventofelixir

  test "greets the world" do
    assert Adventofelixir.hello() == :world
  end
end

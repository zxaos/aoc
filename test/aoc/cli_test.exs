defmodule CliTest do
  use ExUnit.Case
  doctest Aoc

  import Aoc.CLI, only: [ parse_args: 1 ]
  test "-h or help returns :help" do
    assert parse_args(["-h"])          == :help
    assert parse_args(["--help"])      == :help
    assert parse_args(["--help", "7"]) == :help
    assert parse_args(["7", "--help"]) == :help
  end

  test "a number returns that number" do
    #assert parse_args(["0"]) == 0
    assert parse_args(["1"]) == 1
  end
end

defmodule Aoc.CLI do
  @moduledoc """
  Perform command-line parsing and dispatch to the challenged for individual days
  """

  def main(argv) do
    argv
    |> parse_args
    |> run_day
  end

  @doc """
  argv is -h or --help,
  or else it is a day number
  """
  def parse_args(argv) do
    parse = OptionParser.parse(argv, switches: [ help: :boolean],
                                     aliases:  [ h:    :help   ])
    case parse do
      { [ help: true ], _, _ } -> :help
      { _, [day], _ }            -> String.to_integer day
    end
  end

  def run_day(:help) do
    IO.puts """
    usage: aoc <day> : run the puzzle solution for the given day, loading input from <day>.input
    """
    System.halt(0)
  end

  def run_day(1), do: Aoc.Day1.print_solution()
  def run_day(2), do: Aoc.Day2.print_solution()
  def run_day(3), do: Aoc.Day3.print_solution()
end

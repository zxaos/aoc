defmodule Aoc.Day4 do

  @doc """
  Given an integer, returns true if any of the adjacent digits are the same.
  """
  def has_adjacent_double_digits (number) do
    number
    |>Integer.digits
    |>Enum.chunk_by(&(&1)) # chunk adjacent duplicate digits
    |>Enum.any?(&(length(&1) == 2))
  end

  @doc """
  Given an integer, return true if the value of each digit never decreases, left to right.
  """
  def has_increasing_digits (number) do
    number
    |> Integer.digits
    |> Enum.reduce_while(0, fn x, acc -> if acc <= x, do: {:cont, x}, else: {:halt, false} end)
    |>(&(!!&1)).() # any integer coerces to true, unles we get :false back we're good
  end

  def filter_range(from, to) do
    from..to
    |>Enum.filter(&has_increasing_digits/1)
    |>Enum.filter(&has_adjacent_double_digits/1)
  end


  @doc """
  Given a range of positive six-digit numbers, determine the number of values within the range that meet the following criteria:
  * The number has two adjacent digits that are the same (e.g. 122345)
  * from left to right, the digits never decrease (e.g. 123324 is not valid)
  """
  def print_solution do
    [from, to] = with {:ok, contents} = File.read("4.input") do
      String.split(contents, ["-", "\n"], trim: true)
      |> Enum.map(&String.to_integer/1)
    end

    IO.puts "Valid codes in range #{from} to #{to}:"
    IO.puts "#{length filter_range(from, to)}"
  end
end

defmodule Aoc.Day3c.Coordinate do
  defstruct x: 0, y: 0
end

defmodule Aoc.Day3c.WireDirection do
  defstruct direction: 'R', distance: 0

  def new(str) do
    {direction, distance} = String.split_at(str, 1)
    distance = String.to_integer(distance)
    %Aoc.Day3.WireDirection{direction: direction, distance: distance}
  end
end

defmodule Aoc.Day3c do
  alias Aoc.Day3.Coordinate
  alias Aoc.Day3.WireDirection
  def find_nearest_intersection (wires) do
    [{first, first_map}, {second, second_map}] = Enum.map(wires, &coordinates_from_wire/1)
    intersections = first -- (first -- second)   # instersection of wire coordinates

    manhattan_min = Enum.min_by(intersections, &manhattan_distance/1)
    IO.puts "Minimum by manhattan distance:"
    IO.inspect manhattan_min
    IO.inspect manhattan_distance manhattan_min

    wirelength_min = Enum.min_by(intersections, &( first_map[&1] + second_map[&1]))

    IO.puts "Minimum by wire length:"
    IO.inspect wirelength_min
    IO.inspect first_map[wirelength_min] + second_map[wirelength_min]

  end

  def coordinates_from_wire(wire = [_direction | _tail], location \\ %Coordinate{}) do
    {_, coords} = Enum.reduce(wire, {location, []}, fn (elem, acc) ->
      {current_start, generated_wire_coordinates} = acc
      result = run_wire(current_start, elem.direction, elem.distance)
      {end_loc, new_coords} = result
      {end_loc, generated_wire_coordinates ++ new_coords}
    end)

    wire_distance_map = Enum.with_index(coords, 1)
      |> Enum.reverse # into replaces duplicate vaules, but we want to prefer lower counts
      |> Enum.into(%{})

    { coords, wire_distance_map }
  end

  # the start values are all offset by one here because we don't need to include the starting
  # coordinate in the map - either it's the origin, or it was the last generated coordinate from
  # the previous segment
  def run_wire(%{x: x, y: y}, "U", distance) do
    endloc = %Coordinate{x: x, y: y + distance}
    coords = Enum.map(y+1..(y + distance), &(%Coordinate{x: x, y: &1}))
    {endloc, coords}
  end
  def run_wire(%{x: x, y: y}, "D", distance) do
    endloc = %Coordinate{x: x, y: y - distance}
    coords = Enum.map(y-1..(y - distance), &(%Coordinate{x: x, y: &1}))
    {endloc, coords}
  end
  def run_wire(%{x: x, y: y}, "R", distance) do
    endloc = %Coordinate{x: x + distance, y: y}
    coords = Enum.map(x+1..(x + distance), &(%Coordinate{x: &1, y: y}))
    {endloc, coords}
  end
  def run_wire(%{x: x, y: y}, "L", distance) do
    endloc = %Coordinate{x: x - distance, y: y}
    coords = Enum.map(x-1..(x - distance), &(%Coordinate{x: &1, y: y}))
    {endloc, coords}
  end

  def manhattan_distance(first, second \\ %Coordinate{}) do
    abs(first.x - second.x) + abs(first.y - second.y)
  end

  def print_solution() do
    parsed_input = with {:ok, contents} = File.read("3.input") do
      String.split(contents, "\n", trim: true)
      |> Enum.map(&(String.split &1, ",", trim: true))
      |> Enum.map( fn (wire) -> Enum.map(wire, &WireDirection.new/1) end)
    end

    find_nearest_intersection(parsed_input)
  end
end

defmodule Coordinate do
  defstruct x: 0, y: 0
end

defmodule WireDirection do
  defstruct direction: 'R', distance: 0

  def new(str) do
    {direction, distance} = String.split_at(str, 1)
    distance = String.to_integer(distance)
    %WireDirection{direction: direction, distance: distance}
  end
end

defmodule DayThree do
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
end

input_file = "3.input"
parsed_input = with {:ok, contents} = File.read(input_file) do
  String.split(contents, "\n", trim: true)
  |> Enum.map(&(String.split &1, ",", trim: true))
  |> Enum.map( fn (wire) -> Enum.map(wire, &WireDirection.new/1) end)
end

DayThree.find_nearest_intersection(parsed_input)

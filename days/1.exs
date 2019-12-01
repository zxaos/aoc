defmodule DayOne do
  def raw_fuel_for_weight (weight) do
    div(weight, 3) - 2
  end

  def raw_fuel_for_modules (modules) do
    Enum.sum( Enum.map modules, &raw_fuel_for_weight/1 )
  end

  def compounding_fuel_for_modules (modules) do
    Enum.sum( Enum.map modules, &compounding_fuel/1 )
  end

  def compounding_fuel(weight) when weight >= 9 do # 9/3 - 2 = 1, discard anything 0 or smaller
    current = raw_fuel_for_weight(weight)
    current + compounding_fuel(current)
  end
  def compounding_fuel(_), do: 0

end

parsed_input = with {:ok, contents} = File.read("1.input") do
  split = String.split contents, "\n", trim: true
  Enum.map split, &(String.to_integer(&1))
end

IO.puts "Fuel for modules alone: #{DayOne.raw_fuel_for_modules parsed_input }"
IO.puts "Fuel for modules + fuel: #{DayOne.compounding_fuel_for_modules parsed_input }"

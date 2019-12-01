defmodule DayOne do
  def fuel_for_weight (weight) do
    div(weight, 3) - 2
  end

  def raw_fuel_for_modules (modules) do
    Enum.sum( Enum.map modules, &fuel_for_weight/1 )
  end

end

parsed_input = with {:ok, contents} = File.read("1.input") do
  split = String.split contents, "\n", trim: true
  Enum.map split, &(String.to_integer(&1))
end

IO.puts "Fuel for modules alone: #{DayOne.raw_fuel_for_modules parsed_input }"

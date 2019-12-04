defmodule DayTwo do

  def fetch_four_at(index, list) do
    Enum.map(index..index+3, &(Enum.at(list, &1)))
  end

  def run_program(list) do
    IO.puts "Running program with initial list:"
    IO.inspect list
    [a, b, c, d | _tail] = list
    run_step(0, list, [a,b,c,d])
  end

  def patch_program(list, noun, verb) do
    fixed_input = List.replace_at(list, 1, noun)
      |> List.replace_at(2, verb)
  end

  def run_step(index, tape, [1, a, b, dst]) do
    IO.puts "running step 'add' with command [#{1}, #{a}, #{b}, #{dst}] at index #{index}"
    tape = List.replace_at(tape, dst, Enum.at(tape, a) + Enum.at(tape, b))
    next_index = index + 4
    run_step(next_index, tape, fetch_four_at(next_index, tape))
  end

  def run_step(index, tape, [2, a, b, dst]) do
    IO.puts "running step 'mult' with command [#{2}, #{a}, #{b}, #{dst}] at index #{index}"
    tape = List.replace_at(tape, dst, Enum.at(tape, a) * Enum.at(tape, b))
    next_index = index + 4
    run_step(next_index, tape, fetch_four_at(next_index, tape))
  end

  def run_step(_index, tape, [99, _a, _b, _dst]) do
    IO.puts "running step 'end'. Final tape is:"
    IO.inspect tape
    [result | _rest] = tape
    result
  end

end

parsed_input = with {:ok, contents} = File.read("2.input") do
  split = String.split contents, ["\n", ","], trim: true
  Enum.map split, &(String.to_integer(&1))
end

# before running the program, replace position 1 with the value 12 and replace position 2 with the value 2.
fixed_input = DayTwo.patch_program(parsed_input, 12, 2)

IO.puts "Position zero value after program completion: #{DayTwo.run_program fixed_input }"

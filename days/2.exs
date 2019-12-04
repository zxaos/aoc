# Using lists for the random access of the intcode computer is wildly inefficient.
# This should probably have used a map, but I haven't learned them yet

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

  def find_patch_for_output(program, target, 0, 100) do
    IO.puts "Failed to find valid patch for target"
  end
  def find_patch_for_output(program, target, 100, verb) do
    find_patch_for_output(program, target, 0, verb + 1)
  end
  def find_patch_for_output(program, target, noun \\ 0, verb \\ 0) do
    IO.puts "testing patch: #{noun}, #{verb}"
    patched_program = patch_program(program, noun, verb)
    output = run_program patched_program
    if output == target do
      IO.puts "Success for target #{target} with noun #{noun} and verb #{verb}"
    else
      find_patch_for_output(program, target, noun + 1, verb)
    end
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

DayTwo.find_patch_for_output(parsed_input, 19690720)

defmodule AOC.ICC do
  @moduledoc """
  An Intcode computer simulator.

  An Intcode program is a list of integers separated by commas (like 1,0,0,3,99).

  Starting with the first number, the integers match an opcode which performs a specific instruction and reads some number of subsequent numbers. After performing the instruction, execution moves forward to the next instruction, a distance determined by how manuy values were consumed by the opcode.

  The opcode indicates what to do; for example, 99 means that the program is finished and should immediately halt. 

  Encountering an unknown opcode means something went wrong.

  The underlying implementation here uses a list, which is not optimal for random access. Future work shoudl switch it out for a map or possibly an erlang array, performance-dependent.
  """

  require Logger

  def run_program(tape, input \\ nil, output \\ &IO.puts/1 ) do
    Logger.info "Starting program run of length #{length(tape)}"
    Logger.debug inspect tape
  end

  # execute start - tape
  #
  # execute continue - tape, offset
  #
  # execute stop
  #
  # General operation flow:
  # Split an opcode into its parameter modes + opcode
  # Each opcode function receives:
  # The

  def opcode(1, modes, tape, head) do
    
  end

  def pad_modes(modes, count) do
    
  end

  def resolve_parameters(modes, tape, head) do
    
  end

  def patch_program(tape, noun, verb, position \\ 1) do
    Logger.info "Patching program"
    tape
    |> List.replace_at(position, noun)
    |> List.replace_at(position + 1, verb)
  end


end

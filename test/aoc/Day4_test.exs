defmodule CliTest do
  use ExUnit.Case
  doctest Aoc

  import Aoc.Day4, only: [
    has_adjacent_double_digits: 1,
    has_increasing_digits: 1,
    filter_range: 2,
  ]

  test "only two adjacent digits pass" do
    assert has_adjacent_double_digits(11) == true
    assert has_adjacent_double_digits(22) == true
    assert has_adjacent_double_digits(122) == true
    assert has_adjacent_double_digits(221) == true
    assert has_adjacent_double_digits(1221) == true
    assert has_adjacent_double_digits(2233) == true
    assert has_adjacent_double_digits(-11) == true
    assert has_adjacent_double_digits(112233) == true
    assert has_adjacent_double_digits(111122) == true
    assert has_adjacent_double_digits(2) == false
    assert has_adjacent_double_digits(21) == false
    assert has_adjacent_double_digits(212) == false
    assert has_adjacent_double_digits(2121) == false
    assert has_adjacent_double_digits(0) == false
    assert has_adjacent_double_digits(1234444) == false
  end

  test "only increasing digits pass" do
    assert has_increasing_digits(0) == true
    assert has_increasing_digits(1) == true
    assert has_increasing_digits(12) == true
    assert has_increasing_digits(122) == true
    assert has_increasing_digits(123) == true
    assert has_increasing_digits(1233) == true
    assert has_increasing_digits(1232) == false
    assert has_increasing_digits(32) == false
  end

  test "applies filters to range" do
    assert filter_range(20, 29) == [22]
  end
end

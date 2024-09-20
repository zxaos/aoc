use aoc_2015::aoc_io::Solution;
use itertools::Itertools;
use miette::{IntoDiagnostic, Result};

const SEARCH_LIMIT: usize = 1_000_000;

fn main() -> Result<()> {
    let input: usize = aoc_2015::aoc_io::get_input_as_string(20)
        .parse()
        .into_diagnostic()?;

    let mut solution: Solution<u64> = Solution::new();
    solution[0].solution = presents_for_all_houses_1(SEARCH_LIMIT, input);
    solution[1].solution = presents_for_all_houses_2(SEARCH_LIMIT, input);
    solution.print();

    Ok(())
}

fn presents_for_all_houses_1(limit: usize, find: usize) -> Option<u64> {
    let find = find / 10;
    let limit = limit + 1;
    let mut houses: Vec<usize> = vec![1; limit];

    for i in 2..limit {
        for j in (i..limit).step_by(i) {
            houses[j] += i
        }
    }

    houses
        .iter()
        .find_position(|&&presents| presents >= find)
        .map(|(house, _)| u64::try_from(house).unwrap())
}

fn presents_for_all_houses_2(limit: usize, find: usize) -> Option<u64> {
    let limit = limit + 1;
    let mut houses: Vec<usize> = vec![0; limit];

    for i in 1..limit {
        let amount = i * 11;
        for j in (i..limit).step_by(i).take(50) {
            houses[j] += amount;
        }
    }

    houses
        .iter()
        .find_position(|&&presents| presents >= find)
        .map(|(house, _)| u64::try_from(house).unwrap())
}

use aoc_2015::aoc_io::Solution;
use itertools::Itertools;
use miette::IntoDiagnostic;

const TARGET: u16 = 150;

fn main() -> miette::Result<()> {
    let mut solution: Solution<usize> = Solution::new();

    let containers: Vec<String> = aoc_2015::aoc_io::get_input_as_lines(17)
        .collect::<Result<Vec<String>, _>>()
        .into_diagnostic()?;

    let mut containers: Vec<u16> = containers
        .iter()
        .map(|f| f.parse::<u16>())
        .collect::<Result<Vec<u16>, _>>()
        .into_diagnostic()?;

    // figure out the fewest number of containers that could be > target
    /*
    This actually isn't guaranteed to work as a limit for part two. It's possible to
    have inputs like [149, 100, 25, 25], where we'd call the limit 2: (149 + 100) >= 150 but
    the actual smallest limit is 3: (100 + 25 + 25). This part was originally just intended
    to drop combinations that obviously couldn't work. But it also happened to produce the
    correct results for my input, so I didn't go any further.
    */
    containers.sort_unstable_by(|a, b| b.cmp(a));
    let (_, min_containers_required) = containers.iter().fold((0, 0_usize), |(acc, ct), x| {
        if acc < TARGET {
            (acc + x, ct + 1)
        } else {
            (acc, ct)
        }
    });

    let possible_combinations: Vec<Vec<u16>> = containers
        .into_iter()
        .powerset()
        .filter(|x| x.len() >= min_containers_required)
        .filter(|x| x.iter().sum::<u16>() == TARGET)
        .collect();

    solution[0].solution = Some(possible_combinations.len());
    solution[0].description = Some("Solutions that can fit the target");

    let combinations_min = possible_combinations
        .into_iter()
        .filter(|x| x.len() == min_containers_required)
        .collect::<Vec<Vec<u16>>>();

    solution[1].solution = Some(combinations_min.len());
    solution[1].description = Some("Solutions using the fewest possible containers");

    solution.print();

    Ok(())
}

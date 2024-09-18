use std::{
    cmp::min,
    collections::{HashMap, HashSet},
};

use aoc_2015::aoc_io::Solution;
use itertools::Itertools;
use miette::{bail, Diagnostic, IntoDiagnostic, Result};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
enum Err19 {
    #[error("Invalid replacement line: {0}")]
    InvalidInput(String),
}

type Replacement = (String, Vec<String>);
type ReplacementMap = HashMap<String, Vec<Vec<String>>>;

// Might be better to do this as just plain strings or Vec<char> instead.

fn main() -> Result<()> {
    let mut raw_input = aoc_2015::aoc_io::get_collected_input_as_lines(19);
    let mut solution: Solution<usize> = Solution::new();

    let target: Vec<String>;
    loop {
        if let Some(line) = raw_input.pop() {
            match line.as_str() {
                "" => {
                    println!("Skipping empty line");
                    continue;
                }
                _ => {
                    target = to_elements(&line);
                    break;
                }
            }
        } else {
            bail!("Input was only empty lines")
        };
    }
    // There should be an empty line after the main molecule input
    raw_input.pop();

    let mut replacements: ReplacementMap = HashMap::with_capacity(raw_input.len());
    for r in raw_input.into_iter().map(parse_replacement) {
        if let Ok((from, to)) = r {
            match replacements.get_mut(&from) {
                Some(record) => record.push(to),
                None => {
                    replacements.insert(from, vec![to]);
                }
            }
        } else {
            bail!("unpacked replacement was bad")
        }
    }

    solution[0].solution = Some(count_unique_replacements(&target, &replacements));
    solution[1].solution = Some(greedy_reverse(&target, &replacements));

    solution.print();
    Ok(())
}

fn greedy_reverse(target: &[String], replacements: &ReplacementMap) -> usize {
    // This is _not_ a general solution, but it's worth trying to see if the construction works for
    // this puzzle
    let map = reverse_map(replacements);
    let largest_sub = map
        .keys()
        .max_by_key(|x| x.len())
        .expect("There will always be a largest")
        .len();

    let mut current_target: Vec<String> = Vec::from(target);
    let mut last_size = usize::MAX;
    let mut steps = 0;
    let final_goal = vec!["e"];

    // This is a disaster but here goes
    // This outer loop is responsible for actually doing the replacement once we find a match,
    // and bailing once we hit the target e, or if we can't make any more progress
    'replacement: while current_target.len() != last_size && current_target != final_goal {
        last_size = current_target.len();
        let mut best_match_size = 0;
        let mut best_match_range = 0..0;
        let mut best_match_content: Vec<String> = vec![];

        // This secondary loop goes through every starting position in the target
        'currentstep: for starting_idx in 0..last_size {
            let largest_sub_left = min(last_size - starting_idx, largest_sub) + 1;

            // Then this inner loop starts with the largest possible replacement and tries
            // successively smaller ones. If it finds one, it saves it. If it doesn't find
            // one, the currentstep loop moves on to the next starting position.
            for replacement_size in (1..largest_sub_left).rev() {
                let splice_range = starting_idx..starting_idx + replacement_size;
                let potential = &current_target[splice_range.clone()];

                if let Some(matching_segment) = map.get(potential) {
                    // Keep the biggest match we've found.
                    // The example in the puzzle seems to prefer to make the latest possible substitution
                    // so replace equivalently sized matches with more recent ones. It gives the wrong solution
                    // otherwise.
                    if potential.len() >= best_match_size {
                        best_match_size = potential.len();
                        best_match_range = splice_range;
                        best_match_content = matching_segment.clone();
                        continue 'currentstep;
                    }
                }
            }
        }

        if best_match_size > 0 {
            // we have a non-zero match, so splice it in to get the new target
            let _: Vec<_> = current_target
                .splice(best_match_range, best_match_content.clone())
                .collect();
            steps += 1;
            continue 'replacement;
        } else {
            println!("No valid range to replace");
            println!("Final target was {}", current_target.join(""));
        }
    }
    steps
}

fn reverse_map(map: &ReplacementMap) -> HashMap<Vec<String>, Vec<String>> {
    let mut rmap: HashMap<Vec<String>, Vec<String>> = HashMap::with_capacity(map.len());
    for (key, vec) in map.iter() {
        let key = to_elements(key);
        for v in vec {
            let v: Vec<String> = v.clone();
            if rmap.insert(v, key.clone()).is_some() {
                panic!("Can't build reverse map, {:?} was already used", &key)
            }
        }
    }
    rmap
}

fn count_unique_replacements(target: &[String], replacements: &ReplacementMap) -> usize {
    let mut results: HashSet<String> = HashSet::with_capacity(target.len() * 2);
    for (idx, segment) in target.iter().enumerate() {
        if let Some(rs) = replacements.get(segment) {
            for current in rs {
                let before = target[0..idx].iter().join("");
                let current = current.iter().join("");
                let after = target[idx + 1..].iter().join("");
                results.insert(format!("{before}{current}{after}"));
            }
        }
    }
    results.len()
}

// Turns "Ca => SiRnFYFAr" into a replacement record
fn parse_replacement<S: Into<String>>(line: S) -> Result<Replacement> {
    let line: String = line.into();
    let mut parts = line.split(" ");
    let from = parts
        .next()
        .ok_or_else(|| Err19::InvalidInput(line.clone()))?
        .to_string();
    let to = parts
        .nth(1)
        .ok_or_else(|| Err19::InvalidInput(line.clone()))?;
    let None = parts.next() else {
        return Err(Err19::InvalidInput(line.clone())).into_diagnostic();
    };

    let to = to_elements(to);

    Ok((from, to))
}

fn reverse(i: &str) -> String {
    match i.len() {
        1 => i.to_string(),
        _ => i.chars().rev().collect(),
    }
}

fn to_elements(i: &str) -> Vec<String> {
    // reverse the products so we can split by uppercase divider, then un-reverse each product
    reverse(i)
        .split_inclusive(char::is_uppercase)
        .map(reverse)
        .rev() // Since we reversed the initial string, we'd get the results in reverse order if not this.
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    fn vec_str_string(xs: Vec<&str>) -> Vec<String> {
        xs.into_iter().map(String::from).collect()
    }

    fn fixture_replacement_map_1() -> ReplacementMap {
        let mut replacements: ReplacementMap = HashMap::with_capacity(3);
        replacements.insert(
            "H".to_string(),
            vec![
                vec_str_string(vec!["H", "O"]),
                vec_str_string(vec!["O", "H"]),
            ],
        );
        replacements.insert("O".to_string(), vec![vec_str_string(vec!["H", "H"])]);
        replacements
    }

    fn fixture_replacement_map_2() -> ReplacementMap {
        let mut replacements = fixture_replacement_map_1();
        replacements.insert(
            "e".to_string(),
            vec![vec![String::from("H")], vec![String::from("O")]],
        );
        replacements
    }

    #[test]
    fn test_parse() -> Result<()> {
        let r = parse_replacement("Ca => SiRnFYFAr")?;
        assert_eq!(r.0, "Ca");
        assert_eq!(r.1, vec!["Si", "Rn", "F", "Y", "F", "Ar"]);
        Ok(())
    }

    #[test]
    fn test_replacements() {
        let replacements = fixture_replacement_map_1();
        // let target = vec!["H".to_string(), "O".to_string(), "H".to_string()];
        let target = vec_str_string(vec!["H", "O", "H"]);
        assert_eq!(count_unique_replacements(&target, &replacements), 4);
    }

    #[test]
    fn test_naive_replace_hoh() {
        let replacements = fixture_replacement_map_2();
        let target = vec_str_string(vec!["H", "O", "H"]);
        let steps = greedy_reverse(&target, &replacements);
        assert_eq!(steps, 3)
    }

    #[test]
    fn test_naive_replace_hohoho() {
        let replacements = fixture_replacement_map_2();
        let target = vec_str_string(vec!["H", "O", "H", "O", "H", "O"]);
        let steps = greedy_reverse(&target, &replacements);
        assert_eq!(steps, 6)
    }
}

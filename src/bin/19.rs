use std::collections::{HashMap, HashSet};

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

    solution[0].solution = Some(count_unique_replacements(target, replacements));
    solution.print();
    Ok(())
}

fn count_unique_replacements(target: Vec<String>, replacements: ReplacementMap) -> usize {
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
    #[test]
    fn test_parse() -> Result<()> {
        let r = parse_replacement("Ca => SiRnFYFAr")?;
        assert_eq!(r.0, "Ca");
        assert_eq!(r.1, vec!["Si", "Rn", "F", "Y", "F", "Ar"]);
        Ok(())
    }

    #[test]
    fn test_replacements() {
        let mut replacements: ReplacementMap = HashMap::with_capacity(3);
        replacements.insert(
            "H".to_string(),
            vec![
                vec!["H".to_string(), "O".to_string()],
                vec!["O".to_string(), "H".to_string()],
            ],
        );
        replacements.insert(
            "O".to_string(),
            vec![vec!["H".to_string(), "H".to_string()]],
        );
        let target = vec!["H".to_string(), "O".to_string(), "H".to_string()];
        assert_eq!(count_unique_replacements(target, replacements), 4);
    }
}

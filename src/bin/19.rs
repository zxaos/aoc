use miette::{Diagnostic, IntoDiagnostic, Result};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
enum Err19 {
    #[error("Invalid replacement line: {0}")]
    InvalidInput(String),
}

type Replacement = (String, Vec<String>);

fn main() -> Result<()> {
    let raw_input = aoc_2015::aoc_io::get_collected_input_as_lines(19);
    Ok(())
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
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() -> Result<()> {
        let r = parse_replacement("Ca => SiRnFYFAr")?;
        assert_eq!(r.0, "Ca");
        let mut correct = vec!["Si", "Rn", "F", "Y", "F", "Ar"];
        correct.reverse();
        assert_eq!(r.1, correct);
        Ok(())
    }
}

use itertools::Itertools;
use std::ops::RangeInclusive;

fn main() {
    let initial_pass = aoc_2015::aoc_io::get_input_as_string(11);
    let one = next_valid_password(&initial_pass);
    let two = next_valid_password(&one);
    aoc_2015::aoc_io::put_aoc_output((Some(one), Some(two)));
}

const LOWER_A: u8 = b'a';
const LOWER_Z: u8 = b'z';
const LC_U8: RangeInclusive<u8> = LOWER_A..=LOWER_Z;
fn next_valid_password(current: &str) -> String {
    let mut cnums: Vec<u8> = current.chars().map(|c| c as u8).collect();
    let lsd = cnums.len() - 1;
    loop {
        cnums[lsd] += 1;
        // now loop the positions, rolling over on > Z and break as soon as that's not necessary.
        let mut position = lsd;
        loop {
            if cnums[position] > LOWER_Z {
                cnums[position] = LOWER_A;
                if position > 0 {
                    position -= 1;
                } else {
                    position = lsd;
                }
                cnums[position] += 1
            } else {
                break;
            }
        }
        if validate_password(&cnums) {
            return String::from_utf8(cnums).expect("Generated string was invalid, this is a bug");
        }
    }
}

const BANNED_LETTERS: [u8; 3] = [b'i', b'l', b'o'];
fn validate_password(password: &Vec<u8>) -> bool {
    // Passwords must be exactly eight lowercase letters (for security reasons)
    // Passwords may not contain the letters i, o, or l, as these letters can be mistaken for other characters and are therefore confusing.
    // Passwords must include one increasing straight of at least three letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
    // Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.
    if password.len() != 8
        || !password.iter().all(|c| LC_U8.contains(c))
        || password.iter().any(|c| BANNED_LETTERS.contains(c))
        || !password.iter().tuple_windows().any(sequential_chars)
        || non_overlapping_pairs(password) < 2
    {
        return false;
    }
    true
}

fn sequential_chars(xs: (&u8, &u8, &u8)) -> bool {
    let (x, y, z) = xs;
    y.wrapping_sub(*x) == 1 && z.wrapping_sub(*y) == 1
}

fn non_overlapping_pairs(s: &[u8]) -> u8 {
    let mut count = 0;
    let mut pairs = s.iter().tuple_windows();
    while let Some((x, y)) = pairs.next() {
        if x == y {
            count += 1;
            pairs.next(); // advance once since overlap is not allowed
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_validate_password() {
        let examples = vec![
            ("hijklmmn", false),
            ("abbceffg", false),
            ("abbcegjk", false),
            ("ghjaabcc", true),
        ];
        for (pass, valid) in examples {
            let passvec: Vec<u8> = pass.chars().map(|c| c as u8).collect();
            assert_eq!(
                validate_password(&passvec),
                valid,
                "Expected {} to be valid: {}",
                pass,
                valid
            );
        }
    }

    #[test]
    fn test_next_password() {
        assert_eq!(next_valid_password("ghijklmn"), "ghjaabcc");
    }
}

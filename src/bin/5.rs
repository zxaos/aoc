use aoc_2015::aoc_io;
use std::collections::HashMap;

fn main() {
    // We'll need to iterate twice so stick the lines into a vec and iterate that instead of the file directly
    let input: Vec<String> = aoc_io::get_input_as_lines(5)
        .map(|line| line.unwrap())
        .collect();
    let nice = input.iter().filter(|name| is_name_nice(name));
    let nice_count = i64::try_from(nice.count()).unwrap();
    let nice_redux = input.iter().filter(|name| is_name_nice_redux(name));
    let nice_count_redux = i64::try_from(nice_redux.count()).unwrap();

    aoc_io::put_aoc_named_output(
        (Some(nice_count), Some(nice_count_redux)),
        "Nice",
        "Nice (Updated Rules)",
    );
}

fn is_name_nice(name: &str) -> bool {
    at_least_three_vowels(name) && match_char_distance(name, 1) && no_bad_strings(name)
}

fn is_name_nice_redux(name: &str) -> bool {
    has_letter_pair(name) && match_char_distance(name, 2)
}

fn at_least_three_vowels(s: &str) -> bool {
    let mut vowels = 0;
    for c in s.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
            _ => continue,
        }
        if vowels > 2 {
            return true;
        }
    }
    false
}

// This is a generalized version of the pair checker for part one
// and the xyx checker for part 2
fn match_char_distance(s: &str, distance: usize) -> bool {
    let mut iter_pairs = s.chars().zip(s.chars().skip(distance));
    iter_pairs.any(|(x, y)| x == y)
}

fn no_bad_strings(s: &str) -> bool {
    !(s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy"))
}

fn has_letter_pair(s: &str) -> bool {
    // Iterate over every sequential pair of letters
    let iter_pairs = s.char_indices().zip(s.chars().skip(1));
    let mut seen_pairs: HashMap<String, usize> = HashMap::new();
    for x in iter_pairs {
        let ((idx, first), second) = x;
        let pair = format!("{first}{second}");
        let loc = seen_pairs.entry(pair).or_insert(idx + 1);
        // Store the idx of the second letter instead of the first
        // Then, when we check for strictly less than, below, we can be sure
        // the pair is not actually an overlapping set of three letters

        if *loc < idx {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d5_good_vowels() {
        assert!(at_least_three_vowels("aei"), "aei");
        assert!(at_least_three_vowels("uuuu"), "uuuu");
        assert!(!at_least_three_vowels("aoh"));
        assert!(!at_least_three_vowels("abcdefgh"));
    }

    #[test]
    fn d5_has_doubles() {
        assert!(match_char_distance("aa", 1));
        assert!(match_char_distance("abcddcba", 1));
        assert!(match_char_distance("abcdeff", 1));
        assert!(!match_char_distance("a", 1));
        assert!(!match_char_distance("abcdefgh", 1));
    }

    #[test]
    fn d5_bad_strings() {
        assert!(no_bad_strings("aceg"));
        assert!(no_bad_strings("zyxwvut"));
        assert!(!no_bad_strings("abcd"));
        assert!(!no_bad_strings("zzzzzzzxy"));
    }

    #[test]
    fn d5_nice_names() {
        // ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
        assert!(is_name_nice("ugknbfddgicrmopn"));
        // aaa is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
        assert!(is_name_nice("aaa"));
    }

    #[test]
    fn d5_naughty_names() {
        // jchzalrnumimnmhp is naughty because it has no double letter.
        assert!(!is_name_nice("jchzalrnumimnmhp"));
        // haegwjzuvuyypxyu is naughty because it contains the string xy.
        assert!(!is_name_nice("haegwjzuvuyypxyu"));
        // dvszwmarrgswjxmb is naughty because it contains only one vowel
        assert!(!is_name_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn d5_letterpairs() {
        assert!(!has_letter_pair("abcde"));
        assert!(has_letter_pair("aabcdeaa"));
        assert!(!has_letter_pair("aaa")); // pairs can't overlap
    }

    #[test]
    fn d5_xyx() {
        // xyx, abcdefeghi (efe), or even aaa.
        assert!(match_char_distance("xyx", 2));
        assert!(match_char_distance("abcdefeghi", 2));
        assert!(match_char_distance("aaa", 2));
        assert!(!match_char_distance("xyz", 2));
        assert!(!match_char_distance("a", 2));
    }

    #[test]
    fn d5_nice_redux() {
        // qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly one letter between them (zxz).
        assert!(is_name_nice_redux("qjhvhtzxzqqjkmpb"));
        // xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
        assert!(is_name_nice_redux("xxyxx"));
    }

    #[test]
    fn d5_naughty_redux() {
        // uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a single letter between them.
        assert!(!is_name_nice_redux("uurcxstgmygtbstg"));
        // ieodomkazucvgmuy is naughty because it has a repeating letter with one between (odo), but no pair that appears twice.
        assert!(!is_name_nice_redux("ieodomkazucvgmuy"));
    }
}

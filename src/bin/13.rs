use std::{
    sync::{mpsc, Arc},
    thread,
};

use micromap::Map;
use micromap::Set;

use itertools::Itertools;

type WeightMap = Map<u16, i16, 29>;
type GuestList = Set<char, 9>;

pub fn main() {
    let lines = aoc_2015::aoc_io::get_collected_input_as_lines(13);
    let raw_weights: Vec<(String, String, i16)> = lines.iter().map(|l| parseline(l)).collect();
    let mut names: GuestList = Set::new();
    raw_weights
        .iter()
        .for_each(|(name, _, _)| names.insert(name.chars().next().unwrap()));
    let mut weights = build_weighting_table(raw_weights);
    let result_1 = search_exhaustive(&names, weights.clone());
    weights.insert(0, 0);
    names.insert('0');
    let result_2 = search_exhaustive(&names, weights);
    aoc_2015::aoc_io::put_aoc_output((Some(result_1), Some(result_2)));
}

/* A bunch of assumptions here, validated by looking at our puzzle input:
 * The first letter of each person's name is unique.
 * The count of guests is exactly 8 (+1)
 * The graph is fully connected.
 * 8C2 = 28, so we can should use micromap instead of proper HashMap for speed
 * We're hardcoding in the map size for performance and cannot accept larger guest lists
 * +1 for special zero value
*/

fn key_from_chars(a: char, b: char) -> u16 {
    if a == '0' || b == '0' {
        return 0;
    }
    let a: u16 = a as u16;
    let b: u16 = b as u16;
    if a < b {
        (a << 8) + b
    } else {
        (b << 8) + a
    }
}

fn key_from_names<T: AsRef<str>>(a: T, b: T) -> u16 {
    let a: char = a.as_ref()[0..1].as_bytes()[0].into();
    let b: char = b.as_ref()[0..1].as_bytes()[0].into();
    key_from_chars(a, b)
}

fn build_weighting_table<T: AsRef<str> + std::fmt::Debug>(edges: Vec<(T, T, i16)>) -> WeightMap {
    let mut weights: WeightMap = Map::new();

    // Build a lookup table for the distance between every pair of guests
    for (a, b, weight) in edges {
        let key = key_from_names(&a, &b);
        match weights.get_mut(&key) {
            Some(w) => {
                *w += weight;
            }
            None => weights.insert(key, weight),
        }
    }

    weights
}

fn search_exhaustive(guests: &GuestList, weights: WeightMap) -> i32 {
    let (tx, rx) = mpsc::channel();

    // get permutations of guests
    let template_permutations: Vec<Vec<char>> =
        guests.iter().cloned().permutations(guests.len()).collect();
    let template_permutations = Arc::new(template_permutations);
    let template_guests = Arc::new(guests);
    let template_weights = Arc::new(weights);
    for initial in template_guests.iter() {
        let thread_init = *initial;
        let thread_permutation = Arc::clone(&template_permutations);
        let thread_weights = Arc::clone(&template_weights);
        let thread_tx = tx.clone();
        thread::spawn(move || {
            let mut best = i32::MIN;
            for arrangement in thread_permutation.iter() {
                if arrangement[0] != thread_init {
                    continue;
                }

                let mut score: i32 = 0;
                for pair in arrangement.windows(2) {
                    let key = key_from_chars(pair[0], pair[1]);
                    let weight = *thread_weights.get(&key).unwrap();
                    score += i32::from(weight);
                }
                // last and first guests
                score += i32::from(
                    *thread_weights
                        .get(&key_from_chars(
                            thread_init,
                            arrangement[arrangement.len() - 1],
                        ))
                        .unwrap(),
                );

                best = best.max(score);
            }
            thread_tx.send(best).expect("Failed to send result");
        });
    }

    let mut best_all_threads = ::std::i32::MIN;
    for _ in template_guests.iter() {
        let result = rx.recv().unwrap();
        best_all_threads = best_all_threads.max(result);
    }
    best_all_threads
}

fn parseline(line: &str) -> (String, String, i16) {
    // Alice would gain 54 happiness units by sitting next to Bob.
    // 0     1     2    3  4         5     6  7       8    9  10
    // but don't forget consuming the iterator shifts everything
    let mut parts = line.trim_end_matches('.').split(' ');
    let a = parts.next().expect("Bad format: First Person");
    let polarity = parts.nth(1).expect("Bad format: gain/lose");
    let mut weight = parts
        .next()
        .and_then(|diststring| diststring.parse::<i16>().ok())
        .expect("Bad format: Weight");
    if polarity == "lose" {
        weight *= -1;
    }
    let b = parts.nth(6).expect("Bad format: Second Person");
    (a.to_owned(), b.to_owned(), weight)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_positive() {
        let (a, b, weight) =
            parseline("Alice would gain 54 happiness units by sitting next to Bob.");
        assert_eq!(a, "Alice");
        assert_eq!(b, "Bob");
        assert_eq!(weight, 54_i16);
    }
    #[test]
    fn test_parse_negative() {
        let (a, b, weight) =
            parseline("Alice would lose 79 happiness units by sitting next to Carol.");
        assert_eq!(a, "Alice");
        assert_eq!(b, "Carol");
        assert_eq!(weight, -79_i16);
    }

    #[test]
    fn test_key() {
        assert_eq!(
            key_from_names("Alice", "Bob"),
            key_from_names("Bob", "Alice")
        );
        assert_ne!(
            key_from_names("Alice", "Carol"),
            key_from_names("Alice", "Bob")
        );
    }

    #[test]
    fn test_weight_table() {
        let test_weights = vec![
            ("Alice", "Bob", 54),
            ("Alice", "Carol", -79),
            ("Bob", "Alice", 83),
            ("Bob", "Carol", -7),
            ("Carol", "Alice", -62),
            ("Carol", "Bob", 60),
        ];
        let weights = build_weighting_table(test_weights);
        assert_eq!(weights.get(&key_from_names("Alice", "Bob")), Some(&137));
        assert_eq!(weights.get(&key_from_names("Alice", "Carol")), Some(&-141));
        assert_eq!(weights.get(&key_from_names("Bob", "Carol")), Some(&53));
    }

    #[test]
    fn test_optimize_weight() {
        let test_weights = vec![
            ("Alice", "Bob", 54),
            ("Alice", "Carol", -79),
            ("Alice", "David", -2),
            ("Bob", "Alice", 83),
            ("Bob", "Carol", -7),
            ("Bob", "David", -63),
            ("Carol", "Alice", -62),
            ("Carol", "Bob", 60),
            ("Carol", "David", 55),
            ("David", "Alice", 46),
            ("David", "Bob", -7),
            ("David", "Carol", 41),
        ];
        let mut guests: GuestList = Set::new();
        guests.insert('A');
        guests.insert('B');
        guests.insert('C');
        guests.insert('D');
        let table = build_weighting_table(test_weights);
        let result = search_exhaustive(&guests, table);
        assert_eq!(result, 330);
    }
}

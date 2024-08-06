use aoc_2015::aoc_io::Solution;
use std::{
    sync::{mpsc, Arc},
    thread,
};

use micromap::Map;
use micromap::Set;

use itertools::Itertools;

type WeightMap = Map<u16, i32, 28>;
type GuestList = Set<char, 8>;

pub fn main() {
    let mut solution: Solution<i32, &str> = Solution::new();
    let lines = aoc_2015::aoc_io::get_collected_input_as_lines(13);
    let raw_weights: Vec<(String, String, i32)> = lines.iter().map(|l| parseline(l)).collect();
    let mut names: GuestList = Set::new();
    raw_weights
        .iter()
        .for_each(|(name, _, _)| names.insert(name.chars().next().unwrap()));
    let weights = build_weighting_table(raw_weights);
    let search_results = search_exhaustive(&names, weights);
    solution[0].solution = Some(search_results.0);
    solution[0].description = Some("Best possible seating score (no host)");
    solution[1].solution = Some(search_results.1);
    solution[1].description = Some("Best possible seating score (with host)");
    solution.print();
}

/* A bunch of assumptions here, validated by looking at our puzzle input:
 * The first letter of each person's name is unique.
 * The count of guests is exactly 8
 * The graph is fully connected.
 * 8C2 = 28, so we can should use micromap instead of proper HashMap for speed
 * We're hardcoding in the map size for performance and cannot accept larger guest lists
*/

fn key_from_chars(a: char, b: char) -> u16 {
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

fn build_weighting_table<T: AsRef<str> + std::fmt::Debug>(edges: Vec<(T, T, i32)>) -> WeightMap {
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

fn search_exhaustive(guests: &GuestList, weights: WeightMap) -> (i32, i32) {
    // Fix the first guest in place, then run the permutations of every other guest.
    // This removes entire chunks of rotationally equivalent arrangements
    let (tx, rx) = mpsc::channel();

    // get permutations of guests
    let fixed_guest = *guests.iter().next().unwrap();
    let template_permutations: Vec<Vec<char>> = guests
        .iter()
        .skip(1)
        .cloned()
        .permutations(guests.len() - 1)
        .collect();
    let template_permutations = Arc::new(template_permutations);
    let template_guests = Arc::new(guests);
    let template_weights = Arc::new(weights);
    for initial in template_guests.iter().skip(1) {
        let thread_init = *initial;
        let thread_permutation = Arc::clone(&template_permutations);
        let thread_weights = Arc::clone(&template_weights);
        let thread_tx = tx.clone();
        thread::spawn(move || {
            let mut best = i32::MIN;
            let mut best_with_host = i32::MIN;
            let first_guest = [fixed_guest];
            for arrangement in thread_permutation.iter() {
                if arrangement[0] != thread_init {
                    continue;
                }
                let mut score: i32 = 0;
                let mut worst_pair = i32::MAX;
                // Now re-add the fixed first guest at the start and end
                // to get the first and last pairs
                let arr_and_fixed: Vec<&char> = first_guest
                    .iter()
                    .chain(arrangement.iter())
                    .chain(first_guest.iter())
                    .collect();

                for pair in arr_and_fixed.windows(2) {
                    let key = key_from_chars(*pair[0], *pair[1]);
                    let weight = *thread_weights.get(&key).unwrap();
                    score += weight;
                    worst_pair = worst_pair.min(weight);
                }

                best = best.max(score);
                // Adding in the 0 - score host will nullify the worst scoring pair.
                best_with_host = best_with_host.max(score - worst_pair);
            }
            thread_tx
                .send((best, best_with_host))
                .expect("Failed to send result");
        });
    }

    let mut best_all_threads = ::std::i32::MIN;
    let mut best_all_threads_host = ::std::i32::MIN;
    for _ in 0..template_guests.len() - 1 {
        let result = rx.recv().unwrap();
        best_all_threads = best_all_threads.max(result.0);
        best_all_threads_host = best_all_threads_host.max(result.1);
    }
    (best_all_threads, best_all_threads_host)
}

fn parseline(line: &str) -> (String, String, i32) {
    // Alice would gain 54 happiness units by sitting next to Bob.
    // 0     1     2    3  4         5     6  7       8    9  10
    // but don't forget consuming the iterator shifts everything
    let mut parts = line.trim_end_matches('.').split(' ');
    let a = parts.next().expect("Bad format: First Person");
    let polarity = parts.nth(1).expect("Bad format: gain/lose");
    let mut weight = parts
        .next()
        .and_then(|diststring| diststring.parse::<i32>().ok())
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
        assert_eq!(weight, 54);
    }
    #[test]
    fn test_parse_negative() {
        let (a, b, weight) =
            parseline("Alice would lose 79 happiness units by sitting next to Carol.");
        assert_eq!(a, "Alice");
        assert_eq!(b, "Carol");
        assert_eq!(weight, -79);
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
        assert_eq!(result.0, 330);
    }
}

use std::{
    collections::HashMap,
    sync::{mpsc, Arc},
    thread,
};

use itertools::Itertools;

pub fn main() {
    let lines = aoc_2015::aoc_io::get_collected_input_as_lines(9);
    let raw_routes: Vec<(String, String, u16)> = lines.iter().map(|l| parseline(l)).collect();

    // The first n entries will have the distances from one location to every other location
    // So count until we see a different starting location, and then add one for the initial location.
    let total_locations = raw_routes
        .iter()
        .take_while(|(a, _, _)| a == &raw_routes[0].0)
        .count()
        + 1;

    let mut names: Vec<String> = Vec::with_capacity(total_locations);
    let mut name_lookup: HashMap<String, usize> = HashMap::with_capacity(total_locations);
    let mut distances: Vec<Vec<u16>> = vec![vec![0; total_locations]; total_locations];

    for (a, b, dist) in raw_routes {
        let a_idx = insert_or_index(&a, &mut names, &mut name_lookup);
        let b_idx = insert_or_index(&b, &mut names, &mut name_lookup);
        distances[a_idx][b_idx] = dist;
        distances[b_idx][a_idx] = dist;
    }
    let (tx, rx) = mpsc::channel();
    let possible_routes = (0..total_locations).permutations(total_locations);

    let local_distances = Arc::new(distances);
    for starting_location in 0..total_locations {
        let thread_tx = tx.clone();
        let thread_routes = possible_routes.clone();
        let thread_distances = Arc::clone(&local_distances);
        thread::spawn(move || {
            let mut best_path = vec![];
            let thread_routes = thread_routes.filter(|x| x[0] == starting_location);
            let mut best_distance = u16::MAX;
            for path in thread_routes {
                if path[0] != starting_location {
                    continue;
                }
                let mut current_distance = 0;
                for segment in path.windows(2) {
                    let from = segment[0];
                    let to = segment[1];
                    current_distance += thread_distances[from][to];
                }
                if current_distance < best_distance {
                    best_distance = current_distance;
                    best_path = path;
                }
            }
            println!(
                "Thread for starting location {starting_location} sending my best path {:?} with distance {best_distance}",
                best_path
            );
            thread_tx
                .send((best_distance, best_path))
                .expect("Failed to send result");
        });
    }

    let mut best_path = vec![];
    let mut best_distance = u16::MAX;
    for _ in 0..total_locations {
        let (thread_distance, thread_path) = rx.recv().unwrap();
        if thread_distance < best_distance {
            best_distance = thread_distance;
            best_path = thread_path;
        }
    }
    println!("---");
    println!(
        "Shortest distance is {best_distance} using path {:?}",
        best_path
    );
}

pub fn insert_or_index(
    name: &str,
    all_names: &mut Vec<String>,
    lookup: &mut HashMap<String, usize>,
) -> usize {
    match lookup.get(name) {
        Some(index) => *index,
        None => {
            let index = all_names.len();
            all_names.push(name.to_owned());
            lookup.insert(name.to_owned(), index);
            index
        }
    }
}

fn parseline(line: &str) -> (String, String, u16) {
    let mut parts = line.split(' ');
    let a = parts.next().expect("Bad format: First Location");
    let b = parts.nth(1).expect("Bad format: Second Location");
    let dist = parts
        .nth(1)
        .and_then(|diststring| diststring.parse::<u16>().ok())
        .expect("Bad format: Distance");
    (a.to_owned(), b.to_owned(), dist)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let (a, b, dist) = parseline("Faerun to AlphaCentauri = 3");
        assert_eq!(a, "Faerun");
        assert_eq!(b, "AlphaCentauri");
        assert_eq!(dist, 3_u16);
    }
}

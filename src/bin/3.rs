use aoc_2015::aoc_io;
use std::collections::HashMap;

fn main() {
    let input = aoc_io::get_input_as_string(3);

    let houses_basic = run_delivery_path(&input, 1);
    let total_houses_basic = count_houses(&houses_basic);
    let houses_multi = run_delivery_path(&input, 2);
    let total_houses_multi = count_houses(&houses_multi);

    aoc_io::put_aoc_named_output(
        (Some(total_houses_basic), Some(total_houses_multi)),
        "Total Houses (Santa)",
        "Totan Houses (Santa + Robo Santa)",
    );
}

fn run_delivery_path(flightpath: &str, deliverers: usize) -> HashMap<(i64, i64), u64> {
    if deliverers < 1 {
        panic!("At least 1 santa is required")
    }
    let mut houses: HashMap<(i64, i64), u64> = HashMap::new();
    let mut positions: Vec<(i64, i64)> = Vec::with_capacity(deliverers);
    for _ in 0..deliverers {
        positions.push((0, 0));
    }
    houses.insert((0, 0), deliverers as u64);
    for (idx, direction) in flightpath.chars().enumerate() {
        let delta = match direction {
            '^' => (0, 1),
            '>' => (1, 0),
            'v' => (0, -1),
            '<' => (-1, 0),
            x => panic!("Unexpected travel direction `{x}`"),
        };
        let current_deliverer_idx = idx % deliverers;
        let mut current_position = positions[current_deliverer_idx];
        current_position.0 += delta.0;
        current_position.1 += delta.1;
        positions[current_deliverer_idx] = current_position;
        let count = houses.entry(current_position).or_insert(0);
        *count += 1;
    }
    houses
}

fn count_houses(houses: &HashMap<(i64, i64), u64>) -> i64 {
    i64::try_from(houses.len()).expect("Failed to convert house map size to i64")
}

#[cfg(test)]
mod tests {
    use super::*;
    // Tests for single deliverer:
    // > delivers presents to 2 houses: one at the starting location, and one to the east.
    #[test]
    fn d3_goright() {
        let houses = run_delivery_path(">", 1);
        let count = count_houses(&houses);
        assert_eq!(count, 2);
    }
    // ^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
    #[test]
    fn d3_gosquare() {
        let houses = run_delivery_path("^>v<", 1);
        let count = count_houses(&houses);
        assert_eq!(count, 4);
    }
    // ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.
    #[test]
    fn d3_goupdown() {
        let houses = run_delivery_path("^v^v^v^v^v", 1);
        let count = count_houses(&houses);
        assert_eq!(count, 2);
    }

    //Tests for mulitple deliverers:
    //^v delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
    #[test]
    fn d3_basic_multi() {
        let houses = run_delivery_path("^v", 2);
        let count = count_houses(&houses);
        assert_eq!(count, 3);
    }
    //^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
    #[test]
    fn d3_gosquare_multi() {
        let houses = run_delivery_path("^>v<", 2);
        let count = count_houses(&houses);
        assert_eq!(count, 3);
    }
    //^v^v^v^v^v now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.
    #[test]
    fn d3_goupdown_multi() {
        let houses = run_delivery_path("^v^v^v^v^v", 2);
        let count = count_houses(&houses);
        assert_eq!(count, 11);
    }
}

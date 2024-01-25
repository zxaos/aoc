use aoc_2015::aoc_io;

fn main() {
    let input = aoc_io::get_input_as_string(1);
    let result = count_floors(input);
    aoc_io::put_aoc_output((Some(result.0), Some(result.1)));
}

fn count_floors(instructions: String) -> (i64, i64) {
    let mut floor = 0;
    let mut first_basement: Option<i64> = None;

    for (i, c) in instructions.trim().chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Unexpected input character"),
        }
        if first_basement.is_none() && floor < 0 {
            first_basement = Some(i as i64 + 1);
        }
    }
    (floor, first_basement.unwrap())
}

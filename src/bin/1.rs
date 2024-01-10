use std::{fs::File, io::Read, path::Path};

fn main() {
    let path = Path::new("inputs/input.1.txt");
    let mut file = File::open(&path).expect("Failed to open input");

    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Failed to read input");

    let result = count_floors(input);
    println!("1: {}", result.0);
    println!("2: {}", result.1);
}

fn count_floors(instructions: String) -> (i32, u32) {
    let mut floor = 0;
    let mut first_basement: Option<u32> = None;

    for (i, c) in instructions.trim().chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Unexpected input character"),
        }
        if first_basement.is_none() && floor < 0 {
            first_basement = Some(i as u32 + 1);
        }
    }
    return (floor, first_basement.unwrap());
}

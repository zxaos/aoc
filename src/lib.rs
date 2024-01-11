pub mod aoc_io {
    use std::{
        fs::File,
        io::{self, BufRead, Read},
        path::Path,
    };

    pub fn get_input_as_string(day: u8) -> String {
        let mut file = get_file_from_day(day);
        let mut input = String::new();
        file.read_to_string(&mut input)
            .expect("Failed to read input");
        input
    }

    pub fn get_input_as_lines(day: u8) -> io::Lines<io::BufReader<File>> {
        let file = get_file_from_day(day);
        let reader = io::BufReader::new(file);
        reader.lines()
    }

    fn get_file_from_day(day: u8) -> File {
        let pathstring = format!("inputs/input.{}.txt", day);
        let path = Path::new(&pathstring);
        let file = File::open(&path).expect("Failed to open input");
        file
    }

    pub fn put_aoc_named_output(results: (Option<i64>, Option<i64>), first: &str, second: &str) {
        maybe_print_solution(results.0, 1, Some(first));
        maybe_print_solution(results.1, 2, Some(second));
    }

    pub fn put_aoc_output(results: (Option<i64>, Option<i64>)) {
        maybe_print_solution(results.0, 1, None);
        maybe_print_solution(results.1, 2, None);
    }

    fn maybe_print_solution(result: Option<i64>, part: u8, description: Option<&str>) {
        if let Some(r) = result {
            if let Some(description) = description {
                println!("{} - {}: {}", part, description, r);
            } else {
                println!("{}: {}", part, r);
            }
        } else {
            println!("No solution (yet) for part {part}.")
        }
    }
}

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

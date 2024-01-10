pub mod aoc_io {
    use std::{fs::File, io::Read, path::Path};

    pub fn get_aoc_input_as_string(day: u8) -> String {
        let pathstring = format!("inputs/input.{}.txt", day);
        let path = Path::new(&pathstring);
        let mut file = File::open(&path).expect("Failed to open input");

        let mut input = String::new();
        file.read_to_string(&mut input)
            .expect("Failed to read input");
        input
    }

    pub fn put_aoc_output(results: (Option<i64>, Option<i64>)) {
        maybe_print_solution(results.0, 1);
        maybe_print_solution(results.1, 2);
    }

    fn maybe_print_solution(result: Option<i64>, part: u8) {
        if let Some(r) = result {
            println!("{}: {}", part, r);
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

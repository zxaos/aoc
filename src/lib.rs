pub mod aoc_io {
    use std::{
        fmt::Display,
        fs::File,
        io::{self, BufRead, Read},
        path::Path,
    };

    pub fn get_input_as_string(day: u8) -> String {
        let mut file = get_file_from_day(day);
        let mut input = String::new();
        file.read_to_string(&mut input)
            .expect("Failed to read input");
        input.trim().to_string()
    }

    pub fn get_input_as_lines(day: u8) -> io::Lines<io::BufReader<File>> {
        let file = get_file_from_day(day);
        let reader = io::BufReader::new(file);
        reader.lines()
    }

    pub fn get_collected_input_as_lines(day: u8) -> Vec<String> {
        let buf = get_input_as_lines(day);
        let lines: Result<Vec<String>, _> = buf.collect();
        lines.expect("Failed to read input")
    }

    fn get_file_from_day(day: u8) -> File {
        let pathstring = format!("inputs/input.{}.txt", day);
        let path = Path::new(&pathstring);
        File::open(path).expect("Failed to open input")
    }

    pub fn put_aoc_named_output<T: Display>(
        results: (Option<T>, Option<T>),
        first: &str,
        second: &str,
    ) {
        maybe_print_solution(results.0, 1, Some(first));
        maybe_print_solution(results.1, 2, Some(second));
    }

    pub fn put_aoc_output<T: Display>(results: (Option<T>, Option<T>)) {
        maybe_print_solution(results.0, 1, None);
        maybe_print_solution(results.1, 2, None);
    }

    fn maybe_print_solution<T: Display>(result: Option<T>, part: u8, description: Option<&str>) {
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

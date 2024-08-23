pub mod aoc_io {
    use std::{
        fmt,
        fmt::Display,
        fs::File,
        io::{self, BufRead, Read},
        ops::{Deref, DerefMut},
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
        let reader = get_input_as_reader(day);
        reader.lines()
    }

    pub fn get_collected_input_as_lines(day: u8) -> Vec<String> {
        let buf = get_input_as_lines(day);
        let lines: Result<Vec<String>, _> = buf.collect();
        lines.expect("Failed to read input")
    }

    pub fn get_input_as_reader(day: u8) -> io::BufReader<File> {
        let file = get_file_from_day(day);
        io::BufReader::new(file)
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

    #[derive(Default)]
    pub struct SolutionHalf<S, D = &'static str>
    where
        S: Display,
        D: Display,
    {
        pub solution: Option<S>,
        pub description: Option<D>,
    }

    impl<S: Display, D: Display> fmt::Display for SolutionHalf<S, D> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if let Some(solution) = &self.solution {
                if let Some(description) = &self.description {
                    write!(f, "{}: {}", description, solution)
                } else {
                    write!(f, "{}", solution)
                }
            } else {
                write!(f, "(None yet)")
            }
        }
    }

    #[derive(Default)]
    pub struct Solution<S: Display, D: Display = &'static str>(pub [SolutionHalf<S, D>; 2]);

    impl<S: Display, D: Display> Solution<S, D> {
        pub fn new() -> Self {
            let first: SolutionHalf<S, D> = SolutionHalf {
                solution: None,
                description: None,
            };
            let second: SolutionHalf<S, D> = SolutionHalf {
                solution: None,
                description: None,
            };
            self::Solution([first, second])
        }

        pub fn print(&self) {
            println!("{}", self);
        }
    }

    impl<S: Display, D: Display> fmt::Display for Solution<S, D> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Part 1: {}\nPart 2: {}", self.0[0], self.0[1])
        }
    }

    impl<S: Display, D: Display> Deref for Solution<S, D> {
        type Target = [SolutionHalf<S, D>; 2];

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<S: Display, D: Display> DerefMut for Solution<S, D> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_solution_display() {
            let mut solution: Solution<u32> = Solution::default();
            solution[0].solution = Some(42);
            solution[0].description = Some("foo");
        }
    }
}

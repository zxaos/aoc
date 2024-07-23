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
    pub struct SolutionHalf<S: Display, D: AsRef<str>> {
        pub solution: Option<S>,
        pub description: Option<D>,
    }

    impl<S: Display, D: AsRef<str> + Display> fmt::Display for SolutionHalf<S, D> {
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
    pub struct Solution<S: Display, D: AsRef<str>>(pub [SolutionHalf<S, D>; 2]);

    impl<S: Display, D: AsRef<str> + Display> Solution<S, D> {
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

    impl<S: Display, D: AsRef<str> + Display> fmt::Display for Solution<S, D> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Part 1: {}\nPart 2: {}", self.0[0], self.0[1])
        }
    }

    impl<S: Display, D: AsRef<str>> Deref for Solution<S, D> {
        type Target = [SolutionHalf<S, D>; 2];

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<S: Display, D: AsRef<str>> DerefMut for Solution<S, D> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    //     pub struct Solution<S: Into<String>, T: Display> {
    //         first: SingleSolution<T>,
    //         second: SingleSolution<T>,
    //         puzzle: u8,
    //     }

    //     impl<S: Into<String>, T: Display> Solution<S, T> {
    //         pub fn new(puzzle: u8) -> Self {
    //             self::Solution {
    //                 first: SingleSolution::new(),
    //                 second: SingleSolution::new(),
    //                 puzzle,
    //             }
    //         }
    //     }

    //     impl<S: Into<String>, T: Display> Index<usize> for Solution<S, T> {
    //         type Output = SingleSolution<T>;

    //         fn index(&self, index: usize) -> &Self::Output {
    //             match index {
    //                 0 => &self.first,
    //                 1 => &self.second,
    //                 _ => panic!("Invalid index"),
    //             }
    //         }
    //     }

    //     impl<S: Into<String>, T: Display> IndexMut<usize> for Solution<S, T> {
    //         fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    //             match index {
    //                 0 => &mut self.first,
    //                 1 => &mut self.second,
    //                 _ => panic!("Invalid index"),
    //             }
    //         }
    //     }

    //     impl<S: Into<String>, T: Display> fmt::Display for Solution<S, T> {
    //         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //             write!(
    //                 f,
    //                 "Puzzle {}:\nPart 1: {}\nPart 2: {}",
    //                 self.puzzle, self.first, self.second
    //             )
    //         }
    //     }

    //     pub struct SingleSolution<T: Display> {
    //         solution: Option<T>,
    //         description: Option<String>,
    //     }

    //     impl<T: Display> fmt::Display for SingleSolution<T> {
    //         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //             if let Some(solution) = &self.solution {
    //                 if let Some(description) = &self.description {
    //                     write!(f, "{}: {}", description, solution)
    //                 } else {
    //                     write!(f, "{}", solution)
    //                 }
    //             } else {
    //                 write!(f, "(None yet)")
    //             }
    //         }
    //     }
    //     impl<T: Display> SingleSolution<T> {
    //         fn new() -> Self {
    //             SingleSolution {
    //                 solution: None,
    //                 description: None,
    //             }
    //         }
    //     }
}

#[cfg(test)]
mod test {
    use aoc_io::Solution;

    use super::*;

    #[test]
    fn test_solution_display() {
        let mut solution: Solution<u32, &str> = aoc_io::Solution::default();
        solution[0].solution = Some(42);
        solution[0].description = Some("foo");
    }
}

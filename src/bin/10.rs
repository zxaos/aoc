const CONWAY: f64 = 1.303577269034;

fn main() {
    let input = aoc_2015::aoc_io::get_input_as_string(10);
    let mut current = input;
    for _ in 0..40 {
        current = look_and_say(&current);
    }
    let first = current.len();
    for _ in 0..10 {
        current = look_and_say(&current);
    }
    let second = current.len();
    aoc_2015::aoc_io::put_aoc_output((Some(first), Some(second)));
}

fn look_and_say(look: &str) -> String {
    // The naive implementation (just iteratively building a new string every iteration)
    // is quite slow. We can speed it up by preallocating appropriate memory amounts, and
    // then only building the string once at the end.
    let nextlen: usize = ((look.len() as f64) * CONWAY).ceil() as usize;
    let mut offsets = Vec::with_capacity(nextlen / 2);
    let mut say = String::with_capacity(nextlen);

    let mut look_chars = look.chars().enumerate();
    let (_, mut this_char) = look_chars.next().expect("String cannot be empty");
    offsets.push(0);
    for (i, c) in look_chars {
        if c != this_char {
            offsets.push(i);
            this_char = c;
        }
    }
    offsets.push(look.len());

    for win in offsets.windows(2) {
        let start = win[0];
        let end = win[1];
        say.push_str(&(end - start).to_string());
        say.push_str(&look[start..start + 1]);
    }
    say
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn look_and_say_examples() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }
}

use itertools::peek_nth;

fn main() {
    let lines = aoc_2015::aoc_io::get_input_as_lines(8);
    let mut code_len = 0;
    let mut mem_len = 0;
    let mut esc_len = 0;
    for maybe_line in lines {
        if let Ok(line) = maybe_line {
            code_len += line.chars().count();
            mem_len += unescape_string(&line).chars().count();
            esc_len += quote_escape_string(&line).chars().count()
        } else {
            println!("Error: {}", maybe_line.unwrap_err())
        }
    }
    aoc_2015::aoc_io::put_aoc_output((Some(code_len - mem_len), Some(esc_len - code_len)));
}

fn quote_escape_string(source: &str) -> String {
    format!("\"{}\"", source.escape_default())
}

fn unescape_string(source: &str) -> String {
    let source = source.strip_prefix('"').unwrap_or(source);
    let source = source.strip_suffix('"').unwrap_or(source);
    let mut result = String::with_capacity(source.len());
    let mut chars = peek_nth(source.chars());
    while let Some(c) = chars.next() {
        let result_char = match c {
            '\\' => {
                let d = chars.peek();
                if let Some(d) = d {
                    match d {
                        '"' | '\'' | '\\' => {
                            let unescaped = *d; // make a copy so we can safely advance the iterator
                            chars.next();
                            unescaped
                        }
                        'x' => {
                            // d: x
                            // x1: maybe ascii hex
                            // x2: maybe ascii hex
                            let x1 = chars.peek_nth(1).cloned();
                            let x2 = chars.peek_nth(2).cloned();
                            let mut maybe_char = None;
                            let mut result = '\\';
                            if let (Some(x1), Some(x2)) = (x1, x2) {
                                if x1.is_ascii_hexdigit() && x2.is_ascii_hexdigit() {
                                    let hexchar = format!("{x1}{x2}");
                                    let as_hex = u8::from_str_radix(&hexchar, 16);
                                    if let Ok(converted) = as_hex {
                                        maybe_char = Some(char::from(converted));
                                    }
                                }
                            }
                            if let Some(decoded) = maybe_char {
                                chars.nth(2); // on success, discard the processed characters
                                result = decoded;
                            }
                            result
                        }
                        _ => '\\',
                    }
                } else {
                    '\\'
                }
            }
            _ => c,
        };
        result.push(result_char);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_strings() -> [(&'static str, usize, usize, usize); 6] {
        [
            //string, code length, unescaped length, double-escaped length
            (r#""""#, 2, 0, 6),
            (r#""abc""#, 5, 3, 9),
            (r#""aaa\"aaa""#, 10, 7, 16),
            (r#""\x27""#, 6, 1, 11),
            (r#""\\""#, 4, 1, 10),
            (r#""v\xfb\"lgs\"kvjfywmut\x9cr""#, 28, 18, 38),
        ]
    }

    #[test]
    fn test_basic_len() {
        let examples = test_strings();
        for (s, code_len, _, _) in examples {
            assert_eq!(
                s.chars().count(),
                code_len,
                "expected length {code_len} for escaped string {s}"
            )
        }
    }

    #[test]
    fn convert_to_str() {
        assert_eq!(u8::from_str_radix("A", 16), Ok(10));
        assert_eq!(u8::from_str_radix("10", 16), Ok(16));
        assert_eq!(u8::from_str_radix("27", 16), Ok(39));
        assert_eq!(b'\'', 39);
        let result = u8::from_str_radix("27", 16).unwrap();
        assert_eq!(result, b'\'');
    }

    #[test]
    fn test_unescaped_len() {
        let examples = test_strings();
        for (s, _, unescaped_len, _) in examples {
            let un = unescape_string(s);
            println!("escaped: {}", s);
            println!("unescaped: {}", un);
            assert_eq!(
                un.chars().count(),
                unescaped_len,
                "expected length {unescaped_len} for unescaped string {s}"
            )
        }
    }

    #[test]
    fn test_reescaped_len() {
        let examples = test_strings();
        for (s, _, _, reescaped_len) in examples {
            let re = quote_escape_string(s);
            println!("unescaped: {}", s);
            println!("reescaped: {}", re);
            assert_eq!(
                re.chars().count(),
                reescaped_len,
                "expected length {reescaped_len} for escaped string {re}"
            )
        }
    }
}

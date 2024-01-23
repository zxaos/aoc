use aoc_2015::aoc_io;
use md5;

fn main() {
    let input = aoc_io::get_input_as_string(4);

    let first_number = md5_leading_zeroes_from(&input, 5);
    let second_number = md5_leading_zeroes_from(&input, 6);

    aoc_io::put_aoc_output((Some(first_number), Some(second_number)));
}

fn md5_leading_zeroes_from(prefix: &str, num_zeroes: usize) -> u32 {
    let num_zeroes = num_zeroes;
    let num_zeroes = format!("{:0num_zeroes$}", 0);
    let mut count = 0;
    loop {
        let input = format!("{prefix}{}", count.to_string());
        let digest = format!("{:x}", md5::compute(input));
        if digest.starts_with(&num_zeroes) {
            return count;
        } else {
            count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    //If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043 starts with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
    fn d4_five_zeroes_abcdef() {
        let result = md5_leading_zeroes_from("abcdef", 5);
        assert_eq!(result, 609043);
    }
    //If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash starting with five zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970 looks like 000006136ef....
    #[test]
    fn d4_five_zeroes_pqrstuv() {
        let result = md5_leading_zeroes_from("pqrstuv", 5);
        assert_eq!(result, 1048970);
    }
}

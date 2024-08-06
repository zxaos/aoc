use aoc_2015::aoc_io;
use std::iter::zip;

fn main() {
    aoc_io::get_input_as_lines(15);
}

type Ingredient = [i32; 5];

fn ingredient_from_str(line: &str) -> Ingredient {
    // Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    // 0             1        2   3          4   5      6  7       8  9        10
    let mut parts = line.split(' ');
    let mut result: Ingredient = [0; 5];
    for (idx, &nth) in [2, 1, 1, 1, 1].iter().enumerate() {
        let val = parts
            .nth(nth)
            .map(|val| val.trim_end_matches(','))
            .and_then(|val| val.parse().ok())
            .unwrap_or_else(|| panic!("Failed to parse ingredient from string: {}", line));
        result[idx] = val;
    }

    result
}

fn score_cookie(ingredients: Vec<Ingredient>, counts: Vec<i32>) -> u32 {
    assert_eq!(
        ingredients.len(),
        counts.len(),
        "mismatched lengths for ingredients and amounts!"
    );
    let ingredient_totals: Vec<[i32; 4]> = zip(ingredients, counts)
        .map(|(i, c)| i.iter().take(4).map(|&s| s * c).collect())
        .collect();

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_string() {
        let line = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8";
        assert_eq!(ingredient_from_str(line), [-1, -2, 6, 3, 8]);
    }
    /*
    For instance, suppose you have these two ingredients:

    Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
    Then, choosing to use 44 teaspoons of butterscotch and 56 teaspoons of cinnamon (because the amounts of each ingredient must add up to 100) would result in a cookie with the following properties:

    A capacity of 44*-1 + 56*2 = 68
    A durability of 44*-2 + 56*3 = 80
    A flavor of 44*6 + 56*-2 = 152
    A texture of 44*3 + 56*-1 = 76
    Multiplying these together (68 * 80 * 152 * 76, ignoring calories for now) results in a total score of 62842880, which happens to be the best score possible given these ingredients. If any properties had produced a negative total, it would have instead become zero, causing the whole score to multiply to zero.
    */
    /*fn solve_basic(){
        let ingredient = vec![
            Ingredient[]
        ]
    }*/
}

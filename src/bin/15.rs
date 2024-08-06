use aoc_2015::aoc_io::{self, Solution};
use std::iter::zip;

const MAX_INGREDIENTS: usize = 100;
fn main() {
    let mut solution: Solution<u32, &str> = Solution::new();
    let ingredients = aoc_io::get_input_as_lines(15)
        .map(|l| Ingredient::from(l.expect("Couldn't read line").as_str()))
        .collect();
    let mut best_score = 0;
    let mut best_score_500 = 0;
    // Let's just know we have 4 ingredients and save some effort rather than doing this generically
    for a in 0..=MAX_INGREDIENTS {
        for b in 0..=(MAX_INGREDIENTS - a) {
            for c in 0..=(MAX_INGREDIENTS - (a + b)) {
                let d = MAX_INGREDIENTS - a - b - c;
                let counts = vec![a, b, c, d];
                let score = score_cookie(&ingredients, &counts);
                best_score = best_score.max(score);
                if cal_cookie(&ingredients, &counts) == 500 {
                    best_score_500 = best_score_500.max(score);
                }
            }
        }
    }
    solution[0].solution = Some(best_score);
    solution[1].solution = Some(best_score_500);

    solution.print();
}

#[derive(Debug, PartialEq)]
struct Ingredient {
    scores: [i32; 4],
    calories: u32,
}

impl From<&str> for Ingredient {
    // Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    // 0             1        2   3          4   5      6  7       8  9        10
    fn from(value: &str) -> Self {
        let mut parts = value.split(' ');
        let mut scores = [0; 5];
        for (idx, &nth) in [2, 1, 1, 1, 1].iter().enumerate() {
            let val = parts
                .nth(nth)
                .map(|val| val.trim_end_matches(','))
                .and_then(|val| val.parse().ok())
                .unwrap_or_else(|| panic!("Failed to parse ingredient from string: {}", value));
            scores[idx] = val;
        }
        Ingredient {
            scores: scores[0..4]
                .try_into()
                .expect("this will always have 4 elements"),
            calories: scores[4].try_into().expect("calories cannot be negative"),
        }
    }
}

fn cal_cookie(ingredients: &Vec<Ingredient>, counts: &Vec<usize>) -> u32 {
    assert_eq!(
        ingredients.len(),
        counts.len(),
        "mismatched lengths for ingredients and amounts!"
    );
    let counts: Vec<u32> = counts.iter().map(|&x| x as u32).collect();
    zip(ingredients, counts).map(|(i, c)| i.calories * c).sum()
}

fn score_cookie(ingredients: &Vec<Ingredient>, counts: &Vec<usize>) -> u32 {
    assert_eq!(
        ingredients.len(),
        counts.len(),
        "mismatched lengths for ingredients and amounts!"
    );
    let counts: Vec<i32> = counts.iter().map(|&x| x as i32).collect();
    let c_scores: Vec<[i32; 4]> = zip(ingredients, counts)
        .map(|(i, c)| i.scores.map(|s| s * c))
        .collect();
    let mut rest = c_scores.iter();
    let first = *rest.next().unwrap();
    let collected_scores = rest.fold(first, |sum, i| {
        [sum[0] + i[0], sum[1] + i[1], sum[2] + i[2], sum[3] + i[3]]
    });

    if collected_scores.iter().any(|&s| s < 1) {
        return 0;
    }

    collected_scores.iter().product::<i32>().try_into().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_string() {
        let line = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8";
        assert_eq!(
            Ingredient::from(line),
            Ingredient {
                scores: [-1, -2, 6, 3],
                calories: 8
            }
        )
    }

    #[test]
    fn test_score_cookie() {
        let bscotch = Ingredient {
            scores: [-1, -2, 6, 3],
            calories: 8,
        };
        let cinnamon = Ingredient {
            scores: [2, 3, -2, -1],
            calories: 3,
        };
        assert_eq!(
            score_cookie(&vec![bscotch, cinnamon], &vec![44, 56]),
            62_842_880
        );
    }

    #[test]
    fn test_cal_cookie() {
        let bscotch = Ingredient {
            scores: [-1, -2, 6, 3],
            calories: 8,
        };
        let cinnamon = Ingredient {
            scores: [2, 3, -2, -1],
            calories: 3,
        };
        assert_eq!(cal_cookie(&vec![bscotch, cinnamon], &vec![40, 60]), 500);
    }
}

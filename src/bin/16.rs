use std::ops::Index;

use aoc_2015::aoc_io::Solution;

enum Ops {
    Equal(u8),
    LessThan(u8),
    GreaterThan(u8),
}

type Criteria = [Ops; 10];

fn main() {
    let input = aoc_2015::aoc_io::get_input_as_lines(16);
    let mut solution: Solution<usize, &str> = Solution::new();
    let sues: [Sue; 500] = input
        .map(|line| Sue::from(line.expect("failed to read input").as_str()))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    // This is the other half of our input
    let part1_ops = [
        Ops::Equal(3), // children
        Ops::Equal(7), // cats
        Ops::Equal(2), // samoyeds
        Ops::Equal(3), // pomeranians
        Ops::Equal(0), // akitas
        Ops::Equal(0), // vizslas
        Ops::Equal(5), // goldfish
        Ops::Equal(3), // trees
        Ops::Equal(2), // cars
        Ops::Equal(1), // perfumes
    ];

    let part2_ops = [
        Ops::Equal(3),       // children
        Ops::GreaterThan(7), // cats
        Ops::Equal(2),       // samoyeds
        Ops::LessThan(3),    // pomeranians
        Ops::Equal(0),       // akitas
        Ops::Equal(0),       // vizslas
        Ops::LessThan(5),    // goldfish
        Ops::GreaterThan(3), // trees
        Ops::Equal(2),       // cars
        Ops::Equal(1),       // perfumes
    ];

    let candidates = Vec::from(sues.each_ref());
    solution[0].solution = find_sue(&candidates, &part1_ops);
    solution[1].solution = find_sue(&candidates, &part2_ops);
    solution.print();
}

fn find_sue(sues: &[&Sue], criteria: &Criteria) -> Option<usize> {
    let mut candidates = sues.to_vec();
    let mut test_attr = 0;

    while candidates.len() > 1 && test_attr < criteria.len() {
        let mut next_candidates = Vec::with_capacity(candidates.len());
        for &candidate in candidates.iter() {
            if let Some(attr) = candidate[test_attr] {
                match criteria[test_attr] {
                    // Add the candidate to the next round if they pass
                    Ops::Equal(c) if attr == c => next_candidates.push(candidate),
                    Ops::GreaterThan(c) if attr > c => next_candidates.push(candidate),
                    Ops::LessThan(c) if attr < c => next_candidates.push(candidate),
                    _ => continue,
                }
            } else {
                // No data for this attribute, so they continue to the next round
                next_candidates.push(candidate);
            }
        }
        test_attr += 1;
        candidates = next_candidates;
    }

    if candidates.len() == 1 {
        candidates[0].id
    } else {
        None
    }
}

#[derive(Debug, Default, PartialEq)]
struct Sue {
    id: Option<usize>,
    children: Option<u8>,
    cats: Option<u8>,
    samoyeds: Option<u8>,
    pomeranians: Option<u8>,
    akitas: Option<u8>,
    vizslas: Option<u8>,
    goldfish: Option<u8>,
    trees: Option<u8>,
    cars: Option<u8>,
    perfumes: Option<u8>,
}

impl Index<usize> for Sue {
    type Output = Option<u8>;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.children,
            1 => &self.cats,
            2 => &self.samoyeds,
            3 => &self.pomeranians,
            4 => &self.akitas,
            5 => &self.vizslas,
            6 => &self.goldfish,
            7 => &self.trees,
            8 => &self.cars,
            9 => &self.perfumes,
            _ => &None,
        }
    }
}

impl From<&str> for Sue {
    fn from(value: &str) -> Self {
        /* We can't parse in a single regex because the order isn't guaranteed
        So instead:
          * Extract "Sue #+: "
          * Then split reminader on comma
          * Then match and parse first word
        (And we'll just panic if we get invalid input)
        */
        assert!(value.starts_with("Sue "));

        let mut rest = &value[4..];
        let col = rest.find(':').unwrap();
        let sue_id: usize = rest[..col].parse().unwrap();
        let mut result: Sue = Sue {
            id: Some(sue_id),
            ..Default::default()
        };
        rest = &rest[col + 2..]; // jump to first attributes
        let attributes = rest.split(", ");
        for att in attributes {
            let mut inner = att.split(": ");
            let thing = inner.next().unwrap();
            let count = Some(inner.next().unwrap().parse().unwrap());
            match thing {
                "children" => result.children = count,
                "cats" => result.cats = count,
                "samoyeds" => result.samoyeds = count,
                "pomeranians" => result.pomeranians = count,
                "akitas" => result.akitas = count,
                "vizslas" => result.vizslas = count,
                "goldfish" => result.goldfish = count,
                "trees" => result.trees = count,
                "cars" => result.cars = count,
                "perfumes" => result.perfumes = count,
                unknown => println!("Ignoring unknown attribute on Sue {sue_id}: '{unknown}'"),
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_sue() {
        let s1 = "Sue 1: children: 1, cars: 8, vizslas: 7";
        let s2 = "Sue 2: akitas: 10, perfumes: 10, children: 5";
        assert_eq!(
            Sue::from(s1),
            Sue {
                id: Some(1),
                children: Some(1),
                cars: Some(8),
                vizslas: Some(7),
                ..Default::default()
            }
        );
        assert_eq!(
            Sue::from(s2),
            Sue {
                id: Some(2),
                akitas: Some(10),
                perfumes: Some(10),
                children: Some(5),
                ..Default::default()
            }
        )
    }
}

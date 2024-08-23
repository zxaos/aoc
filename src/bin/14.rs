use aoc_2015::aoc_io::Solution;

#[derive(PartialEq, Debug)]
struct Reindeer {
    speed: u64,
    timeout: u64,
    period: u64,
}

impl Reindeer {
    pub fn new(speed: u64, timeout: u64, cooldown: u64) -> Self {
        Reindeer {
            speed,
            timeout,
            period: timeout + cooldown,
        }
    }

    pub fn travel(&self, seconds: u64) -> u64 {
        let partial = seconds % self.period;
        let distance = if partial < self.timeout {
            partial * self.speed
        } else {
            self.timeout * self.speed
        };
        let full_cycles = seconds / self.period;
        distance + (self.speed * self.timeout * full_cycles)
    }
}

#[allow(clippy::ptr_arg)]
fn max_indices(xs: &Vec<u64>) -> Vec<usize> {
    if xs.is_empty() {
        return vec![];
    }
    let mut max = xs[0];
    let mut maxidx = vec![0];

    for (idx, &val) in xs.iter().enumerate().skip(1) {
        #[allow(clippy::comparison_chain)]
        if val > max {
            max = val;
            maxidx.clear();
            maxidx.push(idx);
        } else if val == max {
            maxidx.push(idx);
        }
    }
    maxidx
}

fn main() {
    let input = aoc_2015::aoc_io::get_input_as_lines(14);
    let reindeer: Vec<_> = input
        .map(|l| parseline(&l.expect("Couldn't get input line")))
        .map(|(speed, timeout, cooldown)| Reindeer::new(speed, timeout, cooldown))
        .collect();
    let after_secs = reindeer.iter().map(|r| r.travel(2503));
    let mut solution: Solution<u64> = Solution::new();
    solution[0].solution = after_secs.max();
    solution[0].description = Some("Furthest distance after 2503 seconds");

    let mut scores: Vec<u64> = vec![0; reindeer.len()];
    let mut current_winners: Vec<usize>;
    for sec in 1..2504 {
        current_winners = max_indices(&(reindeer.iter().map(|r| r.travel(sec)).collect()));
        for &w in current_winners.iter() {
            scores[w] += 1
        }
        current_winners.clear();
    }
    solution[1].solution = scores.iter().max().copied();
    solution[1].description = Some("Best score after 2504 seconds");

    solution.print();
}

fn parseline(line: &str) -> (u64, u64, u64) {
    // Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
    // 0     1   2   3  4    5   6  7        8   9    10   11   12  13  14
    // but don't forget consuming the iterator shifts everything
    let mut parts = line.split(' ');
    let _name = parts.next().expect("Bad format: Name");
    let speed = parts
        .nth(2)
        .and_then(|diststring| diststring.parse::<u64>().ok())
        .expect("Bad format: speed");
    let limit = parts
        .nth(2)
        .and_then(|diststring| diststring.parse::<u64>().ok())
        .expect("Bad format: limit");
    let cooldown = parts
        .nth(6)
        .and_then(|diststring| diststring.parse::<u64>().ok())
        .expect("Bad format: cooldown");
    (speed, limit, cooldown)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_travel_comet() {
        let reindeer = Reindeer::new(14, 10, 127);
        assert_eq!(reindeer.travel(1), 14);
        assert_eq!(reindeer.travel(10), 140);
        assert_eq!(reindeer.travel(11), 140);
        assert_eq!(reindeer.travel(1000), 1120);
    }

    #[test]
    fn test_travel_dancer() {
        let reindeer = Reindeer::new(16, 11, 162);
        assert_eq!(reindeer.travel(1), 16);
        assert_eq!(reindeer.travel(10), 160);
        assert_eq!(reindeer.travel(11), 176);
        assert_eq!(reindeer.travel(1000), 1056);
    }

    #[test]
    fn test_parseline() {
        let line = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";
        assert_eq!(parseline(line), (14, 10, 127));
    }
}

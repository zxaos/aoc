use aoc_2015::aoc_io::Solution;
use itertools::Itertools;
use miette::Result;

const WEAPONS: [Item; 5] = [
    Item {
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        cost: 74,
        damage: 8,
        armor: 0,
    },
];

const ARMOR: [Item; 5] = [
    Item {
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        cost: 102,
        damage: 0,
        armor: 5,
    },
];

const RINGS: [Item; 6] = [
    Item {
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

fn main() -> Result<()> {
    let mut solution: Solution<u32> = Solution::new();
    let player_template = Actor {
        health: 100,
        damage: 0,
        armor: 0,
        items: vec![],
    };

    // iterate over the lines and grab the number at the end of each.
    // assume they are in the order health, damage, armor, and load them into
    // the boss template.
    let mut raw_input = aoc_2015::aoc_io::get_input_as_lines(21);
    let mut input_numbers = [0; 3];
    for val in input_numbers.iter_mut() {
        let in_number = raw_input
            .next()
            .unwrap()
            .unwrap()
            .split(':')
            .last()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        *val = in_number;
    }

    let boss_template = Actor {
        health: input_numbers[0],
        damage: input_numbers[1],
        armor: input_numbers[2],
        items: vec![],
    };

    let player_scenarios: Vec<(CombatResult, Actor)> = generate_scenarios(&player_template)
        .into_iter()
        .map(|mut p| (p.combat(&mut boss_template.clone()), p))
        .collect();

    let lowest_win = player_scenarios
        .iter()
        .filter(|(result, _)| matches!(result, CombatResult::Win))
        .map(|(_, p)| p.item_cost())
        .min();

    solution[0].solution = lowest_win;
    solution[0].description = Some("Lowest amount payable to win");

    let highest_loss = player_scenarios
        .iter()
        .filter(|(result, _)| matches!(result, CombatResult::Loss))
        .map(|(_, p)| p.item_cost())
        .max();

    solution[1].solution = highest_loss;
    solution[1].description = Some("Highest amount payable to lose");

    solution.print();

    Ok(())
}

#[derive(Debug, PartialEq)]
enum CombatResult {
    Win,
    Loss,
}

#[derive(Debug, Clone)]
struct Actor<'a> {
    health: i32,
    damage: i32,
    armor: i32,
    items: Vec<&'a Item>,
}

#[derive(Debug, Clone)]
struct Item {
    cost: u32,
    damage: i32,
    armor: i32,
}

impl<'a> Actor<'a> {
    pub fn clone_empty(&self) -> Self {
        Actor {
            health: self.health,
            damage: self.damage,
            armor: self.armor,
            items: vec![],
        }
    }

    /// damage, armor bonuses
    fn item_bonus(&self) -> (i32, i32) {
        self.items
            .iter()
            .fold((0, 0), |(d, a), item| (d + item.damage, a + item.armor))
    }

    fn item_cost(&self) -> u32 {
        self.items.iter().map(|i| i.cost).sum()
    }

    pub fn combat(&mut self, opponent: &mut Actor) -> CombatResult {
        let (self_d_mod, self_a_mod) = self.item_bonus();
        let (opponent_d_mod, opponent_a_mod) = opponent.item_bonus();
        let self_damage = self.damage + self_d_mod - opponent_a_mod - opponent.armor;
        let opp_damage = opponent.damage + opponent_d_mod - self_a_mod - self.armor;

        self.combat_round(opponent, self_damage, opp_damage, 1)
    }

    fn combat_round(
        &mut self,
        opponent: &mut Actor,
        self_damage: i32,
        opp_damage: i32,
        round: usize,
    ) -> CombatResult {
        if round % 2 == 1 {
            opponent.health -= self_damage;
        } else {
            self.health -= opp_damage;
        }

        if opponent.health < 1 {
            CombatResult::Win
        } else if self.health < 1 {
            CombatResult::Loss
        } else {
            self.combat_round(opponent, self_damage, opp_damage, round + 1)
        }
    }
}

fn vec_wrap<T>(x: T) -> Vec<T> {
    vec![x]
}

fn generate_scenarios<'items>(player: &Actor<'items>) -> Vec<Actor<'items>> {
    type ItemSet<'a> = Vec<Vec<&'a Item>>;
    // You must buy exactly one weapon
    // 0 - 1 Armor,
    // 0 - 2 rings, no duplicates
    // Wrap the outputs in a Vec so that we can represent choosing 0 (armor)
    // and so that we have a consistent type with combinations for rings
    let weapons_t: ItemSet = WEAPONS.iter().map(vec_wrap).collect();
    let armor_t = ARMOR.iter().map(vec_wrap).chain(vec![]).collect();
    let rings_t = RINGS
        .iter()
        .combinations(2)
        .chain(RINGS.iter().map(vec_wrap)) // effectively, combinations(1)
        .chain([vec![]])
        .collect();

    [weapons_t, armor_t, rings_t]
        .into_iter()
        .multi_cartesian_product()
        .map(|items| {
            // turn each vec of item selections into a player
            let mut p = player.clone_empty();
            p.items = items.into_iter().flatten().collect();
            p
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_combat_step() {
        let d = Item {
            cost: 0,
            damage: 5,
            armor: 0,
        };
        let a = Item {
            cost: 0,
            damage: 0,
            armor: 5,
        };
        let mut player = Actor {
            health: 8,
            damage: 0,
            armor: 0,
            items: vec![&d, &a],
        };
        let mut boss = Actor {
            health: 12,
            damage: 7,
            armor: 2,
            items: vec![],
        };

        assert_eq!(player.combat(&mut boss), CombatResult::Win);
    }
}

/* What is needed?

- [x] Map a,b,...zz to indices
- [x] Nope, just use a HashMap.
- [x] Create a enum of instruction types
- [x] Parse instructions into a collection of enums
- [x] (No, use a hashmap) Count the number of outputs and create a
- [x] (No, use a hashmap) Maintain a list of solved and unsolved indices
- [ ] Iterate over the instructions. For each unsolved instruction:
    - [ ] Are the prerequisites solved?
    - [ ] Yes: Solve this and add it to the solved list
    - [ ] No: Skip it and come back later
*/

use std::{collections::HashMap, ops::Not, str::FromStr};
use thiserror::Error;

fn main() -> Result<(), CircuitError> {
    let lines =
        aoc_2015::aoc_io::get_input_as_lines(7).collect::<Result<Vec<String>, std::io::Error>>()?;
    let circuit_ops = CircuitOps::from_lines(lines.iter().map(String::as_str))?;
    let mut circuit = Circuit::new(circuit_ops.len());
    circuit.resolve_circuit(circuit_ops)?;
    let result_1 = circuit.get_wire("a").unwrap();

    aoc_2015::aoc_io::put_aoc_named_output((Some(result_1), None), "Wire a", "");

    Ok(())
}

#[derive(Debug, PartialEq, Clone)]
enum CircuitOps {
    // Operation(destination, operands...)
    Set(usize, u16),
    And(usize, usize, usize),
    AndFixed(usize, usize, u16),
    Or(usize, usize, usize),
    OrFixed(usize, usize, u16),
    Not(usize, usize),
    Lshift(usize, usize, u16),
    Rshift(usize, usize, u16),
}

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
enum CircuitError {
    #[error("couldn't parse int")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("couldn't convert {} into a valid instruction", .0)]
    ParseInstructionError(String),
    #[error("couldn't read input file")]
    IOError(#[from] std::io::Error),
    #[error("wires cannot be redefined")]
    WireInUseError(),
    #[error("dependencies not met")]
    UnmetDepsError(),
    #[error("circuit is unresolvable")]
    UnresolvableInstructionsError(),
}

impl FromStr for CircuitOps {
    type Err = CircuitError;
    fn from_str(s: &str) -> Result<Self, CircuitError> {
        use CircuitError::*;
        use CircuitOps::*;
        let (operator, target, operands) =
            Self::tokenize(s).ok_or(ParseInstructionError(s.to_string()))?;

        let op: Option<CircuitOps> = match operator {
            "SET" => operands[0]
                .parse::<u16>()
                .ok()
                .map(|value| Set(target, value)),
            "NOT" => str_idx(operands[0]).map(|value| Not(target, value)),
            "AND" => {
                // operands might be "ab" or they might be "123"
                let wire_operands: Option<Vec<usize>> =
                    operands.iter().map(|op| str_idx(op)).collect();
                let numeric_operands: Option<Vec<u16>> =
                    operands.iter().map(|op| op.parse::<u16>().ok()).collect();
                operands.map(|o| And(target, o[0], o[1]))
            }
            "OR" => {
                let operands: Option<Vec<usize>> = operands.iter().map(|op| str_idx(op)).collect();
                operands.map(|o| Or(target, o[0], o[1]))
            }
            "LSHIFT" => {
                let oper_a = str_idx(operands[0]);
                let oper_b = operands[1].parse::<u16>().ok();
                if let (Some(oper_a), Some(oper_b)) = (oper_a, oper_b) {
                    Some(Lshift(target, oper_a, oper_b))
                } else {
                    None
                }
            }
            "RSHIFT" => {
                let oper_a = str_idx(operands[0]);
                let oper_b = operands[1].parse::<u16>().ok();
                if let (Some(oper_a), Some(oper_b)) = (oper_a, oper_b) {
                    Some(Rshift(target, oper_a, oper_b))
                } else {
                    None
                }
            }
            _ => None,
        };
        op.ok_or(CircuitError::ParseInstructionError(s.to_string()))
    }
}

impl CircuitOps {
    fn tokenize(line: &str) -> Option<(&str, usize, Vec<&str>)> {
        // we want this order:
        // operation, destination, operands
        let mut words: Vec<&str> = line.split(' ').collect();
        if words.len() < 3 || words.len() > 5 {
            return None;
        }

        let destination = words.pop().and_then(str_idx)?;
        if words.pop().unwrap() != "->" {
            return None;
        }

        let mut operands: Vec<&str> = Vec::with_capacity(words.len());

        operands.push(words.pop().unwrap());
        if words.is_empty() {
            return Some(("SET", destination, operands));
        }

        if words.len() == 1 && words.pop().unwrap() == "NOT" {
            return Some(("NOT", destination, operands));
        }

        if words.len() == 2 {
            let operator = words.pop().unwrap();
            if !["AND", "OR", "LSHIFT", "RSHIFT"].contains(&operator) {
                return None;
            }
            operands.insert(0, words.pop().unwrap());
            return Some((operator, destination, operands));
        }

        None
    }

    pub fn from_lines<'a>(lines: impl Iterator<Item = &'a str>) -> Result<Vec<Self>, CircuitError> {
        lines.map(CircuitOps::from_str).collect()
    }

    pub fn get_target(&self) -> &usize {
        use CircuitOps::*;
        // there has to be a better way :-(
        match self {
            Set(target, _)
            | And(target, _, _)
            | Or(target, _, _)
            | Not(target, _)
            | Lshift(target, _, _)
            | Rshift(target, _, _) => target,
        }
    }

    pub fn get_depencies(&self) -> Vec<&usize> {
        use CircuitOps::*;
        // there has to be a better way :-(
        match self {
            Set(_, _) => vec![],
            Not(_, a) | Lshift(_, a, _) | Rshift(_, a, _) => vec![a],
            And(_, a, b) | Or(_, a, b) => vec![a, b],
        }
    }
}

// We could probably speed this up by using a faster hasher.
#[derive(Debug)]
struct Circuit {
    wires: HashMap<usize, u16>,
}

impl Circuit {
    pub fn new(size: usize) -> Circuit {
        Circuit {
            wires: HashMap::with_capacity(size),
        }
    }

    pub fn get_wire(&self, wire: &str) -> Option<u16> {
        self.wires.get(&str_idx(wire)?).cloned()
    }

    fn deps_ok(&self, op: &CircuitOps) -> bool {
        op.get_depencies()
            .iter()
            .map(|dep| self.wires.contains_key(dep))
            .all(|f| f)
    }

    fn run_instruction(&mut self, op: &CircuitOps) -> Result<(), CircuitError> {
        use CircuitOps::*;
        let target = op.get_target();
        if self.wires.contains_key(target) {
            return Err(CircuitError::WireInUseError());
        }
        if !self.deps_ok(op) {
            return Err(CircuitError::UnmetDepsError());
        }
        match op {
            Set(_, v) => self.wires.insert(*target, *v),
            Not(_, oper_a) => self
                .wires
                .insert(*target, self.wires.get(oper_a).unwrap().not()),
            And(_, oper_a, oper_b) => {
                let oper_a = self.wires.get(oper_a).unwrap();
                let oper_b = self.wires.get(oper_b).unwrap();
                self.wires.insert(*target, oper_a & oper_b)
            }
            Or(_, oper_a, oper_b) => {
                let oper_a = self.wires.get(oper_a).unwrap();
                let oper_b = self.wires.get(oper_b).unwrap();
                self.wires.insert(*target, oper_a | oper_b)
            }
            Lshift(_, oper_a, amount) => {
                let oper_a = self.wires.get(oper_a).unwrap();
                self.wires.insert(*target, oper_a << amount)
            }
            Rshift(_, oper_a, amount) => {
                let oper_a = self.wires.get(oper_a).unwrap();
                self.wires.insert(*target, oper_a >> amount)
            }
        };
        Ok(())
    }

    fn resolve_circuit(&mut self, ops: Vec<CircuitOps>) -> Result<(), CircuitError> {
        // There's probably lots of performance badness here.
        let mut removals: Vec<usize> = Vec::new();
        let mut remaining = ops.clone();
        while !remaining.is_empty() {
            for (i, op) in remaining.iter().enumerate() {
                match self.run_instruction(op) {
                    Ok(_) => removals.push(i),
                    Err(CircuitError::UnmetDepsError()) => {
                        continue;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            if removals.is_empty() {
                return Err(CircuitError::UnresolvableInstructionsError());
            }
            let mut removal_idx: usize = 0;
            remaining.retain(|_| {
                let keep = !removals.contains(&removal_idx);
                removal_idx += 1;
                keep
            });
            removals.clear();
        }
        Ok(())
    }
}

const A_OFFSET: u8 = b'a';
const A_BASE: usize = 26;
const A_RANGE: std::ops::RangeInclusive<u8> = b'a'..=b'z';
fn str_idx(alpha_idx: &str) -> Option<usize> {
    if alpha_idx.is_empty() {
        return None;
    }

    let mut total: usize = 0;
    // iterate the string from smallest to largest (so reverse it)
    // multiply each by A_BASE^place
    for (place, c) in alpha_idx.chars().rev().enumerate() {
        // count from 1 or the math doesn't work when multiplying by place
        let c = c as u8;
        if !A_RANGE.contains(&c) {
            return None;
        }
        let c = (c - A_OFFSET + 1) as usize;

        total = A_BASE
            .checked_pow(place as u32)
            .and_then(|pow| c.checked_mul(pow))
            .and_then(|add| total.checked_add(add))
            .expect("Range is too high for this platform!");
    }

    Some(total - 1) // correct to start at zero
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn str_idx_examples() {
        let letters = [
            ("a", 0),
            ("b", 1),
            ("z", 25),
            ("aa", 26),
            ("ab", 27),
            ("ba", 52),
        ];

        for (letter, num) in letters {
            assert_eq!(
                str_idx(letter).unwrap(),
                num,
                "Input string was \"{letter}\""
            );
        }
    }

    #[test]
    fn str_idx_none_from_garbage() {
        let bad = ["", "b-", "z12", "a-a", "🙃"];

        for s in bad {
            assert_eq!(str_idx(s), None, "Input string was \"{s}\"");
        }
    }

    #[test]
    #[should_panic(expected = "too high")]
    fn str_idx_too_big() {
        str_idx("zzzzzzzzzzzzzz");
    }

    #[test]
    fn operator_tokenize_examples() {
        let examples = [
            ("123 -> x", CircuitOps::Set(23, 123)),
            ("456 -> y", CircuitOps::Set(24, 456)),
            ("x AND y -> d", CircuitOps::And(3, 23, 24)),
            ("x OR y -> e", CircuitOps::Or(4, 23, 24)),
            ("x LSHIFT 2 -> f", CircuitOps::Lshift(5, 23, 2)),
            ("y RSHIFT 2 -> g", CircuitOps::Rshift(6, 24, 2)),
            ("NOT x -> h", CircuitOps::Not(7, 23)),
            ("NOT y -> i", CircuitOps::Not(8, 24)),
        ];
        for (s, op) in examples {
            assert_eq!(CircuitOps::from_str(s).unwrap(), op, "enum from {s}")
        }
    }

    #[test]
    fn set_wire() -> Result<(), CircuitError> {
        let mut circ = Circuit::new(1);
        let w_a = str_idx("a").unwrap();
        circ.run_instruction(&CircuitOps::Set(w_a, 123))?;
        assert_eq!(circ.get_wire("a").unwrap(), 123);
        Ok(())
    }

    #[test]
    fn or_wire() -> Result<(), CircuitError> {
        let mut circ = Circuit::new(3);
        let w_a = str_idx("a").unwrap();
        let w_b = str_idx("b").unwrap();
        let w_c = str_idx("c").unwrap();
        circ.run_instruction(&CircuitOps::Set(w_a, 1))?;
        circ.run_instruction(&CircuitOps::Set(w_b, 2))?;
        circ.run_instruction(&CircuitOps::Or(w_c, w_a, w_b))?;
        assert_eq!(circ.get_wire("c").unwrap(), 3);
        Ok(())
    }

    #[test]
    fn and_wire() -> Result<(), CircuitError> {
        let mut circ = Circuit::new(3);
        let w_a = str_idx("a").unwrap();
        let w_b = str_idx("b").unwrap();
        let w_c = str_idx("c").unwrap();
        circ.run_instruction(&CircuitOps::Set(w_a, 3))?;
        circ.run_instruction(&CircuitOps::Set(w_b, 5))?;
        circ.run_instruction(&CircuitOps::And(w_c, w_a, w_b))?;
        assert_eq!(circ.get_wire("c").unwrap(), 1);
        Ok(())
    }

    #[test]
    fn example_circuit_inorder() -> Result<(), CircuitError> {
        let instr_str = [
            "123 -> x",
            "456 -> y",
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "NOT x -> h",
            "NOT y -> i",
        ];
        let results = [
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ];
        let ops = CircuitOps::from_lines(instr_str.into_iter())?;
        let mut circ = Circuit::new(ops.len());
        circ.resolve_circuit(ops)?;
        for (wire, value) in results {
            assert_eq!(circ.get_wire(wire).unwrap(), value);
        }

        Ok(())
    }

    #[test]
    fn example_circuit_out_of_order() -> Result<(), CircuitError> {
        let instr_str = [
            "x LSHIFT 2 -> f",
            "NOT y -> i",
            "456 -> y",
            "x OR y -> e",
            "y RSHIFT 2 -> g",
            "x AND y -> d",
            "NOT x -> h",
            "123 -> x",
        ];
        let results = [
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ];
        let ops = CircuitOps::from_lines(instr_str.into_iter())?;
        let mut circ = Circuit::new(ops.len());
        circ.resolve_circuit(ops)?;
        for (wire, value) in results {
            assert_eq!(circ.get_wire(wire).unwrap(), value);
        }

        Ok(())
    }
}

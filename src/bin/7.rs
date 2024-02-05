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
    let circuit_ops_a = CircuitOp::from_lines(lines.iter().map(String::as_str))?;
    let mut circuit_ops_b = circuit_ops_a.clone();
    let mut circuit_a = Circuit::new(circuit_ops_a.len());
    circuit_a.resolve_circuit(circuit_ops_a)?;
    let result_a = circuit_a.get_wire("a").unwrap();

    // Now, take the signal you got on wire a, override wire b to that signal, and reset the other wires
    //  (including wire a). What new signal is ultimately provided to wire a?
    let b_wire_idx = str_idx("b").unwrap();
    let b_pos = circuit_ops_b
        .iter()
        .position(|op| op.target == b_wire_idx)
        .expect("wire b must be defined");
    circuit_ops_b[b_pos] = CircuitOp {
        target: b_wire_idx,
        op: CircuitOpType::Set,
        operand_a: CircuitOperand::Value(result_a),
        operand_b: None,
    };

    let mut circuit_b = Circuit::new(circuit_ops_b.len());
    circuit_b.resolve_circuit(circuit_ops_b)?;
    let result_2 = circuit_b.get_wire("a").unwrap();

    aoc_2015::aoc_io::put_aoc_named_output(
        (Some(result_a), Some(result_2)),
        "Wire a",
        "Wire a after second iteration",
    );

    Ok(())
}

#[derive(Debug, PartialEq, Clone)]
struct CircuitOp {
    target: usize,
    op: CircuitOpType,
    operand_a: CircuitOperand,
    operand_b: Option<CircuitOperand>,
}

#[derive(Debug, PartialEq, Clone)]
enum CircuitOperand {
    Value(u16),
    Wire(usize),
}

impl CircuitOperand {
    pub fn resolve_memo(&mut self, c: &Circuit) -> Option<u16> {
        use self::CircuitOperand::*;
        if let Value(result) = self {
            return Some(*result);
        }

        if let Wire(wire_id) = self {
            if let Some(wire_value) = c.get_wire_by_id(wire_id) {
                let mut newself = Value(wire_value);
                std::mem::swap(&mut newself, self);
                Some(wire_value)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Return the resolved value of an operand, or None if the wire has no output
    pub fn resolve(&self, c: &Circuit) -> Option<u16> {
        use self::CircuitOperand::*;
        match self {
            Value(val) => Some(*val),
            Wire(id) => c.get_wire_by_id(id),
        }
    }

    pub fn build_from_str(s: &str) -> Option<CircuitOperand> {
        if let Ok(fixed_number) = s.parse::<u16>() {
            Some(CircuitOperand::Value(fixed_number))
        } else {
            Some(CircuitOperand::Wire(str_idx(s)?))
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum CircuitOpType {
    Set,
    And,
    Or,
    Not,
    Lshift,
    Rshift,
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

impl FromStr for CircuitOp {
    type Err = CircuitError;
    fn from_str(s: &str) -> Result<Self, CircuitError> {
        use CircuitError::*;
        use CircuitOpType::*;
        let (operator, target, operands) =
            Self::tokenize(s).ok_or(ParseInstructionError(s.to_string()))?;

        if operands.len() != 1 && operands.len() != 2 {
            return Err(ParseInstructionError(
                format!(
                    "wrong number of operands for input: {s} -- (Got {}, expected 1 or 2",
                    operands.len()
                )
                .to_string(),
            ));
        }

        let operand_a = CircuitOperand::build_from_str(operands[0]).ok_or(
            CircuitError::ParseInstructionError(("couldn't parse mandatory operand").to_string()),
        )?;

        let operand_b = operands
            .get(1)
            .and_then(|op| CircuitOperand::build_from_str(op));

        let op: Option<CircuitOpType> = match operator {
            "SET" => Some(Set),
            "NOT" => Some(Not),
            "AND" => Some(And),
            "OR" => Some(Or),
            "LSHIFT" => Some(Lshift),
            "RSHIFT" => Some(Rshift),
            _ => None,
        };

        let op = op.ok_or(CircuitError::ParseInstructionError(s.to_string()))?;

        // assert that we have a second operand for ops that require it
        match op {
            And | Or | Lshift | Rshift => {
                if operand_b.is_none() {
                    return Err(CircuitError::ParseInstructionError(
                        format!("operation {operator} requires a second operand").to_string(),
                    ));
                }
            }
            _ => (),
        }

        Ok(CircuitOp {
            target,
            op,
            operand_a,
            operand_b,
        })
    }
}

impl CircuitOp {
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
        lines.map(CircuitOp::from_str).collect()
    }

    // TODO: Remove this and just access it directly
    pub fn get_target(&self) -> &usize {
        &(self.target)
    }

    pub fn resolve_operands(&self, with: &Circuit) -> Option<Vec<u16>> {
        use CircuitOpType::*;
        let a = self.operand_a.resolve(with);
        // TODO: remove the clone?
        let b = self.operand_b.clone().and_then(|b| b.resolve(with));
        a?;

        match self.op {
            Set | Not => Some(vec![a.unwrap()]),
            And | Or | Lshift | Rshift if b.is_some() => Some(vec![a.unwrap(), b.unwrap()]),
            _ => None,
        }
    }

    /*/    pub fn get_depencies(&self) -> Vec<&usize> {
    use CircuitOpType::*;
    // there has to be a better way :-(
    let deps => vec![self.operand_a];
    if let Some(operand_b) = self.operand_b {
        deps.push(operand_b)
    }
    match self {
        Set(_, _) => vec![],
        Not(_, a) | Lshift(_, a, _) | Rshift(_, a, _) => vec![a],
        And(_, a, b) | Or(_, a, b) => vec![a, b],
    }
    }*/
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
        self.get_wire_by_id(&str_idx(wire)?)
    }

    pub fn get_wire_by_id(&self, wire: &usize) -> Option<u16> {
        self.wires.get(wire).cloned()
    }

    fn run_instruction(&mut self, op: &CircuitOp) -> Result<(), CircuitError> {
        use CircuitOpType::*;
        let target = op.get_target();
        if self.wires.contains_key(target) {
            return Err(CircuitError::WireInUseError());
        }

        let operands = op
            .resolve_operands(self)
            .ok_or(CircuitError::UnmetDepsError())?;

        // By this point we're asserting:
        // * the operation has the correct number of operands (or it wouldn't build)
        // * the operands are resolvable and resolved (we just did this)
        // * so we can just assume everything is working and in place and check nothing.
        match op.op {
            Set => self.wires.insert(*target, operands[0]),
            Not => self.wires.insert(*target, operands[0].not()),
            And => self.wires.insert(*target, operands[0] & operands[1]),
            Or => self.wires.insert(*target, operands[0] | operands[1]),
            Lshift => self.wires.insert(*target, operands[0] << operands[1]),
            Rshift => self.wires.insert(*target, operands[0] >> operands[1]),
        };
        Ok(())
    }

    fn resolve_circuit(&mut self, ops: Vec<CircuitOp>) -> Result<(), CircuitError> {
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

    fn circuit_example() -> [(&'static str, CircuitOp); 8] {
        use self::CircuitOpType::*;
        use self::CircuitOperand::*;
        [
            (
                "123 -> x",
                CircuitOp {
                    target: 23,
                    op: Set,
                    operand_a: Value(123),
                    operand_b: None,
                },
            ),
            (
                "456 -> y",
                CircuitOp {
                    target: 24,
                    op: Set,
                    operand_a: Value(456),
                    operand_b: None,
                },
            ),
            (
                "x AND y -> d",
                CircuitOp {
                    target: 3,
                    op: And,
                    operand_a: Wire(23),
                    operand_b: Some(Wire(24)),
                },
            ),
            (
                "x OR y -> e",
                CircuitOp {
                    target: 4,
                    op: Or,
                    operand_a: Wire(23),
                    operand_b: Some(Wire(24)),
                },
            ),
            (
                "x LSHIFT 2 -> f",
                CircuitOp {
                    target: 5,
                    op: Lshift,
                    operand_a: Wire(23),
                    operand_b: Some(Value(2)),
                },
            ),
            (
                "y RSHIFT 2 -> g",
                CircuitOp {
                    target: 6,
                    op: Rshift,
                    operand_a: Wire(24),
                    operand_b: Some(Value(2)),
                },
            ),
            (
                "NOT x -> h",
                CircuitOp {
                    target: 7,
                    op: Not,
                    operand_a: Wire(23),
                    operand_b: None,
                },
            ),
            (
                "NOT y -> i",
                CircuitOp {
                    target: 8,
                    op: Not,
                    operand_a: Wire(24),
                    operand_b: None,
                },
            ),
        ]
    }

    fn circuit_example_results() -> [(&'static str, u16); 8] {
        [
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ]
    }

    #[test]
    fn operator_tokenize_examples() {
        let examples = circuit_example();
        for (s, op) in examples {
            assert_eq!(CircuitOp::from_str(s).unwrap(), op, "enum from {s}")
        }
    }

    #[test]
    fn set_wire() -> Result<(), CircuitError> {
        let mut circ = Circuit::new(1);
        let w_a = str_idx("a").unwrap();
        let set = CircuitOp {
            target: w_a,
            op: CircuitOpType::Set,
            operand_a: CircuitOperand::Value(123),
            operand_b: None,
        };
        circ.run_instruction(&set)?;
        assert_eq!(circ.get_wire("a").unwrap(), 123);
        Ok(())
    }

    #[test]
    fn or_wire() -> Result<(), CircuitError> {
        let mut circ = Circuit::new(3);
        let w_a = str_idx("a").unwrap();
        let w_b = str_idx("b").unwrap();
        let w_c = str_idx("c").unwrap();
        let set_a = CircuitOp {
            target: w_a,
            op: CircuitOpType::Set,
            operand_a: CircuitOperand::Value(1),
            operand_b: None,
        };

        let set_b = CircuitOp {
            target: w_b,
            op: CircuitOpType::Set,
            operand_a: CircuitOperand::Value(2),
            operand_b: None,
        };
        let or_c = CircuitOp {
            target: w_c,
            op: CircuitOpType::Or,
            operand_a: CircuitOperand::Wire(w_a),
            operand_b: Some(CircuitOperand::Wire(w_b)),
        };

        circ.run_instruction(&set_a)?;
        circ.run_instruction(&set_b)?;
        circ.run_instruction(&or_c)?;
        assert_eq!(circ.get_wire("c").unwrap(), 3);
        Ok(())
    }

    #[test]
    fn and_wire() -> Result<(), CircuitError> {
        let mut circ = Circuit::new(3);
        let w_a = str_idx("a").unwrap();
        let w_b = str_idx("b").unwrap();
        let w_c = str_idx("c").unwrap();
        let set_a = CircuitOp {
            target: w_a,
            op: CircuitOpType::Set,
            operand_a: CircuitOperand::Value(3),
            operand_b: None,
        };

        let set_b = CircuitOp {
            target: w_b,
            op: CircuitOpType::Set,
            operand_a: CircuitOperand::Value(5),
            operand_b: None,
        };
        let and_c = CircuitOp {
            target: w_c,
            op: CircuitOpType::And,
            operand_a: CircuitOperand::Wire(w_a),
            operand_b: Some(CircuitOperand::Wire(w_b)),
        };

        circ.run_instruction(&set_a)?;
        circ.run_instruction(&set_b)?;
        circ.run_instruction(&and_c)?;
        assert_eq!(circ.get_wire("c").unwrap(), 1);
        Ok(())
    }

    #[test]
    fn example_circuit_inorder() -> Result<(), CircuitError> {
        let ops = CircuitOp::from_lines(circuit_example().iter().map(|i| i.0))?;
        let mut circ = Circuit::new(ops.len());
        circ.resolve_circuit(ops)?;
        for (wire, value) in circuit_example_results() {
            assert_eq!(circ.get_wire(wire).unwrap(), value);
        }
        Ok(())
    }

    #[test]
    fn example_circuit_out_of_order() -> Result<(), CircuitError> {
        let mut ops = CircuitOp::from_lines(circuit_example().iter().map(|i| i.0))?;
        ops.swap(0, 4);
        ops.swap(1, 6);
        let mut circ = Circuit::new(ops.len());
        circ.resolve_circuit(ops)?;
        for (wire, value) in circuit_example_results() {
            assert_eq!(circ.get_wire(wire).unwrap(), value);
        }

        Ok(())
    }
}

use std::{iter::repeat, num::ParseIntError};

use aoc_2015::aoc_io;

fn main() {
    let input = aoc_io::get_input_as_string(6);
    let end_state_1 = run_light_instructions(input.lines());
    let part1 = end_state_1.total_lit();
    let end_state_2 = run_light_instructions_advanced(input.lines());
    let part2 = end_state_2.total_brightness();
    aoc_io::put_aoc_output((Some(u64::from(part1)), Some(part2)));
}

#[derive(Debug)]
enum ParseError {
    InstructionError(String),
    RectangleError(String),
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::RectangleError(err.to_string())
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    On(Rect),
    Off(Rect),
    Toggle(Rect),
    Brighten(Rect, u8),
    Dim(Rect),
}

impl Instruction {
    pub fn parse_instruction_basic(inst: &str) -> Result<Instruction, ParseError> {
        match inst {
            instr if instr.starts_with("turn on") => {
                Ok(Instruction::On(Rect::new_from_description(&instr[8..])?))
            }
            instr if instr.starts_with("turn off") => {
                Ok(Instruction::Off(Rect::new_from_description(&instr[9..])?))
            }
            instr if instr.starts_with("toggle") => Ok(Instruction::Toggle(
                Rect::new_from_description(&instr[7..])?,
            )),
            instr => Err(ParseError::InstructionError(format!("Can't parse {instr}"))),
        }
    }

    pub fn parse_instruction_advanced(inst: &str) -> Result<Instruction, ParseError> {
        match inst {
            instr if instr.starts_with("turn on") => Ok(Instruction::Brighten(
                Rect::new_from_description(&instr[8..])?,
                1,
            )),
            instr if instr.starts_with("turn off") => {
                Ok(Instruction::Dim(Rect::new_from_description(&instr[9..])?))
            }
            instr if instr.starts_with("toggle") => Ok(Instruction::Brighten(
                Rect::new_from_description(&instr[7..])?,
                2,
            )),
            instr => Err(ParseError::InstructionError(format!("Can't parse {instr}"))),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Rect {
    p1_x: u16,
    p1_y: u16,
    p2_x: u16,
    p2_y: u16,
}

impl Rect {
    pub fn new(p1_x: u16, p1_y: u16, p2_x: u16, p2_y: u16) -> Self {
        Rect {
            p1_x,
            p1_y,
            p2_x,
            p2_y,
        }
    }

    /// Takes a string with the format "w,x through y,z" and creates a new Rect
    /// with w,x describing the x and y co-ordinates of the top left point and
    /// y,z describing the co-ordinates of the bottom right point of the rectangle.
    pub fn new_from_description(input: &str) -> Result<Rect, ParseError> {
        let mut split = input.split(' ');
        let mut rectpoints = [0; 4];
        let mut idx: usize = 0;
        while idx < 4 {
            let mut pointstr = split
                .next()
                .ok_or(ParseError::RectangleError(
                    "Failed to find coordinate pair".to_string(),
                ))?
                .split(',');
            for _ in 0..2 {
                let point = pointstr.next().ok_or(ParseError::RectangleError(
                    "Failed to split coordinate pair".to_string(),
                ))?;
                let point = point.parse()?;
                rectpoints[idx] = point;
                idx += 1;
            }
            // throw away "through"
            split.next();
        }

        Ok(Rect::new(
            rectpoints[0],
            rectpoints[1],
            rectpoints[2],
            rectpoints[3],
        ))
    }

    pub fn get_xy_iterator(&self) -> Box<dyn Iterator<Item = (usize, usize)>> {
        let lowx: usize = self.p1_x.min(self.p2_x).into();
        let highx: usize = self.p1_x.max(self.p2_x).into();
        let lowy: usize = self.p1_y.min(self.p2_y).into();
        let highy: usize = self.p1_y.max(self.p2_y).into();
        let xs = lowx..=highx;
        // we have to build the second range actually in the closure or it errors:
        // cannot move out of ys, a captured variable in a FnMut closure
        // let ys = lowy..=highy;
        // let iter = xs.flat_map(move |x| repeat(x).zip(ys));
        let iter = xs.flat_map(move |x| repeat(x).zip(lowy..=highy));
        Box::new(iter)
    }
}

struct LightGrid {
    lights: [[u8; 1000]; 1000],
}

impl From<u8> for LightGrid {
    fn from(initial: u8) -> Self {
        LightGrid {
            lights: [[initial; 1000]; 1000],
        }
    }
}

impl LightGrid {
    pub fn new() -> Self {
        LightGrid {
            lights: [[0; 1000]; 1000],
        }
    }

    pub fn off(&mut self, bounds: &Rect) {
        self.set(bounds, 0);
    }

    pub fn on(&mut self, bounds: &Rect) {
        self.set(bounds, 1);
    }

    pub fn set(&mut self, bounds: &Rect, value: u8) {
        for (x, y) in bounds.get_xy_iterator() {
            self.lights[x][y] = value;
        }
    }

    pub fn toggle(&mut self, bounds: &Rect) {
        for (x, y) in bounds.get_xy_iterator() {
            if self.lights[x][y] > 0 {
                self.lights[x][y] = 0;
            } else {
                self.lights[x][y] = 1;
            }
        }
    }

    pub fn brighten(&mut self, bounds: &Rect, amount: u8) {
        for (x, y) in bounds.get_xy_iterator() {
            self.lights[x][y] = self.lights[x][y].saturating_add(amount);
        }
    }

    pub fn dim(&mut self, bounds: &Rect, amount: u8) {
        for (x, y) in bounds.get_xy_iterator() {
            self.lights[x][y] = self.lights[x][y].saturating_sub(amount);
        }
    }

    pub fn total_lit(&self) -> u32 {
        u32::try_from(
            self.lights
                .iter()
                .flatten()
                .filter(|light| **light > 0)
                .count(),
        )
        .expect("1000 * 1000 should always fit in u32")
    }

    pub fn total_brightness(&self) -> u64 {
        self.lights
            .iter()
            .flatten()
            .fold(0u64, |acc, x| acc + (*x as u64))
    }
}

fn run_light_instructions<'a>(input: impl IntoIterator<Item = &'a str>) -> LightGrid {
    let mut lights = LightGrid::new();
    for line in input {
        if let Ok(instruction) = Instruction::parse_instruction_basic(line) {
            transform_light_array(&mut lights, &instruction);
        } else {
            println!("WARNING: Skipping unknown instruction: {line}")
        }
    }
    lights
}

fn run_light_instructions_advanced<'a>(input: impl IntoIterator<Item = &'a str>) -> LightGrid {
    let mut lights = LightGrid::new();
    for line in input {
        if let Ok(instruction) = Instruction::parse_instruction_advanced(line) {
            transform_light_array(&mut lights, &instruction);
        } else {
            println!("WARNING: Skipping unknown instruction: {line}")
        }
    }
    lights
}

fn transform_light_array<'a>(
    lights: &'a mut LightGrid,
    instruction: &Instruction,
) -> &'a mut LightGrid {
    match instruction {
        Instruction::On(bounds) => lights.on(&bounds),
        Instruction::Off(bounds) => lights.off(&bounds),
        Instruction::Toggle(bounds) => lights.toggle(bounds),
        Instruction::Brighten(bounds, amount) => lights.brighten(bounds, *amount),
        Instruction::Dim(bounds) => lights.dim(bounds, 1),
    }
    lights
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rectangle_builder() -> Result<(), ParseError> {
        let result = Rect::new_from_description("10,10 through 20,20")?;
        assert_eq!(result.p1_x, 10, "top left x");
        assert_eq!(result.p1_y, 10, "top left y");
        assert_eq!(result.p2_x, 20, "bottom right x");
        assert_eq!(result.p2_y, 20, "bottom right y");
        Ok(())
    }

    #[test]
    fn test_parse_instruction() -> Result<(), ParseError> {
        assert_eq!(
            Instruction::parse_instruction_basic("turn on 0,0 through 999,999")?,
            Instruction::On(Rect::new(0, 0, 999, 999))
        );
        assert_eq!(
            Instruction::parse_instruction_basic("turn off 0,0 through 0,0")?,
            Instruction::Off(Rect::new(0, 0, 0, 0))
        );
        assert_eq!(
            Instruction::parse_instruction_basic("toggle 499,499 through 500,500")?,
            Instruction::Toggle(Rect::new(499, 499, 500, 500))
        );
        Ok(())
    }

    #[test]
    fn test_count_lights() {
        let mut lights = LightGrid::new();
        assert_eq!(lights.total_lit(), 0);
        lights.on(&Rect::new(0, 0, 0, 0));
        assert_eq!(lights.total_lit(), 1);
        lights.on(&Rect::new(999, 999, 999, 999));
        assert_eq!(lights.total_lit(), 2);
    }

    // turn on 0,0 through 999,999 would turn on (or leave on) every light.
    #[test]
    fn test_basic_transform_all() {
        let mut lights = LightGrid::new();
        let instruction =
            Instruction::parse_instruction_basic("turn on 0,0 through 999,999").unwrap();
        transform_light_array(&mut lights, &instruction);
        assert_eq!(lights.total_lit(), 1000 * 1000);
    }

    // toggle 0,0 through 999,0 would toggle the first line of 1000 lights, turning off the ones that were on, and turning on the ones that were off.
    #[test]
    fn test_basic_transform_line() {
        let mut lights = LightGrid::new();
        let instruction = Instruction::parse_instruction_basic("toggle 0,0 through 999,0").unwrap();
        transform_light_array(&mut lights, &instruction);
        assert_eq!(lights.total_lit(), 1000);
    }

    // turn off 499,499 through 500,500 would turn off (or leave off) the middle four lights.
    #[test]
    fn test_basic_transform_rect() {
        let mut lights = LightGrid::from(1);
        let instruction =
            Instruction::parse_instruction_basic("turn off 499,499 through 500,500").unwrap();
        transform_light_array(&mut lights, &instruction);
        assert_eq!(lights.total_lit(), (1000 * 1000) - 4);
    }

    // turn on 0,0 through 0,0 would increase the total brightness by 1.
    #[test]
    fn test_advanced_single() {
        let mut lights = LightGrid::new();
        let instruction =
            Instruction::parse_instruction_advanced("turn on 0,0 through 0,0").unwrap();
        transform_light_array(&mut lights, &instruction);
        assert_eq!(lights.total_brightness(), 1);
    }

    // toggle 0,0 through 999,999 would increase the total brightness by 2000000.
    #[test]
    fn test_advanced_toggle() {
        let mut lights = LightGrid::new();
        let instruction =
            Instruction::parse_instruction_advanced("toggle 0,0 through 999,999").unwrap();
        transform_light_array(&mut lights, &instruction);
        assert_eq!(lights.total_brightness(), 2_000_000);
    }
}

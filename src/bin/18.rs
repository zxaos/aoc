use std::ops::Range;

fn main() -> Result<(), &'static str> {
    let input = aoc_2015::aoc_io::get_input_as_string(18);
    let mut solution = aoc_2015::aoc_io::Solution::<usize>::new();
    let mut lights = LightGrid::<{ 100 * 100 }>::try_from(&input[..])?;
    let mut lights_stuck = lights.clone();
    for _ in 0..100 {
        lights.step_lights();
        lights_stuck.step_lights_corners_on();
    }
    solution[0].solution = Some(lights.count_on());
    solution[1].solution = Some(lights_stuck.count_on());

    solution.print();
    Ok(())
}

#[derive(Debug, Clone)]
struct LightGrid<const N: usize> {
    pub lights: [bool; N],
    rowsize: usize,
    extents: [(Range<usize>, LightType); 6], // Top and bottom light styles
}

type PartialExtents = [(Range<usize>, LightType); 6];
impl<const N: usize> LightGrid<N> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let (rowsize, extents) = LightGrid::<N>::new_support();
        LightGrid {
            lights: [false; N],
            rowsize,
            extents,
        }
    }

    // This is common creation code for new and TryFrom<str>
    fn new_support() -> (usize, PartialExtents) {
        assert!(N >= 4, "Cannot instantiate a Light grid smaller than 4");
        let rowsize = (N as f64).sqrt() as usize;

        assert!(
            rowsize.pow(2) == N,
            "Light Grids may only be square, check the size of the generic"
        );

        let extents = [
            // We can easily check for everything except left and right edges)
            (0..1, LightType::NW),                             // NW
            (1..(rowsize - 1), LightType::N),                  // N
            ((rowsize - 1)..rowsize, LightType::NE),           // NE
            ((N - rowsize)..(N - rowsize + 1), LightType::SW), // SW
            ((N - rowsize + 1)..(N - 1), LightType::S),        // S
            ((N - 1)..(N), LightType::SE),                     // SE
        ];

        (rowsize, extents)
    }

    pub fn count_on(&self) -> usize {
        self.lights.iter().filter(|&&x| x).count()
    }

    fn light_type(&self, idx: usize) -> &LightType {
        // maybe it's one of the canned ranges we built in new...
        if let Some((_, lt)) = self.extents.iter().find(|&(range, _)| range.contains(&idx)) {
            lt
        } else {
            match idx {
                // Ok, it's either an E/W edge, a middle, or invalid.
                x if x % self.rowsize == 0 => &LightType::W,
                x if (x + 1) % self.rowsize == 0 => &LightType::E,
                _ if (0..N).contains(&idx) => &LightType::Mid, // this should just be 0..N but we can't
                _ => &LightType::Invalid,
            }
        }
    }

    fn lit_neighbors_for(&self, light: usize) -> usize {
        /*
        0 1 2
        3 4 5
        6 7 8
        rowsize = 3
        */
        use self::LightType::*;
        let rowsize = self.rowsize;
        let lt = *self.light_type(light);

        // the enums refer to the directions on the grid where a light sits,
        // l<dir> is precalculated incides for lights relative to current light
        // so that we don't need to write (light - rowsize - 1) a million times
        // It's safe to ignore underflow here, as the cases which underflow
        // (the top row and the left side of 0) will never actually be used.
        #[rustfmt::skip]
        let (
            l_nw, l_n, l_ne,
            l_w,       l_e,
            l_sw, l_s, l_se
        ) = (
            // NB the wrapping subs have signs inverted because of precedence in the method call
            // i.e. light - rowsize - 1 = light.minus(rowsize + 1)
            light.wrapping_sub(rowsize + 1), light.wrapping_sub(rowsize), light.wrapping_sub(rowsize - 1),
            light.wrapping_sub(1),           /* this light */             light +1,
            light + rowsize - 1,             light + rowsize,             light + rowsize + 1
        );

        let neighbours = match lt {
            Mid => vec![l_n, l_ne, l_e, l_se, l_s, l_sw, l_w, l_nw],
            N => vec![l_e, l_se, l_s, l_sw, l_w],
            S => vec![l_n, l_ne, l_e, l_w, l_nw],
            E => vec![l_n, l_s, l_sw, l_w, l_nw],
            W => vec![l_n, l_ne, l_e, l_se, l_s],
            NW => vec![l_e, l_se, l_s],
            NE => vec![l_s, l_sw, l_w],
            SW => vec![l_n, l_ne, l_e],
            SE => vec![l_n, l_w, l_nw],
            Invalid => panic!("Got an invalid light position"),
        };
        neighbours
            .iter()
            .map(|&n| self.lights[n])
            .filter(|&l| l)
            .count()
    }

    fn next_state(&self, this_light: usize, this_light_state: bool) -> bool {
        let lit_neighbours = self.lit_neighbors_for(this_light);
        match this_light_state {
            true if lit_neighbours == 2 || lit_neighbours == 3 => true,
            true => false,
            false if lit_neighbours == 3 => true,
            false => false,
        }
    }

    fn step_lights(&mut self) {
        let mut idx = 0; // sadly there is no enumerate for array map
        self.lights = self.lights.map(|state| {
            let ret = self.next_state(idx, state);
            idx += 1;
            ret
        })
    }

    fn step_lights_corners_on(&mut self) {
        let mut idx = 0;
        self.lights = self.lights.map(|state| {
            let lt = self.light_type(idx);
            let ret = match lt {
                LightType::NW | LightType::NE | LightType::SW | LightType::SE => true,
                _ => self.next_state(idx, state),
            };
            idx += 1;
            ret
        })
    }
}

impl<const N: usize> TryFrom<&str> for LightGrid<N> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let actual_chars: Vec<char> = value.chars().filter(|&c| c != '\n').collect();
        if actual_chars.len() != N {
            println!("Length was {}, expected {}", actual_chars.len(), N);
            Err("Input is too short")
        } else if !actual_chars.iter().all(|&c| c == '#' || c == '.') {
            Err("Parsed string may contain only # and . characters.")
        } else {
            let mut lights = [false; N];
            for (idx, c) in value.chars().filter(|&c| c != '\n').enumerate() {
                lights[idx] = match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Got a bad char in grid build"),
                };
            }
            let (rowsize, extents) = LightGrid::<N>::new_support();
            Ok(LightGrid::<N> {
                lights,
                rowsize,
                extents,
            })
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum LightType {
    NW,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    Mid,
    Invalid,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_light_types_9() {
        use LightType::*;
        let lg = LightGrid::<9>::new();
        #[rustfmt::skip]
        let vals = [
            NW, N,   NE,
            W,  Mid, E,
            SW, S,   SE
        ];

        for (idx, lt) in vals.iter().enumerate() {
            assert_eq!((lg.light_type(idx)), lt);
        }
    }

    #[test]
    fn test_light_types_16() {
        use LightType::*;
        let lg = LightGrid::<16>::new();
        #[rustfmt::skip]
        let vals = [
            NW, N,   N,   NE,
            W,  Mid, Mid, E,
            W,  Mid, Mid, E,
            SW, S,   S,   SE
        ];

        for (idx, lt) in vals.iter().enumerate() {
            assert_eq!((lg.light_type(idx)), lt);
        }
    }

    #[test]
    fn test_light_count() {
        let mut lg = LightGrid::<16>::new();

        #[rustfmt::skip]
        let test_lights = [
            true,  false, false, false,
            false, true,  true,  true,
            false, false, false, false,
            false, true,  false, true
        ];
        #[rustfmt::skip]
        let n_counts = [
            1, 3, 3, 2,
            2, 2, 2, 1,
            2, 3, 5, 3,
            1, 0, 2, 0
        ];

        lg.lights = test_lights;
        for (idx, result) in n_counts.iter().enumerate() {
            assert_eq!(
                lg.lit_neighbors_for(idx),
                *result,
                "Light {idx} should have {result} lit neighbours!"
            )
        }
    }

    #[test]
    fn test_light_step() {
        let mut lg = LightGrid::<36>::new();

        #[rustfmt::skip]
        let frame_0 = [
            false, true,  false, true,  false, true,
            false, false, false, true,  true,  false,
            true,  false, false, false, false, true,
            false, false, true,  false, false, false,
            true,  false, true,  false, false, true,
            true,  true,  true,  true,  false, false,
        ];

        #[rustfmt::skip]
        let frame_1 = [
            false, false, true,  true,  false, false,
            false, false, true,  true,  false, true,
            false, false, false, true,  true,  false,
            false, false, false, false, false, false,
            true,  false, false, false, false, false,
            true,  false, true,  true,  false, false,
        ];

        lg.lights = frame_0;
        lg.step_lights();
        assert_eq!(lg.lights, frame_1);
    }
}

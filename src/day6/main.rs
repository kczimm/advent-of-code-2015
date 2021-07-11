/*!
--- Day 6: Probably a Fire Hazard ---

Because your neighbors keep defeating you in the holiday house decorating contest year after year, you've decided to deploy one million lights in a 1000x1000 grid.

Furthermore, because you've been especially nice this year, Santa has mailed you instructions on how to display the ideal lighting configuration.

Lights in your grid are numbered from 0 to 999 in each direction; the lights at each corner are at 0,0, 0,999, 999,999, and 999,0. The instructions include whether to turn on, turn off, or toggle various inclusive ranges given as coordinate pairs. Each coordinate pair represents opposite corners of a rectangle, inclusive; a coordinate pair like 0,0 through 2,2 therefore refers to 9 lights in a 3x3 square. The lights all start turned off.

To defeat your neighbors this year, all you have to do is set up your lights by doing the instructions Santa sent you in order.

For example:

    turn on 0,0 through 999,999 would turn on (or leave on) every light.
    toggle 0,0 through 999,0 would toggle the first line of 1000 lights, turning off the ones that were on, and turning on the ones that were off.
    turn off 499,499 through 500,500 would turn off (or leave off) the middle four lights.

After following the instructions, how many lights are lit?

--- Part Two ---

You just finish implementing your winning light pattern when you realize you mistranslated Santa's message from Ancient Nordic Elvish.

The light grid you bought actually has individual brightness controls; each light can have a brightness of zero or more. The lights all start at zero.

The phrase turn on actually means that you should increase the brightness of those lights by 1.

The phrase turn off actually means that you should decrease the brightness of those lights by 1, to a minimum of zero.

The phrase toggle actually means that you should increase the brightness of those lights by 2.

What is the total brightness of all lights combined after following Santa's instructions?

For example:

    turn on 0,0 through 0,0 would increase the total brightness by 1.
    toggle 0,0 through 999,999 would increase the total brightness by 2000000.
*/

use util::load;

use std::io;

fn main() -> io::Result<()> {
    let contents = load("src/day6/input.txt")?;

    let instructions = contents
        .lines()
        .map(|line| Instruction::from(line))
        .collect();

    let mut lights = Lights::new();
    lights.do_instructions(&instructions);
    println!("part1: {}", lights.num_lit_lights());

    let mut lights = Lights::new();
    lights.do_new_instructions(&instructions);
    println!("part2: {}", lights.total_brightness());

    Ok(())
}

const GRID_DIM: usize = 1000;

type Brightness = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Coordinate(usize, usize);

impl Coordinate {
    fn from(s: &str) -> Self {
        let comma = s.find(",").expect("no comma in coordinate");
        let first = (&s[..comma])
            .parse()
            .expect("first coordinate parse failed");
        let second = (&s[comma + 1..])
            .parse()
            .expect("first coordinate parse failed");
        Self(first, second)
    }
}

type CoordinatePair = (Coordinate, Coordinate);

struct Lights {
    grid: Vec<Vec<Brightness>>,
}

impl Lights {
    fn new() -> Self {
        Self {
            grid: vec![vec![0; GRID_DIM]; GRID_DIM],
        }
    }

    fn total_brightness(&self) -> Brightness {
        self.grid
            .iter()
            .map(|row| {
                let brightness: usize = row.iter().sum();
                brightness
            })
            .sum()
    }

    fn do_instruction(&mut self, instruction: Instruction) {
        let (c1, c2) = instruction.coordinates;
        for i in c1.0..=c2.0 {
            for j in c1.1..=c2.1 {
                match instruction.action {
                    LightAction::TurnOff => self.grid[i][j] = 0,
                    LightAction::TurnOn => self.grid[i][j] = 1,
                    LightAction::Toggle => self.grid[i][j] ^= 1,
                }
            }
        }
    }

    fn do_instructions(&mut self, instructions: &Vec<Instruction>) {
        instructions
            .iter()
            .for_each(|instruction| self.do_instruction(*instruction));
    }

    fn do_new_instruction(&mut self, instruction: Instruction) {
        let (c1, c2) = instruction.coordinates;
        for i in c1.0..=c2.0 {
            for j in c1.1..=c2.1 {
                match instruction.action {
                    LightAction::TurnOn => self.grid[i][j] += 1,
                    LightAction::TurnOff => {
                        self.grid[i][j] = if self.grid[i][j] == 0 {
                            0
                        } else {
                            self.grid[i][j] - 1
                        }
                    }
                    LightAction::Toggle => self.grid[i][j] += 2,
                }
            }
        }
    }

    fn do_new_instructions(&mut self, instructions: &Vec<Instruction>) {
        instructions
            .iter()
            .for_each(|instruction| self.do_new_instruction(*instruction));
    }

    fn num_lit_lights(&self) -> usize {
        self.grid
            .iter()
            .map(|inner| inner.iter().filter(|brightness| **brightness == 1).count())
            .sum()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LightAction {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Instruction {
    coordinates: CoordinatePair,
    action: LightAction,
}

impl Instruction {
    fn from(s: &str) -> Self {
        let mut parts = s.split(' ');
        let turn_or_toggle = parts.next().unwrap();
        match turn_or_toggle {
            "turn" => {
                let on_or_off = parts.next().unwrap();
                let c1 = Coordinate::from(parts.next().unwrap());
                parts.next();
                let c2 = Coordinate::from(parts.next().unwrap());
                match on_or_off {
                    "on" => Self {
                        coordinates: (c1, c2),
                        action: LightAction::TurnOn,
                    },
                    "off" => Self {
                        coordinates: (c1, c2),
                        action: LightAction::TurnOff,
                    },
                    s => panic!("unexpected word in instruction: {}", s),
                }
            }
            "toggle" => {
                let c1 = Coordinate::from(parts.next().unwrap());
                parts.next();
                let c2 = Coordinate::from(parts.next().unwrap());
                Self {
                    coordinates: (c1, c2),
                    action: LightAction::Toggle,
                }
            }
            s => panic!("unexpected word in instruction: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_instruction() {
        let mut lights = Lights::new();
        lights.do_instruction(Instruction {
            coordinates: (Coordinate(0, 0), Coordinate(999, 999)),
            action: LightAction::TurnOn,
        });
        assert_eq!(lights.num_lit_lights(), 1_000_000);

        let mut lights = Lights::new();
        lights.do_instruction(Instruction {
            coordinates: (Coordinate(0, 0), Coordinate(999, 0)),
            action: LightAction::Toggle,
        });
        assert_eq!(lights.num_lit_lights(), 1_000);

        let mut lights = Lights::new();
        lights.do_instruction(Instruction {
            coordinates: (Coordinate(499, 499), Coordinate(500, 500)),
            action: LightAction::Toggle,
        });
        assert_eq!(lights.num_lit_lights(), 4);
    }

    #[test]
    fn test_coordinate_from() {
        assert_eq!(Coordinate::from("1,1"), Coordinate(1, 1));
        assert_eq!(Coordinate::from("999,999"), Coordinate(999, 999));
    }

    #[test]
    fn test_instruction_from() {
        assert_eq!(
            Instruction::from("turn on 0,0 through 999,999"),
            Instruction {
                coordinates: (Coordinate(0, 0), Coordinate(999, 999)),
                action: LightAction::TurnOn,
            }
        );
        assert_eq!(
            Instruction::from("toggle 0,0 through 999,0"),
            Instruction {
                coordinates: (Coordinate(0, 0), Coordinate(999, 0)),
                action: LightAction::Toggle,
            }
        );
        assert_eq!(
            Instruction::from("turn off 499,499 through 500,500"),
            Instruction {
                coordinates: (Coordinate(499, 499), Coordinate(500, 500)),
                action: LightAction::TurnOff,
            }
        );
    }
}

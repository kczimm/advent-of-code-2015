/*!
--- Day 3: Perfectly Spherical Houses in a Vacuum ---

Santa is delivering presents to an infinite two-dimensional grid of houses.

He begins by delivering a present to the house at his starting location, and then an elf at the North Pole calls him via radio and tells him where to move next. Moves are always exactly one house to the north (^), south (v), east (>), or west (<). After each move, he delivers another present to the house at his new location.

However, the elf back at the north pole has had a little too much eggnog, and so his directions are a little off, and Santa ends up visiting some houses more than once. How many houses receive at least one present?

For example:

    > delivers presents to 2 houses: one at the starting location, and one to the east.
    ^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
    ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.

--- Part Two ---

The next year, to speed up the process, Santa creates a robot version of himself, Robo-Santa, to deliver presents with him.

Santa and Robo-Santa start at the same location (delivering two presents to the same starting house), then take turns moving based on instructions from the elf, who is eggnoggedly reading from the same script as the previous year.

This year, how many houses receive at least one present?

For example:

    ^v delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
    ^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
    ^v^v^v^v^v now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.

*/
use util::load;

use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let moves = load("src/day3/input.txt")?;

    let mut santa = Santa::ready_sleigh();
    santa.deliver_presents(&moves);
    println!("part1: {}", santa.num_lucky_children());

    let mut santa = Santa::with_robo_santa();
    santa.deliver_presents(&moves);
    println!("part2: {}", santa.num_lucky_children());

    Ok(())
}

type Moves<'a> = &'a str;

type Location = (isize, isize);

const STARTING_LOCATION: Location = (0, 0);

struct Santa {
    current_location: Location,
    presents_delivered: HashMap<Location, usize>,
    robo_location: Option<Location>,
}

impl Santa {
    fn ready_sleigh() -> Self {
        let mut presents_delivered = HashMap::new();
        presents_delivered.insert(STARTING_LOCATION, 1);
        Self {
            current_location: STARTING_LOCATION,
            presents_delivered,
            robo_location: None,
        }
    }

    fn with_robo_santa() -> Self {
        let mut presents_delivered = HashMap::new();
        presents_delivered.insert(STARTING_LOCATION, 2);
        Self {
            current_location: STARTING_LOCATION,
            presents_delivered,
            robo_location: Some(STARTING_LOCATION),
        }
    }

    fn deliver_presents(&mut self, moves: Moves) {
        for (i, direction) in moves.chars().enumerate() {
            let location = if let Some(robo_location) = self.robo_location.as_mut() {
                if i % 2 == 0 {
                    // Santa's turn
                    self.current_location = move_in_direction(self.current_location, direction);
                    self.current_location
                } else {
                    // Robo-Santa's turn
                    *robo_location = move_in_direction(*robo_location, direction);
                    *robo_location
                }
            } else {
                self.current_location = move_in_direction(self.current_location, direction);
                self.current_location
            };
            let num_presents = self.presents_delivered.entry(location).or_insert(0);
            *num_presents += 1;
        }
    }

    fn num_lucky_children(&self) -> usize {
        self.presents_delivered
            .values()
            .filter(|v| **v >= 1)
            .count()
    }
}

fn move_in_direction(location: Location, direction: char) -> Location {
    let mut new = (location.0, location.1);
    match direction {
        '^' => new.1 += 1,
        '<' => new.0 -= 1,
        '>' => new.0 += 1,
        'v' => new.1 -= 1,
        d => panic!("bad direction: {}", d),
    }
    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_lucky_children() {
        let moves: Moves = "^v^v^v^v^v";
        let mut santa = Santa::ready_sleigh();
        santa.deliver_presents(moves);
        assert_eq!(santa.num_lucky_children(), 2);

        let moves: Moves = "^>v<^>v";
        let mut santa = Santa::ready_sleigh();
        santa.deliver_presents(moves);
        assert_eq!(santa.num_lucky_children(), 4);

        let moves: Moves = "^>v<";
        let mut santa = Santa::ready_sleigh();
        santa.deliver_presents(moves);
        assert_eq!(santa.num_lucky_children(), 4);
    }

    #[test]
    fn test_num_lucky_children_with_robo_santa() {
        let moves: Moves = "^v";
        let mut santa = Santa::with_robo_santa();
        santa.deliver_presents(moves);
        assert_eq!(santa.num_lucky_children(), 3);

        let moves: Moves = "^>v<";
        let mut santa = Santa::with_robo_santa();
        santa.deliver_presents(moves);
        assert_eq!(santa.num_lucky_children(), 3);

        let moves: Moves = "^v^v^v^v^v";
        let mut santa = Santa::with_robo_santa();
        santa.deliver_presents(moves);
        assert_eq!(santa.num_lucky_children(), 11);
    }
}

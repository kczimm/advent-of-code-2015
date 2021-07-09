/*!
--- Day 2: I Was Told There Would Be No Math ---

The elves are running low on wrapping paper, and so they need to submit an order for more. They have a list of the dimensions (length l, width w, and height h) of each present, and only want to order exactly as much as they need.

Fortunately, every present is a box (a perfect right rectangular prism), which makes calculating the required wrapping paper for each gift a little easier: find the surface area of the box, which is 2*l*w + 2*w*h + 2*h*l. The elves also need a little extra paper for each present: the area of the smallest side.

For example:

    A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
    A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.

All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?
*/

use util::load;

use std::io;

fn main() -> io::Result<()> {
    let list = load("src/day2/input.txt")?;

    let presents = read_list(&list);

    println!("part1: {}", total_wrapping_paper(&presents));

    Ok(())
}

type Length = usize;
type Width = usize;
type Height = usize;
type SquareFeet = usize;

fn read_list(list: &str) -> Vec<Present> {
    list.lines().map(|dims| Present::from(dims)).collect()
}

fn total_wrapping_paper(presents: &Vec<Present>) -> SquareFeet {
    presents.iter().map(|p| p.surface_area()).sum()
}

struct Present {
    dimensions: (Length, Width, Height),
}

impl Present {
    fn from(s: &str) -> Self {
        let mut parts = s.split('x');
        let length = parts
            .next()
            .expect("no length")
            .parse::<Length>()
            .expect("failed to parse length");

        let width = parts
            .next()
            .expect("no width")
            .parse::<Width>()
            .expect("failed to parse width");

        let height = parts
            .next()
            .expect("no height")
            .parse::<Height>()
            .expect("failed to parse height");

        Self {
            dimensions: (length, width, height),
        }
    }

    fn surface_area(&self) -> SquareFeet {
        let side1 = self.dimensions.0 * self.dimensions.1;
        let side2 = self.dimensions.1 * self.dimensions.2;
        let side3 = self.dimensions.0 * self.dimensions.2;
        let smallest = *[side1, side2, side3]
            .iter()
            .reduce(|a, b| if a < b { a } else { b })
            .unwrap();
        2 * (side1 + side2 + side3) + smallest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_area() {
        let present = Present::from("2x3x4");
        assert_eq!(present.surface_area(), 58);

        let present = Present::from("1x1x10");
        assert_eq!(present.surface_area(), 43);
    }
}

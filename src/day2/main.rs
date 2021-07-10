/*!
--- Day 2: I Was Told There Would Be No Math ---

The elves are running low on wrapping paper, and so they need to submit an order for more. They have a list of the dimensions (length l, width w, and height h) of each present, and only want to order exactly as much as they need.

Fortunately, every present is a box (a perfect right rectangular prism), which makes calculating the required wrapping paper for each gift a little easier: find the surface area of the box, which is 2*l*w + 2*w*h + 2*h*l. The elves also need a little extra paper for each present: the area of the smallest side.

For example:

    A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
    A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.

All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?

--- Part Two ---

The elves are also running low on ribbon. Ribbon is all the same width, so they only have to worry about the length they need to order, which they would again like to be exact.

The ribbon required to wrap a present is the shortest distance around its sides, or the smallest perimeter of any one face. Each present also requires a bow made out of ribbon as well; the feet of ribbon required for the perfect bow is equal to the cubic feet of volume of the present. Don't ask how they tie the bow, though; they'll never tell.

For example:

    A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to wrap the present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet.
    A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to wrap the present plus 1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet.

How many total feet of ribbon should they order?

*/

use util::load;

use std::io;

fn main() -> io::Result<()> {
    let list = load("src/day2/input.txt")?;

    let presents = read_list(&list);

    println!("part1: {}", total_wrapping_paper(&presents));
    println!("part2: {}", total_ribbon(&presents));

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

fn total_ribbon(presents: &Vec<Present>) -> Length {
    presents.iter().map(|p| p.required_ribbon()).sum()
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

    fn required_ribbon(&self) -> Length {
        let bow_length = self.dimensions.0 * self.dimensions.1 * self.dimensions.2;
        let ribbon_length = if self.dimensions.0 > self.dimensions.1
            && self.dimensions.0 > self.dimensions.2
        {
            2 * (self.dimensions.1 + self.dimensions.2)
        } else if self.dimensions.1 > self.dimensions.0 && self.dimensions.1 > self.dimensions.2 {
            2 * (self.dimensions.0 + self.dimensions.2)
        } else {
            2 * (self.dimensions.0 + self.dimensions.1)
        };

        ribbon_length + bow_length
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

    #[test]
    fn test_required_ribbon() {
        let present = Present::from("2x3x4");
        assert_eq!(present.required_ribbon(), 34);

        let present = Present::from("2x4x3");
        assert_eq!(present.required_ribbon(), 34);

        let present = Present::from("3x2x4");
        assert_eq!(present.required_ribbon(), 34);

        let present = Present::from("3x4x2");
        assert_eq!(present.required_ribbon(), 34);

        let present = Present::from("4x2x3");
        assert_eq!(present.required_ribbon(), 34);

        let present = Present::from("4x3x2");
        assert_eq!(present.required_ribbon(), 34);

        let present = Present::from("1x1x10");
        assert_eq!(present.required_ribbon(), 14);

        let present = Present::from("10x1x1");
        assert_eq!(present.required_ribbon(), 14);

        let present = Present::from("1x10x1");
        assert_eq!(present.required_ribbon(), 14);
    }
}

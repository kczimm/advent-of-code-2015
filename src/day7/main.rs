/*!
--- Day 7: Some Assembly Required ---

This year, Santa brought little Bobby Tables a set of wires and bitwise logic gates! Unfortunately, little Bobby is a little under the recommended age range, and he needs help assembling the circuit.

Each wire has an identifier (some lowercase letters) and can carry a 16-bit signal (a number from 0 to 65535). A signal is provided to each wire by a gate, another wire, or some specific value. Each wire can only get a signal from one source, but can provide its signal to multiple destinations. A gate provides no signal until all of its inputs have a signal.

The included instructions booklet describes how to connect the parts together: x AND y -> z means to connect wires x and y to an AND gate, and then connect its output to wire z.

For example:

    123 -> x means that the signal 123 is provided to wire x.
    x AND y -> z means that the bitwise AND of wire x and wire y is provided to wire z.
    p LSHIFT 2 -> q means that the value from wire p is left-shifted by 2 and then provided to wire q.
    NOT e -> f means that the bitwise complement of the value from wire e is provided to wire f.

Other possible gates include OR (bitwise OR) and RSHIFT (right-shift). If, for some reason, you'd like to emulate the circuit instead, almost all programming languages (for example, C, JavaScript, or Python) provide operators for these gates.

For example, here is a simple circuit:

123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i

After it is run, these are the signals on the wires:

d: 72
e: 507
f: 492
g: 114
h: 65412
i: 65079
x: 123
y: 456

In little Bobby's kit's instructions booklet (provided as your puzzle input), what signal is ultimately provided to wire a?
*/

use util::load;

use std::io;

use std::collections::{HashMap, HashSet};

fn main() -> io::Result<()> {
    let contents = load("src/day7/input.txt")?;

    let booklet = read_booklet(&contents);

    let mut wires = Wires::new();
    wires.run_booklet(&booklet);

    println!("part1: {}", wires.inner.get("a").unwrap());

    Ok(())
}

fn read_booklet(s: &str) -> Vec<Instruction> {
    s.lines().map(|line| Instruction::from(line)).collect()
}

#[derive(Debug, PartialEq)]
enum Instruction<'a> {
    Literal(u16, &'a str),
    And(&'a str, &'a str, &'a str),
    Or(&'a str, &'a str, &'a str),
    Not(&'a str, &'a str),
    Rshift(&'a str, u8, &'a str),
    Lshift(&'a str, u8, &'a str),
    Move(&'a str, &'a str),
}

impl<'a> Instruction<'a> {
    fn from(s: &'a str) -> Self {
        let mut parts = s.split(' ');
        if s.contains("AND") {
            let input1 = parts.next().unwrap();
            parts.next(); // skip AND
            let input2 = parts.next().unwrap();
            parts.next(); // skip ->
            let output = parts.next().unwrap();
            Self::And(input1, input2, output)
        } else if s.contains("OR") {
            let input1 = parts.next().unwrap();
            parts.next(); // skip OR
            let input2 = parts.next().unwrap();
            parts.next(); // skip ->
            let output = parts.next().unwrap();
            Self::Or(input1, input2, output)
        } else if s.contains("RSHIFT") {
            let input = parts.next().unwrap();
            parts.next(); // skip RSHIFT
            let amount = parts.next().unwrap().parse().unwrap();
            parts.next(); // skip ->
            let output = parts.next().unwrap();
            Self::Rshift(input, amount, output)
        } else if s.contains("LSHIFT") {
            let input = parts.next().unwrap();
            parts.next(); // skip LSHIFT
            let amount = parts.next().unwrap().parse().unwrap();
            parts.next(); // skip ->
            let output = parts.next().unwrap();
            Self::Lshift(input, amount, output)
        } else if s.contains("NOT") {
            parts.next(); // skip NOT
            let input = parts.next().unwrap();
            parts.next(); // skip ->
            let output = parts.next().unwrap();
            Self::Not(input, output)
        } else {
            // LITERAL and MOVE
            let input = parts.next().unwrap();
            parts.next(); // skip ->
            let output = parts.next().unwrap();
            if let Ok(literal) = input.parse() {
                Self::Literal(literal, output)
            } else {
                Self::Move(input, output)
            }
        }
    }
}

struct Wires<'a> {
    inner: HashMap<&'a str, u16>,
}

impl<'a> Wires<'a> {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn run_booklet(&mut self, booklet: &'a Vec<Instruction>) {
        let mut remaining: HashSet<usize> = (0..booklet.len()).collect();
        while !remaining.is_empty() {
            for (i, instruction) in booklet.iter().enumerate() {
                if remaining.contains(&i) && self.do_instruction(instruction).is_ok() {
                    remaining.remove(&i);
                }
            }
            if remaining.len() == 320 {
                println!("{:?}", self.inner);

                panic!("inf loop");
            }
        }
    }

    fn do_instruction(&mut self, instruction: &'a Instruction<'a>) -> Result<(), &'a str> {
        match instruction {
            Instruction::And(in1, in2, out) => self.inner.insert(
                out,
                *self.inner.get(in1).ok_or("not set yet")?
                    & *self.inner.get(in2).ok_or("not set yet")?,
            ),
            Instruction::Or(in1, in2, out) => self.inner.insert(
                out,
                *self.inner.get(in1).ok_or("not set yet")?
                    | *self.inner.get(in2).ok_or("not set yet")?,
            ),
            Instruction::Rshift(input, amount, out) => self
                .inner
                .insert(out, *self.inner.get(input).ok_or("not set yet")? >> amount),
            Instruction::Lshift(input, amount, out) => self
                .inner
                .insert(out, *self.inner.get(input).ok_or("not set yet")? << amount),
            Instruction::Not(input, out) => self
                .inner
                .insert(out, !*self.inner.get(input).ok_or("not set yet")?),
            Instruction::Literal(value, name) => self.inner.insert(name, *value),
            Instruction::Move(input, output) => self
                .inner
                .insert(output, *self.inner.get(input).ok_or("not set yet")?),
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_from() {
        let instruction = Instruction::from("123 -> x");
        assert_eq!(instruction, Instruction::Literal(123, "x"));

        let instruction = Instruction::from("x AND y -> d");
        assert_eq!(instruction, Instruction::And("x", "y", "d"));

        let instruction = Instruction::from("x OR y -> e");
        assert_eq!(instruction, Instruction::Or("x", "y", "e"));

        let instruction = Instruction::from("x LSHIFT 2 -> f");
        assert_eq!(instruction, Instruction::Lshift("x", 2, "f"));

        let instruction = Instruction::from("y RSHIFT 2 -> g");
        assert_eq!(instruction, Instruction::Rshift("y", 2, "g"));

        let instruction = Instruction::from("NOT x -> h");
        assert_eq!(instruction, Instruction::Not("x", "h"));

        let instruction = Instruction::from("NOT y -> i");
        assert_eq!(instruction, Instruction::Not("y", "i"));

        let instruction = Instruction::from("y -> i");
        assert_eq!(instruction, Instruction::Move("y", "i"));
    }

    #[test]
    fn test_wire_do_instructions() {
        let s = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

        let mut wires = Wires::new();
        let booklet = read_booklet(s);
        wires.run_booklet(&booklet);

        // d: 72
        assert_eq!(*wires.inner.get("d").unwrap(), 72);

        // e: 507
        assert_eq!(*wires.inner.get("e").unwrap(), 507);

        // f: 492
        assert_eq!(*wires.inner.get("f").unwrap(), 492);

        // g: 114
        assert_eq!(*wires.inner.get("g").unwrap(), 114);

        // h: 65412
        assert_eq!(*wires.inner.get("h").unwrap(), 65412);

        // i: 65079
        assert_eq!(*wires.inner.get("i").unwrap(), 65079);

        // x: 123
        assert_eq!(*wires.inner.get("x").unwrap(), 123);

        // y: 456
        assert_eq!(*wires.inner.get("y").unwrap(), 456);
    }

    #[test]
    fn test_wire_do_instructions_reordered() {
        let s = "x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
123 -> x
456 -> y";

        let mut wires = Wires::new();
        let booklet = read_booklet(s);
        wires.run_booklet(&booklet);

        // d: 72
        assert_eq!(*wires.inner.get("d").unwrap(), 72);

        // e: 507
        assert_eq!(*wires.inner.get("e").unwrap(), 507);

        // f: 492
        assert_eq!(*wires.inner.get("f").unwrap(), 492);

        // g: 114
        assert_eq!(*wires.inner.get("g").unwrap(), 114);

        // h: 65412
        assert_eq!(*wires.inner.get("h").unwrap(), 65412);

        // i: 65079
        assert_eq!(*wires.inner.get("i").unwrap(), 65079);

        // x: 123
        assert_eq!(*wires.inner.get("x").unwrap(), 123);

        // y: 456
        assert_eq!(*wires.inner.get("y").unwrap(), 456);
    }
}

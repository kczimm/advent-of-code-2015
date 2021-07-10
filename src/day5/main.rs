/*!
--- Day 5: Doesn't He Have Intern-Elves For This? ---

Santa needs help figuring out which strings in his text file are naughty or nice.

A nice string is one with all of the following properties:

    It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
    It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
    It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.

For example:

    ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
    aaa is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
    jchzalrnumimnmhp is naughty because it has no double letter.
    haegwjzuvuyypxyu is naughty because it contains the string xy.
    dvszwmarrgswjxmb is naughty because it contains only one vowel.

How many strings are nice?
*/

use util::load;

use std::io;

fn main() -> io::Result<()> {
    let text = load("src/day5/input.txt")?;

    println!("part1: {}", number_of_nice_strings(&text));

    Ok(())
}

fn number_of_nice_strings(text: &str) -> usize {
    text.lines()
        .filter(|s| SantaString::from(s).is_nice())
        .count()
}

const FORBIDDEN_COMBOS: [&'static str; 4] = ["ab", "cd", "pq", "xy"];

struct SantaString<'a>(&'a str);

impl<'a> SantaString<'a> {
    fn from(s: &'a str) -> Self {
        Self(s)
    }

    fn is_nice(&self) -> bool {
        self.contains_three_vowels() & self.has_repeated_letter() & self.has_no_forbidden_combos()
    }

    fn contains_three_vowels(&self) -> bool {
        self.0
            .chars()
            .filter(|c| match c {
                'a' | 'e' | 'i' | 'o' | 'u' => true,
                _ => false,
            })
            .count()
            >= 3
    }

    fn has_repeated_letter(&self) -> bool {
        let mut prev = self.0.chars().next().expect("string has no characters");
        for c in self.0.chars().skip(1) {
            if c == prev {
                return true;
            }
            prev = c;
        }
        false
    }

    fn has_no_forbidden_combos(&self) -> bool {
        FORBIDDEN_COMBOS.iter().all(|combo| !self.0.contains(combo))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice() {
        let s = SantaString::from("ugknbfddgicrmopn");
        assert!(s.is_nice());

        let s = SantaString::from("jchzalrnumimnmhp");
        assert!(!s.is_nice());

        let s = SantaString::from("haegwjzuvuyypxyu");
        assert!(!s.is_nice());

        let s = SantaString::from("dvszwmarrgswjxmb");
        assert!(!s.is_nice());
    }

    #[test]
    fn test_contains_three_vowels() {
        let s = SantaString::from("aei");
        assert!(s.contains_three_vowels());

        let s = SantaString::from("xazegov");
        assert!(s.contains_three_vowels());

        let s = SantaString::from("aeiouaeiouaeiou");
        assert!(s.contains_three_vowels());
    }

    #[test]
    fn test_has_repeated_letter() {
        let s = SantaString::from("xx");
        assert!(s.has_repeated_letter());

        let s = SantaString::from("abcdde");
        assert!(s.has_repeated_letter());

        let s = SantaString::from("aabbccdd");
        assert!(s.has_repeated_letter());
    }

    #[test]
    fn test_has_no_forbidden_combos() {
        let s = SantaString::from("aei");
        assert!(s.has_no_forbidden_combos());

        let s = SantaString::from("xazegov");
        assert!(s.has_no_forbidden_combos());

        let s = SantaString::from("aeiouaeiouaeiou");
        assert!(s.has_no_forbidden_combos());

        let s = SantaString::from("abcdefg");
        assert!(!s.has_no_forbidden_combos());
    }
}

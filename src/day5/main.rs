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

--- Part Two ---

Realizing the error of his ways, Santa has switched to a better model of determining whether a string is naughty or nice. None of the old rules apply, as they are all clearly ridiculous.

Now, a nice string is one with all of the following properties:

    It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
    It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.

For example:

    qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly one letter between them (zxz).
    xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
    uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a single letter between them.
    ieodomkazucvgmuy is naughty because it has a repeating letter with one between (odo), but no pair that appears twice.

How many strings are nice under these new rules?

*/

use util::load;

use std::io;

fn main() -> io::Result<()> {
    let text = load("src/day5/input.txt")?;

    println!("part1: {}", number_of_nice_strings(&text));
    println!("part2: {}", number_of_nicer_strings(&text));

    Ok(())
}

fn number_of_nice_strings(text: &str) -> usize {
    text.lines()
        .filter(|s| SantaString::from(s).is_nice())
        .count()
}

fn number_of_nicer_strings(text: &str) -> usize {
    text.lines()
        .filter(|s| SantaString::from(s).is_nicer())
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

    fn is_nicer(&self) -> bool {
        self.has_repeated_pair_without_overlap() & self.contains_repeat_with_one_letter_between()
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
        let mut prev = match self.0.chars().next() {
            Some(p) => p,
            _ => return false,
        };
        for c in self.0.chars().skip(1) {
            if c == prev {
                return true;
            }
            prev = c;
        }
        false
    }

    fn has_repeated_pair_without_overlap(&self) -> bool {
        for i in 0..self.0.len() - 3 {
            let this = &self.0[i..i + 2];
            for j in i + 1..self.0.len() - 1 {
                let that = &self.0[j..j + 2];
                if this == that {
                    return true;
                }
            }
        }
        false
    }

    fn contains_repeat_with_one_letter_between(&self) -> bool {
        let mut chars = self.0.chars();
        let mut prev2 = match chars.next() {
            Some(p) => p,
            _ => return false,
        };
        let mut prev = match chars.next() {
            Some(p) => p,
            _ => return false,
        };

        for c in self.0.chars().skip(2) {
            if c == prev2 {
                return true;
            }
            prev2 = prev;
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
    fn test_is_nicer() {
        let s = SantaString::from("qjhvhtzxzqqjkmpb");
        assert!(s.is_nicer());

        let s = SantaString::from("xxyxx");
        assert!(s.is_nicer());

        let s = SantaString::from("uurcxstgmygtbstg");
        assert!(!s.is_nicer());

        let s = SantaString::from("ieodomkazucvgmuy");
        assert!(!s.is_nicer());
    }

    #[test]
    fn test_contains_repeat_with_one_letter_between() {
        let s = SantaString::from("xyxy");
        assert!(s.contains_repeat_with_one_letter_between());

        let s = SantaString::from("abcdefeghi");
        assert!(s.contains_repeat_with_one_letter_between());

        let s = SantaString::from("aaa");
        assert!(s.contains_repeat_with_one_letter_between());

        let s = SantaString::from("ieodomkazucvgmuy");
        assert!(s.contains_repeat_with_one_letter_between());
    }

    #[test]
    fn test_has_repeated_pair_without_overlap() {
        let s = SantaString::from("xyxy");
        assert!(s.has_repeated_pair_without_overlap());

        let s = SantaString::from("aabcdefgaa");
        assert!(s.has_repeated_pair_without_overlap());

        let s = SantaString::from("aaa");
        assert!(!s.has_repeated_pair_without_overlap());

        let s = SantaString::from("ieodomkazucvgmuy");
        assert!(!s.has_repeated_pair_without_overlap());
    }

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

/*!
--- Day 8: Matchsticks ---

Space on the sleigh is limited this year, and so Santa will be bringing his list as a digital copy. He needs to know how much space it will take up when stored.

It is common in many programming languages to provide a way to escape special characters in strings. For example, C, JavaScript, Perl, Python, and even PHP handle special characters in very similar ways.

However, it is important to realize the difference between the number of characters in the code representation of the string literal and the number of characters in the in-memory string itself.

For example:

    "" is 2 characters of code (the two double quotes), but the string contains zero characters.
    "abc" is 5 characters of code, but 3 characters in the string data.
    "aaa\"aaa" is 10 characters of code, but the string itself contains six "a" characters and a single, escaped quote character, for a total of 7 characters in the string data.
    "\x27" is 6 characters of code, but the string itself contains just one - an apostrophe ('), escaped using hexadecimal notation.

Santa's list is a file that contains many double-quoted string literals, one on each line. The only escape sequences used are \\ (which represents a single backslash), \" (which represents a lone double-quote character), and \x plus two hexadecimal characters (which represents a single character with that ASCII code).

Disregarding the whitespace in the file, what is the number of characters of code for string literals minus the number of characters in memory for the values of the strings in total for the entire file?

For example, given the four strings above, the total number of characters of string code (2 + 5 + 10 + 6 = 23) minus the total number of characters in memory for string values (0 + 3 + 7 + 1 = 11) is 23 - 11 = 12.

--- Part Two ---

Now, let's go the other way. In addition to finding the number of characters of code, you should now encode each code representation as a new string and find the number of characters of the new encoded representation, including the surrounding double quotes.

For example:

    "" encodes to "\"\"", an increase from 2 characters to 6.
    "abc" encodes to "\"abc\"", an increase from 5 characters to 9.
    "aaa\"aaa" encodes to "\"aaa\\\"aaa\"", an increase from 10 characters to 16.
    "\x27" encodes to "\"\\x27\"", an increase from 6 characters to 11.

Your task is to find the total number of characters to represent the newly encoded strings minus the number of characters of code in each original string literal. For example, for the strings above, the total encoded length (6 + 9 + 16 + 11 = 42) minus the characters in the original code representation (23, just like in the first part of this puzzle) is 42 - 23 = 19.
*/

use util::load;

use std::io;

fn main() -> io::Result<()> {
    let contents = load("src/day8/input.txt")?;

    let list = SantasList::from(&contents);

    println!("part1: {}", list.code_minus_memory());
    println!("part2: {}", list.encoded_minus_code());

    Ok(())
}

struct SantasList<'a>(Vec<MyString<'a>>);

impl<'a> SantasList<'a> {
    fn from(s: &'a str) -> Self {
        Self(s.lines().map(|line| MyString::from(line)).collect())
    }

    fn code_minus_memory(&self) -> usize {
        self.0
            .iter()
            .map(|s| s.code_length() - s.memory_length())
            .sum()
    }

    fn encoded_minus_code(&self) -> usize {
        self.0
            .iter()
            .map(|s| s.encoded_length() - s.code_length())
            .sum()
    }
}

#[derive(Debug, PartialEq)]
struct MyString<'a> {
    inner: &'a str,
}

impl<'a> MyString<'a> {
    fn from(s: &'a str) -> Self {
        Self { inner: s }
    }

    fn code_length(&self) -> usize {
        self.inner.len()
    }

    fn encoded_length(&self) -> usize {
        // code_length + 2 + (number of \ and ")
        let num_slash = self.inner.chars().filter(|c| *c == '\\').count();
        let num_paren = self.inner.chars().filter(|c| *c == '"').count();
        self.code_length() + 2 + num_slash + num_paren
    }

    fn memory_length(&self) -> usize {
        let mut count = 0;
        let mut escape = false;
        let mut hex = 0;

        for c in self.inner[1..self.inner.len() - 1].chars() {
            if c == '\\' {
                if escape {
                    // for double \\
                    count += 1;
                    escape = false;
                } else {
                    escape = true;
                }
            } else if escape && c == 'x' {
                hex = 2;
            } else {
                if hex == 2 {
                    hex -= 1;
                    continue;
                } else if hex == 1 {
                    hex = 0;
                    escape = false;
                } else if escape {
                    escape = false;
                }
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mystring_encoded_length() {
        assert_eq!(MyString::from(r#""""#).encoded_length(), 6);
        assert_eq!(MyString::from(r#""abc""#).encoded_length(), 9);
        assert_eq!(MyString::from(r#""aaa\"aaa""#).encoded_length(), 16);
        assert_eq!(MyString::from(r#""\x27""#).encoded_length(), 11);
    }

    #[test]
    fn test_mystring_code_minus_memory() {
        let s = r#"""
"abc"
"aaa\"aaa"
"\x27"
"#;
        let list = SantasList::from(&s);
        assert_eq!(list.code_minus_memory(), 12);
    }

    #[test]
    fn test_mystring_code_length() {
        assert_eq!(MyString::from(r#""""#).code_length(), 2);
        assert_eq!(MyString::from(r#""abc""#).code_length(), 5);
        assert_eq!(MyString::from(r#""aaa\"aaa""#).code_length(), 10);
        assert_eq!(MyString::from(r#""\x27""#).code_length(), 6);
    }

    #[test]
    fn test_mystring_memory_length() {
        assert_eq!(MyString::from(r#""""#).memory_length(), 0);
        assert_eq!(MyString::from(r#""abc""#).memory_length(), 3);
        assert_eq!(MyString::from(r#""aaa\"aaa""#).memory_length(), 7);
        assert_eq!(MyString::from(r#""\x27""#).memory_length(), 1);

        assert_eq!(MyString::from(r#""aaa\\aaa""#).memory_length(), 7);
    }
}

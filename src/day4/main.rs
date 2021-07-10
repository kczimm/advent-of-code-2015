/*!
--- Day 4: The Ideal Stocking Stuffer ---

Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts for all the economically forward-thinking little girls and boys.

To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes. The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a number in decimal. To mine AdventCoins, you must find Santa the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces such a hash.

For example:

    If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043 starts with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
    If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash starting with five zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970 looks like 000006136ef....

--- Part Two ---

Now find one that starts with six zeroes.

*/
use md5;
use std::io;

fn main() -> io::Result<()> {
    let secret: SecretKey = "iwrupvqb";

    println!("part1: {}", lowest_number_with_n_leading_zeros(secret, 5));
    println!("part2: {}", lowest_number_with_n_leading_zeros(secret, 6));

    Ok(())
}

fn lowest_number_with_n_leading_zeros(secret: SecretKey, n: usize) -> Answer {
    let mut answer = 0;

    while !has_n_leading_zeros(&make_hash(secret, answer), n) {
        answer += 1;
    }

    answer
}

type SecretKey<'a> = &'a str;
type Answer = usize;

fn make_hash(secret: SecretKey, answer: Answer) -> String {
    format!("{:x}", md5::compute(format!("{}{}", secret, answer)))
}

fn has_n_leading_zeros(s: &str, n: usize) -> bool {
    if s.len() < n {
        false
    } else {
        s.chars()
            .enumerate()
            .filter(|(i, _)| *i < n)
            .all(|(_, c)| c == '0')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_five_leading_zeros() {
        let secret: SecretKey = "abcdef";
        let answer = 609043;
        assert!(has_n_leading_zeros(&make_hash(secret, answer), 5));

        let secret: SecretKey = "pqrstuv";
        let answer = 1048970;
        assert!(has_n_leading_zeros(&make_hash(secret, answer), 5));
    }

    #[test]
    fn test_lowest_number_with_five_leading_zeros() {
        let secret: SecretKey = "abcdef";
        assert_eq!(lowest_number_with_n_leading_zeros(secret, 5), 609043);

        let secret: SecretKey = "pqrstuv";
        assert_eq!(lowest_number_with_n_leading_zeros(secret, 5), 1048970);
    }
}

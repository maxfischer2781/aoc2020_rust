use std::str::FromStr;
use std::num::ParseIntError;
use std::io::Error;
use crate::utility::parse_file_lines;

pub fn solve() -> Result<(), Error> {
    let values: Vec<PolicyPassword> = parse_file_lines("data/day2.txt")?;
    let range_count = values.iter().filter(|p| p.in_range()).count();
    let index_count = values.iter().filter(|p| p.in_position()).count();
    println!("Range count {}", range_count);
    println!("Index count {}", index_count);
    Ok(())
}

#[derive(Debug)]
struct PolicyPassword {
    min: usize,
    max: usize,
    symbol: String,
    password: String,
}

impl PolicyPassword {
    /// Whether `symbol` appears between `min` and `max` times
    fn in_range(&self) -> bool {
        let count = self.password.matches(&self.symbol).count();
        self.min <= count && count <= self.max
    }

    /// Whether `symbol` appears either at the `min`'th or `max`'th position
    fn in_position(&self) -> bool {
        let password_chars: Vec<char> = self.password.chars().collect();
        (String::from(password_chars[self.min - 1]) == self.symbol) ^ (String::from(password_chars[self.max - 1]) == self.symbol)
    }
}

impl FromStr for PolicyPassword {
    type Err = ParseIntError;

    /// Parse a string such as `3-7 x: xjxbgpxxgtx`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        let range: Vec<&str> = parts[0].split("-").collect();
        Ok(PolicyPassword {
            min: range[0].parse()?,
            max: range[1].parse()?,
            symbol: String::from(&parts[1][..parts[1].len() - 1]),
            password: String::from(parts[2]),
        })
    }
}
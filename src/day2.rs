use std::ops::Range;
use std::str::FromStr;
use std::num::ParseIntError;
use std::io::Error;
use crate::utility::parse_file_lines;

pub fn solve() -> Result<(), Error> {
    let values: Vec<PolicyPassword> = parse_file_lines("data/day2_2/input.txt")?;
    for pw in &values {
        println!("{:?} ({}) ", pw, pw.is_valid());
        println!("{}", pw.password.matches(&pw.symbol).count())
    }
    let valid = values.iter().filter(|p| p.is_valid()).count();
    println!("Valid count {}", valid);
    Ok(())
}

#[derive(Debug)]
struct PolicyPassword {
    range: Range<usize>,
    symbol: String,
    password: String,
}

impl PolicyPassword {
    fn is_valid(&self) -> bool {
        self.range.contains(
            &self.password.matches(&self.symbol).count()
        )
    }
}

impl FromStr for PolicyPassword {
    type Err = ParseIntError;

    /// Parse a string such as `3-7 x: xjxbgpxxgtx`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        let range: Vec<&str> = parts[0].split("-").collect();
        let min= range[0].parse()?;
        let max: usize = range[1].parse()?;
        Ok(PolicyPassword {
            range: Range{ start: min, end: 1 + max},
            symbol: String::from(&parts[1][..parts[1].len() - 1]),
            password: String::from(parts[2]),
        })
    }
}
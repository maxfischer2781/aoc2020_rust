use std::str::FromStr;
use std::io::Error;
use std::collections::BTreeMap;

use crate::utility::parse_file_lines;

pub fn solve() -> Result<(), Error> {
    let passports = compile_passports(&parse_file_lines("data/day4.txt")?);
    let no_complete = passports.iter().filter(|pp| pp.is_complete()).count();
    println!("No compl {}", no_complete);
    let no_valid = passports.iter().filter(|pp| pp.is_valid()).count();
    println!("No valid {}", no_valid);
    Ok(())
}

/// Merge consecutive passport data
fn compile_passports(partials: &Vec<PartialPassport>) -> Vec<PartialPassport> {
    let mut compiled = vec![];
    let mut current = PartialPassport::new();
    for partial in partials.iter() {
        if partial.is_empty() && !current.is_empty() {
            compiled.push(current);
            current = PartialPassport::new();
        }
        else {
            current = current.merge(&partial)
        }
    }
    if !current.is_empty() {
        compiled.push(current);
    }
    compiled
}

// Helpers for validating fields
fn is_inrange(literal: &String, min: i32, max: i32) -> bool {
    match literal.parse::<i32>() {
        Err(_) => false,
        Ok(x) => min <= x && x <= max,
    }
}

#[derive(Debug)]
struct PartialPassport (BTreeMap<String, String>);


impl PartialPassport {
    fn new() -> PartialPassport {
        PartialPassport(BTreeMap::new())
    }

    /// Whether any fields are present
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Whether all required fields are present
    fn is_complete(&self) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().all(
            |required_key| self.0.contains_key(*required_key)
        )
    }

    /// Whether all required fields hold valid values
    fn is_valid(&self) -> bool {
        self.is_complete() &&
            // byr (Birth Year) - four digits; at least 1920 and at most 2002.
            is_inrange(&self.0["byr"], 1920, 2002) &&
            // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            is_inrange(&self.0["iyr"], 2010, 2020) &&
            // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
            is_inrange(&self.0["eyr"], 2020, 2030) &&
            // hgt (Height) - a number followed by either cm or in:
            match &self.0["hgt"].split_at(self.0["hgt"].len() - 2) {
                // If cm, the number must be at least 150 and at most 193.
                (x, "cm") => is_inrange(&String::from(*x), 150, 193),
                // If in, the number must be at least 59 and at most 76.
                (x, "in") => is_inrange(&String::from(*x), 59, 76),
                _ => false,
            } &&
            // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            self.0["hcl"].starts_with('#') &&
            self.0["hcl"].len() == 7 &&
            self.0["hcl"][1..].chars().all(|c| "0123456789abcdef".contains(c)) &&
            // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|color| *color == &self.0["ecl"]) &&
            // pid (Passport ID) - a nine-digit number, including leading zeroes.
            self.0["pid"].len() == 9 &&
            is_inrange(&self.0["pid"], 0, 999999999)
    }

    /// Merge two partial passports to a new one
    fn merge(&self, other: &PartialPassport) -> PartialPassport {
        let m = self.0.iter().chain(&other.0).map(|(k, v)| (k.clone(), v.clone())).collect();
        PartialPassport(m)
    }
}


impl FromStr for PartialPassport {
    type Err = ();

    /// Read a literal such as `hcl:#cfa07d byr:1929`
    fn from_str(s: &str)  -> Result<Self, Self::Err> {
        let mut new_self = PartialPassport::new();
        for key_value in s.split_whitespace() {
            let (key, value) = key_value.split_at(3);
            new_self.0.insert(String::from(key), String::from(&value[1..]));
        }
        Ok(new_self)
    }
}
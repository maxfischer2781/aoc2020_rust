use std::str::FromStr;
use std::io::Error;
use std::collections::BTreeMap;

use crate::utility::parse_file_lines;

pub fn solve() -> Result<(), Error> {
    let passports = compile_passports(&parse_file_lines("data/day4.txt")?);
    let valids = passports.iter().filter(|pp| pp.is_complete()).count();
    println!("No valid {}", valids);
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
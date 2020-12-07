use std::fs::File;
use std::io::{BufRead, Error, BufReader};
use std::str::FromStr;
use std::fmt::Debug;

/// Parse lines from a buffer to a specific type T
pub fn parse_lines<T: FromStr, R: BufRead>(in_stream: R) -> Result<Vec<T>, Error> where <T as std::str::FromStr>::Err: Debug{
    let mut results = vec![];
    for line in in_stream.lines() {
        results.push(line?.trim().parse().unwrap())
    }
    Ok(results)
}

pub fn parse_file_lines<T: FromStr>(path: &str) -> Result<Vec<T>, Error> where <T as std::str::FromStr>::Err: Debug{
    let f = File::open(&path)?;
    let values: Vec<T> = parse_lines(BufReader::new(f))?;
    Ok(values)
}

pub fn partition(s: &str, by: &str) -> (String, Option<String>) {
    match s.find(&by) {
        Some(i) => (String::from(&s[..i]), Some(String::from(&s[i + by.len()..]))),
        None => (String::from(s), None),
    }
}
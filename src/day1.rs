use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::str::{FromStr};
use std::fmt::{Debug};

pub fn solve() -> Result<(), Error> {
    let f = File::open("data/day1.txt")?;
    let values: Vec<i32> = read(BufReader::new(f))?;
    match search_double(&values, 2020) {
        Some(x) => {
            let (a, b) = x;
            println!("{} * {} = {}", a, b, a * b)
        },
        None => println!("No Match!")
    };
    match search_triple(&values, 2020) {
        Some(x) => {
            let (a, b, c) = x;
            println!("{} * {} * {} = {}", a, b, c, a * b * c)
        },
        None => println!("No Match!")
    };
    Ok(())
}

fn search_double(candidates: &Vec<i32>, total: i32) -> Option<(&i32, &i32)> {
    for a in candidates {
        for b in candidates {
            if a + b == total {
                return Some((a, b));
            }
        }
    }
    return None;
}

fn search_triple(candidates: &Vec<i32>, total: i32) -> Option<(&i32, &i32, &i32)> {
    for a in candidates {
        for b in candidates {
            for c in candidates {
                if a + b + c == total {
                    return Some((a, b, c));
                }
            }
        }
    }
    return None;
}


fn read<T: FromStr, R: BufRead>(in_stream: R) -> Result<Vec<T>, Error> where <T as std::str::FromStr>::Err: Debug{
    let mut results = vec![];
    for line in in_stream.lines() {
        results.push(line?.trim().parse().unwrap())
    }
    Ok(results)
}

use std::io::Error;
use std::iter::once;

use crate::utility::parse_file_lines;


pub fn solve() -> Result<(), Error> {
    let connectors: Vec<i64> = parse_file_lines("data/day10.txt")?;
    let steps = shortest_steps(connectors.clone());
    let step_counts = count(&steps);
    println!("Differences {}", step_counts.0 * step_counts.1);
    println!("Variations {}", variations(&steps));
    Ok(())
}

fn shortest_steps(mut connectors: Vec<i64>) -> Vec<i64> {
    connectors.sort();
    // precede total-0 for the outlet, append step-3 for the device
    once(&0).chain(&connectors).zip(&connectors).map(
        |(prev, next)| next - prev
    ).chain(once(3)).collect()
}

fn count(steps: &Vec<i64>) -> (i64, i64) {
    steps.iter().fold(
        (0, 0), |acc, step| match step {
            1 => (acc.0 + 1, acc.1),
            3 => (acc.0, acc.1 + 1),
            x => {println!("Discard {}", x); acc},
        }
    )
}

// # To find the number of variations, it is enough to look at the steps instead of the values:
// If we have the steps 3111, we can shorten it to 33, 321, 312 without caring about the values.
// Notably, we only need to look at sequences of 1s and 2s; sequences of 3s can never be shortened.
// Conveniently, there are no 2s in the data. It is enough to know the sequences of 1s and to count
// their variations.
// 11 +> 2
// 111 +> 3, 12, 21
// 1111 +> 31, 211, 22, 121, 112, 13
// 11111 +> 311, 131, 113, 32, 23, 2111, 1211, 1121, 1112, 221, 212, 122
// Given that we can only replace 11 and 111, we can work from one end and consume the
// variations of the remainder.
fn variations(steps: &Vec<i64>) -> i64 {
    // variations for a given length. cost[3] => Cost of 111
    let mut cost = vec![1, 1, 2, 4];
    let mut total = 1;
    // indices of 3-steps
    let starts: Vec<_> = steps.iter().enumerate().filter(
        |(_, &delta)| delta == 3
    ).map(|(i, _)| i).collect();
    for (prev_i, curr_i) in once(&0).chain(&starts).zip(&starts) {
        let length = if prev_i == &0usize {*curr_i} else {curr_i - prev_i - 1};
        total *= match cost.get(length) {
            Some(x) => x,
            None => {
                for sub_length in cost.len()..length+1 {
                    cost.push(2 * cost[sub_length-1] + cost[sub_length-2] + cost[sub_length-3])
                };
                &cost[length]
            }
        };
    }
    total
}

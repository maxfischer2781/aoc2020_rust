use std::io::Error;
use std::collections::{VecDeque, HashSet};

use crate::utility::parse_file_lines;


pub fn solve() -> Result<(), Error> {
    let series: Vec<i64> = parse_file_lines("data/day9.txt")?;
    if let Some(outlier) = find_nonsum(&series, 25) {
        println!("Non-sum {}", outlier);
        if let Some((min, max)) = find_sum(&series, &outlier) {
            println!("Pair-sum {} = {} + {}", min + max, min, max);
        }
    }
    Ok(())
}

/*
Both problems ("item not the sum of previous 25 items" and "items with sum of outlier")
come down to finding a consecutive window in the input series. Thus, the core of both
solutions is a sliding window, growing for each new item and removing the oldest item(s).

In general, the approach is "add new, pop until condition". In the first case, that condition
is "``window.len() == window_size``" aka we always pop for each push. In the second case, that
condition is "``window.sum() <= total``"; this means remove elements until adding any element
might solve the problem.
*/


fn find_nonsum(series: &Vec<i64>, window_size: usize) -> Option<i64> {
    let mut window: VecDeque<i64> = series.iter().take(window_size).cloned().collect();
    for item in series.iter().skip(window_size) {
        let current_window: HashSet<i64> = window.iter().cloned().collect();
        if current_window.iter().any(
            |part1| current_window.contains(&(item - part1)) && &(item - part1) != part1
        ) {
            window.pop_front();
            window.push_back(item.clone());
        }
        else {
            return Some(item.clone());
        }
    }
    None
}

fn find_sum(series: &Vec<i64>, total: &i64) -> Option<(i64, i64)> {
    let mut window: VecDeque<i64> = series.iter().take(1).cloned().collect();
    let mut current_sum: i64;
    for item in series.iter().skip(1) {
        window.push_back(item.clone());
        current_sum = window.iter().sum();
        while &current_sum > total {
            if let Some(x) = window.pop_front(){
                current_sum -= x;
            }
            else {
                // only happens if total is negative
                return None;
            }
        }
        if &current_sum == total && window.len() >= 2 {
            // return the largest and smallest member
            window.make_contiguous().sort();
            return Some((window.front().unwrap().clone(), window.back().unwrap().clone()));
        }
    }
    None
}

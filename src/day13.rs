use std::io::Error;
use crate::utility::parse_file_lines;


pub fn solve() -> Result<(), Error> {
    let timetable: Vec<String> = parse_file_lines("data/day13.txt")?;
    let (departure, bus_ids) = parse_part1(&timetable);
    let (earliest_wait, earliest_id) = find_bus(departure, bus_ids);
    println!("Bus {} departs after {} => {}", earliest_id, earliest_wait, earliest_id * earliest_wait);
    let buses = parse_part2(&timetable);
    let earliest_common = common_time(&buses);
    println!("Earliest common time {}", earliest_common);
    Ok(())
}

fn parse_part1(timetable: &Vec<String>) -> (u64, Vec<u64>) {
    let departure = timetable.first().unwrap().parse().unwrap();
    let bus_ids = timetable.iter().skip(1).next().unwrap().split(',').filter(
        |&field| field != "x"
    ).map(
        |field| field.parse().unwrap()
    ).collect();
    (departure, bus_ids)
}

fn find_bus(departure: u64, bus_ids: Vec<u64>) -> (u64, u64) {
    bus_ids.iter().map(|bus_id| (bus_id - departure % bus_id, *bus_id)).min().unwrap()
}

// Approach:
// Use the "Chinese Remainder Theorem" https://en.wikipedia.org/wiki/Chinese_remainder_theorem
// The idea is to match up buses pairwise: Once you have a common time t_{ab} for them, any
// time t_{ab} + n * lcm(a, b) also matches them. This allows searching the next bus at every
// time t_{abc} = t_{ab} + n_{abc}*lcm(a, b), which is then stable for t_{abc} + n * lcm(a, b, c).
// This is doable if you have a lcm = Least Common Multiple function.
// See https://en.wikipedia.org/wiki/Greatest_common_divisor for algorithms.
// NB: The input appears to be all primes. In other words, lcm(a, b) == a * b. There is no
// guarantee for that, so using the pedantic variant.
#[derive(Debug)]
struct Bus {
    number: u64,
    offset: u64,
}

fn parse_part2(timetable: &Vec<String>) -> Vec<Bus> {
    (0..).zip(timetable.iter().skip(1).next().unwrap().split(',')).filter(
        |&field| field.1 != "x"
    ).map(
        |field| Bus {offset: field.0, number: field.1.parse().unwrap()}
    ).collect()
}

/// greatest common divisor for a pair of numbers
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
      gcd(b, a % b)
    }
}

/// least common multiple for a pair of numbers
fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

/// Chinese Remainder Theorem: https://en.wikipedia.org/wiki/Chinese_remainder_theorem
fn common_time(buses: &Vec<Bus>) -> u64 {
    let (mut timestamp, mut step) = match buses.first() {
        Some(first) => {
            (0, first.number)
        },
        None => (0, 1)
    };
    for bus in buses.iter().skip(1) {
        while (timestamp + bus.offset) % bus.number != 0 {
            timestamp += step
        }
        step = lcm(step, bus.number)
    }
    timestamp % step
}

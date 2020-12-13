use std::io::Error;
use crate::utility::parse_file_lines;


pub fn solve() -> Result<(), Error> {
    let timetable: Vec<String> = parse_file_lines("data/day13.txt")?;
    let (departure, bus_ids) = parse_part1(&timetable);
    let (earliest_wait, earliest_id) = find_bus(departure, bus_ids);
    println!("Bus {} departs after {} => {}", earliest_id, earliest_wait, earliest_id * earliest_wait);
    Ok(())
}

fn parse_part1(timetable: &Vec<String>) -> (i64, Vec<i64>) {
    let departure = timetable.first().unwrap().parse().unwrap();
    let bus_ids = timetable.iter().skip(1).next().unwrap().split(',').filter(
        |&field| field != "x"
    ).map(
        |field| field.parse().unwrap()
    ).collect();
    (departure, bus_ids)
}

fn find_bus(departure: i64, bus_ids: Vec<i64>) -> (i64, i64) {
    bus_ids.iter().map(|bus_id| (bus_id - departure % bus_id, *bus_id)).min().unwrap()
}

// insert part 2 here if I ever feel like it...
// Approach:
// Use the "Chinese Remainder Theorem" https://en.wikipedia.org/wiki/Chinese_remainder_theorem
// This is easy if you have a Least Common Multiple function over arbitrary many inputs.
// See https://en.wikipedia.org/wiki/Greatest_common_divisor for algorithms.

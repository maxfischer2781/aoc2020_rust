use std::env;
use std::io::Error;
use std::time::{Instant, Duration};

mod utility;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;


/// Format a duration nicely
fn format_duration(delta: Duration) -> String {
    let mut fractional = delta.as_secs_f64();
    for symbol in ["s", "ms", "μs", "ns"].iter() {
        if fractional > 0.5 {
            return format!("{:.2} {}", fractional, symbol);
        }
        else {
            fractional *= 1000.0;
        }
    }
    format!("{:?}", delta)
}


fn run_solver(solver: fn() -> Result<(), Error>) -> Result<(), Error> {
    let pre = Instant::now();
    solver()?;
    let end = Instant::now();
    println!("[> Elapsed {} <]", format_duration(end-pre));
    Ok(())
}

fn main() -> Result<(), Error> {
    let default_day = String::from("13");
    let arguments: Vec<String> = env::args().collect();
    let solver = match arguments.get(1).unwrap_or(&default_day).as_str() {
        "1" => day1::solve,
        "2" => day2::solve,
        "3" => day3::solve,
        "4" => day4::solve,
        "5" => day5::solve,
        "6" => day6::solve,
        "7" => day7::solve,
        "8" => day8::solve,
        "9" => day9::solve,
        "10" => day10::solve,
        "11" => day11::solve,
        "12" => day12::solve,
        "13" => day13::solve,
        x => panic!("Unknown Exercise {}", x)
    };
    run_solver(solver)?;
    Ok(())
}

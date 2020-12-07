use std::env;
use std::io::Error;

mod utility;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() -> Result<(), Error> {
    let default_day = String::from("7");
    let arguments: Vec<String> = env::args().collect();
    match arguments.get(1).unwrap_or(&default_day).as_str() {
        "1" => day1::solve()?,
        "2" => day2::solve()?,
        "3" => day3::solve()?,
        "4" => day4::solve()?,
        "5" => day5::solve()?,
        "6" => day6::solve()?,
        "7" => day7::solve()?,
        x => println!("Unknown Exercise {}", x)
    }
    Ok(())
}

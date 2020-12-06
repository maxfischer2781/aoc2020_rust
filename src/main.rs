use std::env;
use std::io::Error;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod utility;

fn main() -> Result<(), Error> {
    let default_day = String::from("5");
    let arguments: Vec<String> = env::args().collect();
    match arguments.get(1).unwrap_or(&default_day).as_str() {
        "1" => day1::solve()?,
        "2" => day2::solve()?,
        "3" => day3::solve()?,
        "4" => day4::solve()?,
        "5" => day5::solve()?,
        x => println!("Unknown Exercise {}", x)
    }
    Ok(())
}

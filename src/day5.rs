use std::str::FromStr;
use std::io::Error;

use crate::utility::parse_file_lines;

pub fn solve() -> Result<(), Error> {
    let mut seats: Vec<Seat> = parse_file_lines("data/day5.txt")?;
    seats.sort_by_key(|s| s.id());
    let max_seat = seats.last();
    println!("Max seat id {}", max_seat.unwrap().id());
    for (prev, next) in seats[..seats.len() - 1].iter().zip(&seats[1..]){
        if next.id() - prev.id() == 2 {
            println!("Mid seat id {}", next.id() - 1)
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Seat{
    row: i16,
    column: i16,
}


impl Seat {
    fn id(&self) -> i16 {
        self.row * 8 + self.column
    }
}



impl FromStr for Seat {
    type Err = ();

    /// Read a literal such as `BFFFBBFRRR` to `70, 7`
    fn from_str(s: &str)  -> Result<Self, Self::Err> {
        let row = (0..).zip(s.chars().take(7)).map(
            |(index, fb)| if fb == 'B' {2i16.pow(6 - index)} else {0}
        ).sum::<i16>();
        let column = (0..).zip(s.chars().skip(7).take(3)).map(
            |(index, fb)| if fb == 'R' {2i16.pow(2 - index)} else {0}
        ).sum::<i16>();
        Ok(Seat {row, column})
    }
}
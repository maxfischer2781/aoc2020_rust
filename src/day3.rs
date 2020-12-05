use std::str::FromStr;
use std::io::Error;
use std::fmt;
use crate::utility::parse_file_lines;


pub fn solve() -> Result<(), Error> {
    let map: Vec<MapLine> = parse_file_lines("data/day3.txt")?;
    println!("Hits simple {}", sum_slope(&map, 3, 1));
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let slope_hits = slopes.iter().map(
        |(r, d)| sum_slope(&map, *r, *d)
    ).product::<i64>();
    println!("Hits many {}", slope_hits);
    Ok(())
}

fn sum_slope(map: &Vec<MapLine>, right: usize, down: usize) -> i64 {
    let mut hits = 0;
    for (index, line) in map.iter().enumerate() {
        match (line.is_tree(index / down * right), index % down) {
            (true, 0) => hits += 1,
            _ => (),
        }
    };
    hits
}


#[derive(Debug)]
struct MapLine(Vec<bool>);

impl MapLine {
    /// Whether there is a tree at `i`
    fn is_tree(&self, i: usize) -> bool {
        self.0[i % self.0.len()]
    }
}

impl fmt::Display for MapLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fields: Vec<&str> = self.0.iter().map(|b| match b {true => "#", _ => "."}).collect();
        write!(f, "[{}]", fields.join(""))
    }
}

impl FromStr for MapLine {
    type Err = ();

    /// Read a literal such as `.........#....#.###.........##.`
    fn from_str(s: &str)  -> Result<Self, Self::Err> {
        Ok(MapLine(
            s.chars().map(|c| c == '#').collect()
        ))
    }
}
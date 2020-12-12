use std::str::FromStr;
use std::ops::{Add, Mul, AddAssign};
use std::num::ParseIntError;
use std::io::Error;
use crate::utility::parse_file_lines;

pub fn solve() -> Result<(), Error> {
    let directions: Vec<Instruction> = parse_file_lines("data/day12.txt")?;
    let destination_nav = navigate(&directions);
    println!("Distance {}", destination_nav.manhattan());
    let destination_way = direct(&directions);
    println!("Distance {}", destination_way.manhattan());
    Ok(())
}

#[derive(Debug)]
struct Instruction(char, i32);

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (identifier, number) = s.split_at(1);
        Ok(Self (identifier.chars().next().unwrap(), number.parse()?))
    }
}

/// 2D Vector, with x pointing right and y pointing up
#[derive(Debug)]
struct Vector2{
    x: i32,
    y: i32,
}

enum Rotation {LEFT, RIGHT, FLIP}

impl Vector2 {
    const fn right() -> Self { Self {x: 1, y: 0} }
    const fn left() -> Self { Self {x: -1, y: 0} }
    const fn up() -> Self { Self {x: 0, y: 1} }
    const fn dow() -> Self { Self {x: 0, y: -1} }

    const fn zero() -> Self { Self {x: 0, y: 0} }

    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn rotate(&self, by: &Rotation) -> Self {
        match by {
            Rotation::LEFT => Self {x: -self.y, y: self.x},
            Rotation::RIGHT => Self {x: self.y, y: -self.x},
            Rotation::FLIP => Self {x: -self.x, y: -self.y},
        }
    }
}

impl Add for &Vector2 {
    type Output = Vector2;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Mul<&i32> for &Vector2 {
    type Output = Vector2;

    fn mul(self, other: &i32) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

fn navigate(directions: &Vec<Instruction>) -> Vector2 {
    let (mut position, mut orientation) = (Vector2::zero(), Vector2::right());
    for instruction in directions {
        match instruction {
            Instruction('N', amount) => position += &Vector2::up() * amount,
            Instruction('S', amount) => position += &Vector2::dow() * amount,
            Instruction('E', amount) => position += &Vector2::right() * amount,
            Instruction('W', amount) => position += &Vector2::left() * amount,
            Instruction('F', amount) => position += &orientation * amount,
            Instruction('R', amount) => match amount % 360 {
                90 => orientation = orientation.rotate(&Rotation::RIGHT),
                180 => orientation = orientation.rotate(&Rotation::FLIP),
                270 => orientation = orientation.rotate(&Rotation::LEFT),
                _ => (),
            },
            Instruction('L', amount) => match amount % 360 {
                90 => orientation = orientation.rotate(&Rotation::LEFT),
                180 => orientation = orientation.rotate(&Rotation::FLIP),
                270 => orientation = orientation.rotate(&Rotation::RIGHT),
                _ => (),
            },
            smt => panic!("Unknown instruction {:?}", smt),
        }
    }
    return position;
}


fn direct(directions: &Vec<Instruction>) -> Vector2 {
    let mut position = Vector2::zero();
    let mut waypoint = &Vector2::up() + &(&Vector2::right() * &10);
    for instruction in directions {
        match instruction {
            Instruction('N', amount) => waypoint += &Vector2::up() * amount,
            Instruction('S', amount) => waypoint += &Vector2::dow() * amount,
            Instruction('E', amount) => waypoint += &Vector2::right() * amount,
            Instruction('W', amount) => waypoint += &Vector2::left() * amount,
            Instruction('F', amount) => position += &waypoint * amount,
            Instruction('R', amount) => match amount % 360 {
                90 => waypoint = waypoint.rotate(&Rotation::RIGHT),
                180 => waypoint = waypoint.rotate(&Rotation::FLIP),
                270 => waypoint = waypoint.rotate(&Rotation::LEFT),
                _ => (),
            },
            Instruction('L', amount) => match amount % 360 {
                90 => waypoint = waypoint.rotate(&Rotation::LEFT),
                180 => waypoint = waypoint.rotate(&Rotation::FLIP),
                270 => waypoint = waypoint.rotate(&Rotation::RIGHT),
                _ => (),
            },
            smt => panic!("Unknown instruction {:?}", smt),
        }
    }
    return position;
}

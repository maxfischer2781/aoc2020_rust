use std::io::Error;
use std::iter::{repeat, once};

use crate::utility::parse_file_lines;


pub fn solve() -> Result<(), Error> {
    let rows: Vec<String> = parse_file_lines("data/day11.txt")?;
    let seats = Seats::from_rows(rows);
    let stable_seats = seats.clone().stable_neighbours(false);
    println!("Neighbour count {}", stable_seats.occupied());
    let stable_seats = seats.stable_neighbours(true);
    println!("Visible count {}", stable_seats.occupied());
    Ok(())
}

/// 2D Vector of seats
/// This is padded on all directions, with a row/column of empty positions
#[derive(PartialEq)]
#[derive(Clone)]
struct Seats{
    /// Whether a seat is at row/column
    positions: Vec<Vec<bool>>,
    /// Whether some is at row/column
    occupancy: Vec<Vec<bool>>,
    /// Which seats are seen by which seat
    visibility: Vec<Vec<[(usize, usize); 8]>>,
}

impl Seats {
    /// Compute the visibility of seats for each seat
    fn _visibility(positions: &Vec<Vec<bool>>) -> Vec<Vec<[(usize, usize); 8]>> {
        let mut visibility = vec![vec![[(0usize, 0usize);8]; positions[0].len()]; positions.len()];
        for (row_i, row) in positions.iter().enumerate() {
            for (column_i, seat) in row.iter().enumerate() {
                if *seat {
                    visibility[row_i][column_i] = Seats::_seat_visiblity(
                        &positions, row_i, column_i
                    );
                }
            }
        };
        visibility
    }

    fn _seat_visiblity(positions: &Vec<Vec<bool>>, row_i: usize, column_i: usize) -> [(usize, usize); 8] {
        let (max_row, max_column) = (positions.len() - 1, positions[0].len() - 1);
        // up left, up, up right, right, down right, down, down left, left
        // 0:0 is always empty, like any other border field
        let mut visibility = [(0usize, 0usize);8];
        for (up_i, left_i) in (0..row_i).rev().zip((0..column_i).rev()) {
            if positions[up_i][left_i] {
                visibility[0] = (up_i, left_i);
                break
            }
        }
        for up_i in (0..row_i).rev() {
            if positions[up_i][column_i] {
                visibility[1] = (up_i, column_i);
                break
            }
        }
        for (up_i, right_i) in (0..row_i).rev().zip(column_i+1..max_column) {
            if positions[up_i][right_i] {
                visibility[2] = (up_i, right_i);
                break
            }
        }
        for right_i in column_i+1..max_column {
            if positions[row_i][right_i] {
                visibility[3] = (row_i, right_i);
                break
            }
        }
        for (down_i, right_i) in (row_i+1..max_row).zip(column_i+1..max_column) {
            if positions[down_i][right_i] {
                visibility[4] = (down_i, right_i);
                break
            }
        }
        for down_i in row_i+1..max_row {
            if positions[down_i][column_i] {
                visibility[5] = (down_i, column_i);
                break
            }
        }
        for (down_i, left_i) in (row_i+1..max_row).zip((0..column_i).rev()) {
            if positions[down_i][left_i] {
                visibility[6] = (down_i, left_i);
                break
            }
        }
        for left_i in (0..column_i).rev() {
            if positions[row_i][left_i] {
                visibility[7] = (row_i, left_i);
                break
            }
        }
        visibility
    }

    fn from_rows(rows: Vec<String>) -> Self {
        let (num_rows, num_colums) = (rows.len(), rows[0].len());
        let occupancy: Vec<Vec<bool>> = repeat(
                repeat(false).take(num_colums + 2).collect()
            ).take(num_rows + 2).collect();
        let mut positions = Vec::new();
        // there are no seats at the padding; add it anyway for consistency
        positions.push(repeat(false).take(num_colums + 2).collect());
        for row in rows {
            positions.push(
                once(false).chain(row.chars().map(
                    |c| c == 'L'
                ).chain(once(false))).collect()
            )
        };
        positions.push(repeat(false).take(num_colums + 2).collect());
        let visibility= Seats::_visibility(&positions);
        Self {positions, occupancy, visibility}
    }

    /// Evolve seats until the layout is stable
    fn stable_neighbours(self, visible: bool) -> Self {
        let mut prev = self.occupancy.clone();
        let mut next = self.evolve_neighbours(visible);
        while next.occupancy != prev {
            prev = next.occupancy.clone();
            next = next.evolve_neighbours(visible);
        }
        next
    }

    /// Compute the next layout using neighbor or visible rules
    fn evolve_neighbours(self, visible: bool) -> Self {
        let threshold = if visible {5} else {4};
        let mut new_occupancy: Vec<Vec<bool>> = repeat(
            vec![false; self.positions[0].len()]
        ).take(self.occupancy.len()).collect();
        for (row_i, row) in self.positions.iter().enumerate() {
            for (column_i, seat) in row.iter().enumerate() {
                if *seat {
                    // We do not actually have to check whether the seat itself is occupied:
                    // The result is always the same.
                    let neighbours = if visible {
                        self.visible(row_i, column_i)
                    } else {
                        self.neighbours(row_i, column_i)
                    };
                    // If a seat is empty (L) and there are no occupied seats adjacent to it,
                    // the seat becomes occupied.
                    if neighbours == 0 {
                        new_occupancy[row_i][column_i] = true;
                    }
                    // If a seat is occupied (#) and four or more seats adjacent to it are also
                    // occupied, the seat becomes empty.
                    else if neighbours < threshold {
                        new_occupancy[row_i][column_i] = self.occupancy[row_i][column_i]
                    }
                }
            }
        }
        Self {positions: self.positions, occupancy: new_occupancy, visibility: self.visibility}
    }

    /// Number of occupied seats next to the one at `row_i`, `column_i`
    fn neighbours(&self, row_i: usize, column_i: usize) -> usize {
        let occupancy = &self.occupancy;
        // NB: Iterating over offsets might be better, but I got fed up with signed - usize math :/
        [
            occupancy[row_i - 1][column_i - 1],
            occupancy[row_i - 1][column_i],
            occupancy[row_i - 1][column_i + 1],
            occupancy[row_i][column_i - 1],
            occupancy[row_i][column_i + 1],
            occupancy[row_i + 1][column_i - 1],
            occupancy[row_i + 1][column_i],
            occupancy[row_i + 1][column_i + 1],
        ].iter().filter(|&occupied| *occupied).count()
    }

    /// Number of occupied seats visible from the one at `row_i`, `column_i`
    fn visible(&self, row_i: usize, column_i: usize) -> usize {
        self.visibility[row_i][column_i].iter().map(
            |(row, col)| self.occupancy[*row][*col]
        ).filter(|&b| b).count()
    }

    /// Number of occupied seats in total
    fn occupied(&self) -> usize {
        self.occupancy.iter().flatten().filter(|&seat| *seat).count()
    }
}
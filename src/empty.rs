//! This module is dedicated to the creation of valid empty boards. See empty::Maker.

use std::cmp;

use crate::base::{Board, Cell};
use crate::filter;
use crate::tools::{fill, stats};

/// Makes all  unique empty boards of the given size.
pub struct Maker {
    /// Next value used to determine the pattern of floor and wall tiles. Range [0, self.combinations).
    seed: usize,
    /// Number of rows (or columns) in each board.
    size: usize,
    /// Total number of floor & wall combinations possible in this board size (2 ^ (size * size)).
    combinations: usize,
    /// Used to remove duplicates under symmetry.
    filter: filter::Symmetries,
}

impl Maker {
    /// Returns a new Maker for the given board size.
    pub fn new(size: usize) -> Maker {
        return Maker {
            seed: 0,
            size: size,
            combinations: 1 << (size * size),
            filter: filter::Symmetries::new(),
        };
    }

    /// Converts a seed into a combination of floor and wall cells; full set of seeds covers all possible combinations.
    fn make_board(&self) -> Board {
        let mut board = Vec::new();
        for y in 0..self.size {
            let mut row = Vec::new();
            for x in 0..self.size {
                let index = y * self.size + x;
                let mask = (self.seed >> index) & 1;
                let cell = if mask == 1 { Cell::Floor } else { Cell::Wall };
                row.push(cell);
            }
            board.push(row);
        }

        return board;
    }
}

impl Iterator for Maker {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        while self.seed < self.combinations {
            let mut board = self.make_board();
            fill::remove_components(&mut board);
            self.seed += 1;

            if valid_stats(&board) && self.filter.accept(&board) {
                return Some(board);
            }
        }

        return None;
    }
}

/// Calculates Floor and Wall stats for the given board and ensures they're within the desired ranges.
fn valid_stats(board: &Board) -> bool {
    let size = board.len();

    if size < 2 { return false; }
    
    let floor_stats = stats::cell_stats(board, Cell::Floor);
    let wall_stats = stats::cell_stats(board, Cell::Wall);

    if floor_stats.count < 3 || wall_stats.count < 1 {
        return false;
    }

    if !check_limits(floor_stats, 1, size - 1) || !check_limits(wall_stats, 1, size - 1) {
        return false;
    }

    return true;
}

/// Returns true if stats has valid bounds and they contain the square delimited by x, y ∈ [min, max].
fn check_limits(stats: stats::CellStats, min: usize, max: usize) -> bool {
    if let (Some(stats_min), Some(stats_max)) = (stats.min, stats.max) {
        if stats_min.0 <= min && stats_min.1 <= min && stats_max.0 >= max && stats_max.1 >= max {
            return true;
        } 
    }

    return false;
}

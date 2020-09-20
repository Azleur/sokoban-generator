//! This module is dedicated to the creation of valid empty boards. See empty::Maker.

use std::u128;
use rand;
use rand::Rng;
// use rand::SeedableRng;

use crate::base::{Board, Cell};
use crate::iters::filter::Symmetries;
use crate::tools::{fill, stats};

/// Provides an exhaustive empty board maker.
/// 
/// Guaranteed to find all valid combinations and finish immediately.
/// Slower to find single examples than random(); deterministic and predictable order.
pub fn serial(size: u8) -> Symmetries<SerialMaker> {
    let serial_maker = SerialMaker::new(size);
    let filter = Symmetries::new(serial_maker);
    return filter;
}

/// Provides a RNG-based empty board maker.
/// 
/// Generally faster at finding single examples than serial().
/// Slower to find the whole set and no confirmation when whole set found.
pub fn random(size: u8) -> Symmetries<RngMaker> {
    let random_maker = RngMaker::new(size);
    let filter = Symmetries::new(random_maker);
    return filter;
}

/// Makes all  unique empty boards of the given size.
pub struct SerialMaker {
    /// Next value used to determine the pattern of floor and wall tiles. Range [0, self.combinations).
    seed: u128,
    /// Number of rows (or columns) in each board.
    size: u8,
    /// Total number of floor & wall combinations possible in this board size (2 ^ (size * size)).
    combinations: u128,
}

impl SerialMaker {
    /// Returns a new Maker for the given board size.
    fn new(size: u8) -> SerialMaker {
        return SerialMaker {
            seed: 0,
            size: size,
            combinations: 1 << (size * size),
        };
    }
}

impl Iterator for SerialMaker {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        while self.seed < self.combinations {
            let board = make_board(self.size, self.seed);
            self.seed += 1;

            if valid_board(&board) {
                return Some(board);
            }
        }

        return None;
    }
}

/// Calculates various stats for the given board and ensures they're within the desired ranges.
fn valid_board(board: &Board) -> bool {
    let size = board.len();

    if size < 2 { return false; }

    if count_connected_components(board) != 1 {
        return false;
    }
    
    let floor_stats = stats::cell_stats(board, Cell::Floor);
    let wall_stats = stats::cell_stats(board, Cell::Wall);

    if floor_stats.count < 2 || wall_stats.count < 1 {
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

/// Returns the number of connected components.
fn count_connected_components(board: &Board) -> usize {
    let mut dummy = board.clone();
    let counts = fill::mark_components(&mut dummy);
    return counts.len();
}

fn make_board(size: u8, seed: u128) -> Board {
    let mut board = Vec::new();
    for y in 0..size {
        let mut row = Vec::new();
        for x in 0..size {
            let index = y * size + x;
            let mask = (seed >> index) & 1;
            let cell = if mask == 1 { Cell::Floor } else { Cell::Wall };
            row.push(cell);
        }
        board.push(row);
    }

    return board;
}

pub struct RngMaker {
    // Number of rows (or columns) in each board.
    size: u8,
    /// This is the magic sauce.
    rng: rand::rngs::ThreadRng,
    /// Safety measure to avoid infinite looping.
    /// Looks like we're skipping one combination?
    count: u128,
    /// Total number of floor & wall combinations possible in this board size (2 ^ (size * size)).
    /// DUPLICATE IN SerialMaker.
    combinations: u128,
}

impl RngMaker {
    fn new(size: u8) -> RngMaker {
        return RngMaker {
            size: size,
            rng: rand::thread_rng(),
            count: 0,
            combinations: 1 << (size * size),
        }
    }
}

impl Iterator for RngMaker {
    type Item = Board;

    // TODO: LOTS OF OVERLAP WITH SerialMaker.
    fn next(&mut self) -> Option<Self::Item> {
        while self.count < u128::MAX { // DANGEROUS
            self.count += 1;
            let seed: u128 = self.rng.gen::<u128>() % self.combinations;
            // println!("seed: {} / {}", seed, self.combinations);
            let board = make_board(self.size, seed);

            if valid_board(&board) {
                return Some(board);
            }
        }

        return None;
    }
}
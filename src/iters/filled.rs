use rand;
use rand::seq::{SliceRandom, IteratorRandom};

use crate::base::{Board, Cell, SecondaryIter};
use crate::iters::filter::Symmetries;

/// Provides an exhaustive board filler.
pub fn serial(board: Board) -> Symmetries<SerialFiller> {
    Symmetries::<SerialFiller>::new_secondary(board)
}

/// Provides a PRNG board filler.
pub fn random(board: Board) -> Symmetries<RngFiller> {
    Symmetries::<RngFiller>::new_secondary(board)
}

pub struct SerialFiller {
    empty_board: Board,
    slots: Vec<(usize, usize)>,
    box_idx: usize,
    goal_idx: usize,
}

impl SecondaryIter for SerialFiller {
    fn new_secondary(empty_board: Board) -> SerialFiller {
        return SerialFiller {
            slots: get_slots(&empty_board),
            empty_board: empty_board,
            box_idx: 0,
            goal_idx: 1,
        };
    }
}

impl Iterator for SerialFiller {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        if self.box_idx >= self.slots.len() {
            return None;
        }

        // Clone empty board and add Cell::Piece, Cell::Goal.
        let mut filled_board = self.empty_board.clone();

        let (box_i, box_j) = self.slots[self.box_idx];
        filled_board[box_j][box_i] = Cell::Piece;

        let (goal_i, goal_j) = self.slots[self.goal_idx];
        filled_board[goal_j][goal_i] = Cell::Goal;

        // Update indices.
        self.goal_idx += 1;
        if self.goal_idx == self.box_idx {
            self.goal_idx += 1;
        }
        if self.goal_idx >= self.slots.len() {
            self.box_idx += 1;
            self.goal_idx = 0;
        }

        // If all is good, return filled board.
        return Some(filled_board);
    }
}

pub struct RngFiller {
    empty_board: Board,
    slots: Vec<(usize, usize)>,
    rng: rand::rngs::ThreadRng,
}

impl SecondaryIter for RngFiller {
    fn new_secondary(empty_board: Board) -> RngFiller {
        return RngFiller {
            slots: get_slots(&empty_board),
            empty_board: empty_board,
            rng: rand::thread_rng(),

        };
    }
}

impl Iterator for RngFiller {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        let mut filled_board = self.empty_board.clone();
        let mut slots: Vec<(usize, usize)> = self.slots.iter().cloned().choose_multiple(&mut self.rng, 2);
        slots.shuffle(&mut self.rng);
        
        let (box_i, box_j) = slots[0];
        filled_board[box_j][box_i] = Cell::Piece;

        let (goal_i, goal_j) = slots[1];
        filled_board[goal_j][goal_i] = Cell::Goal;

        return Some(filled_board);
    }
}

fn get_slots(board: &Board) -> Vec<(usize, usize)> {
    let mut slots: Vec<(usize, usize)> = Vec::new();
    let size = board.len();

    for j in 0..size {
        for i in 0..size {
            if let Cell::Floor = board[j][i] {
                slots.push((i, j));
            }
        }
    }

    return slots;
}

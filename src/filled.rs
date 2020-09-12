use crate::base::{Board, Cell};

pub fn add_box_goal(original: &Board) -> Vec<Board> {
    let mut output = Vec::new();
    let size = original.len();

    let mut floor_indices: Vec<(usize, usize)> = Vec::new();
    for j in 0..size {
        for i in 0..size {
            if let Cell::Floor = original[j][i] {
                floor_indices.push((i, j));
            }
        }
    }

    if floor_indices.len() < 2 {
        return output;
    }

    // for each possible combination add box and goal.
    for box_idx in &floor_indices {
        for goal_idx in &floor_indices {
            if box_idx != goal_idx {
                let mut board = original.clone();

                let (box_i, box_j) = *box_idx;
                board[box_j][box_i] = Cell::Piece;

                let (goal_i, goal_j) = *goal_idx;
                board[goal_j][goal_i] = Cell::Goal;

                output.push(board);
            }
        }
    }

    return output;
}

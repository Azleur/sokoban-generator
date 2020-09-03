//! Utilities for calculating all square symmetries on a board.

use crate::base::Board;

/// Counter-clockwise 90° rotation.
pub fn rotate(board: &Board) -> Board {
    let size = board.len();
    let mut output = Vec::new();
    for y in 0..size {
        let mut row = Vec::new();
        for x in 0..size {
            row.push(board[x][size - 1 - y]);
        }
        output.push(row);
    }

    return output;
}

/// Reflect over the vertical axis x = size / 2.
pub fn reflect_x(board: &Board) -> Board {
    let size = board.len();
    let mut output = Vec::new();
    for y in 0..size {
        let mut row = Vec::new();
        for x in 0..size {
            row.push(board[y][size - 1 - x]);
        }
        output.push(row);
    }

    return output;
}

/// Calculate all D4 (dihedral 4) symmetries of a board.
pub fn all(board: &Board) -> Vec<Board> {
    let e = board.clone();
    let a = rotate(board);
    let a2 = rotate(&a);
    let a3 = rotate(&a2);
    let b = reflect_x(board);
    let ab = rotate(&b);
    let a2b = rotate(&ab);
    let a3b = rotate(&a2b);

    return vec![e, a, a2, a3, b, ab, a2b, a3b];
}

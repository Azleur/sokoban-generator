//! Board statistics utilities.

use std::cmp;

use crate::base::{Board, Cell};

/// Return type of cell_stats(). Gives aggregate information about the presence of one Cell type in a Board.
#[derive(Debug)]
pub struct CellStats {
    /// Number of cells of the given type in the board.
    pub count: usize,
    /// Lower bound of the rectangle containing all cells of the given type in the board, if any.
    pub min: Option<(usize, usize)>,
    /// Upper bound of the rectangle containing all cells of the given type in the board, if any.
    pub max: Option<(usize, usize)>,
    /// Size of the smallest rectangle containing all cells of the given type in the board.
    pub rank: (usize, usize),
}

/// Calculate aggregate statistics for the presence of a Cell type in the Board (see CellStats).
pub fn cell_stats(board: &Board, cell: Cell) -> CellStats {
    let size = board.len();
    let mut stats = CellStats {
        count: 0,
        min: None,
        max: None,
        rank: (0, 0),
    };

    for y in 0..size {
        for x in 0..size {
            if board[y][x] == cell {
                stats.count += 1;

                stats.min = match stats.min {
                    Some((z, w)) => Some((cmp::min(x, z), cmp::min(y, w))),
                    None => Some((x, y)),
                };

                stats.max = match stats.max {
                    Some((z, w)) => Some((cmp::max(x, z), cmp::max(y, w))),
                    None => Some((x, y)),
                };
            }
        }
    }

    if let (Some(min), Some(max)) = (stats.min, stats.max) {
        stats.rank = (1 + max.0 - min.0, 1 + max.1 - min.1);
    }

    return stats;
}

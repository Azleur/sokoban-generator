use crate::base::{Cell, Board, is_wall, is_goal, find_cell};

pub enum Direction { Up, Right, Down, Left }

/// Object returned by move_piece(), summarizing board status information.
pub struct MoveStats {
    /// Number of cells the piece traveled.
    pub cells_moved: u8,
    /// (x,y) indices for the piece position, after moving.
    pub piece_pos: (u8, u8),
    /// True if this move won the game (i.e., moved the piece into the goal).
    pub victory: bool,
}

/// Try moving the piece in the given direction, and return stats for the board status after moving.
pub fn move_piece(board: &mut Board, direction: Direction) -> Option<MoveStats> {
    let size = board.len() as u8;

    if let Some(init_pos) = find_cell(board, Cell::Piece) {
        let delta: (i32, i32) = match direction {
            Direction::Up    => ( 0, -1),
            Direction::Right => ( 1,  0),
            Direction::Down  => ( 0,  1),
            Direction::Left  => (-1,  0),
        };
        let mut pos = init_pos;
        let mut num_cells = 0;
        let mut victory = false;

        loop {
            let new_pos = ((pos.0 as i32 + delta.0) as u8, (pos.1 as i32 + delta.1) as u8);

            // NOTE: This depends on uint overflow to work correctly!
            if new_pos.0 >= size || new_pos.1 >= size {
                break;
            }

            let cell = board[new_pos.1 as usize][new_pos.0 as usize];

            if is_wall(cell) {
                break;
            }

            pos = new_pos;
            num_cells += 1;

            if is_goal(cell) {
                victory = true;
                break;
            }
        }

        board[init_pos.1 as usize][init_pos.0 as usize] = Cell::Floor;
        board[pos.1 as usize][pos.0 as usize] = Cell::Piece;

        return Some(MoveStats { cells_moved: num_cells, piece_pos: pos, victory: victory});
    }
    
    return None;
}
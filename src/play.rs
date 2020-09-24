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
///
/// Returns None if there is no piece to move; Some(stats) otherwise.
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

/// Summary of recursive exploration of the move space of a board.
pub struct ExploreStats {
    /// Can this board be solved?
    solvable: bool,
}

// struct TreeNode {
//     board: Board,
//     parent: &TreeNode,
// }

// /// Recursively explore all possible moves.
// explore_space(board: &Board) -> ExploreStats {
//     let size = board.len();
//     if let None = 
//     if let Some(init_pos) = find_cell(board, Cell::Piece) {
//         let tree = 


//         let mut found = vec![init_pos];
//         let mut idx = 0;

//         while idx < found.len() {

//             for dir in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
//                 let stats = move_piece(board, dir).unwrap();
//                 if stats.victory {
//                     return ExploreStats { solvable: true, };
//                 } else if stats.cells_moved == 0 || found.contains(&stats.piece_pos) {
//                     continue;
//                 } else {
//                     found.push(stats.piece_pos);
//                     pending.push(stats.piece_pos);
//                 }
                
//             }
//         }

//         return ExploreStats { solvable: false, };
//     }

//     return ExploreStats { solvable: false, };
// }
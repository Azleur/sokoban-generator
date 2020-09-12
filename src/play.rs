use crate::base::{Cell, Board, is_wall, is_goal};

pub enum Direction { Up, Right, Down, Left }

pub fn move_piece(board: &mut Board, direction: Direction) -> usize {
    let size = board.len();

    if let Some(init_pos) = find_piece(board) {
        let delta: (i32, i32) = match direction {
            Direction::Up    => ( 0, -1),
            Direction::Right => ( 1,  0),
            Direction::Down  => ( 0,  1),
            Direction::Left  => (-1,  0),
        };
        let mut pos = init_pos;
        let mut num_cells = 0;

        loop {
            let new_pos = ((pos.0 as i32 + delta.0) as usize, (pos.1 as i32 + delta.1) as usize);

            // NOTE: This depends on uint overflow to work correctly!
            if new_pos.0 >= size || new_pos.1 >= size {
                break;
            }

            let cell = board[new_pos.1][new_pos.0];

            if is_wall(cell) {
                break;
            }

            pos = new_pos;
            num_cells += 1;

            if is_goal(cell) {
                break;
            }
        }

        board[init_pos.1][init_pos.0] = Cell::Floor;
        board[pos.1][pos.0] = Cell::Piece;

        return num_cells;
    }
    
    return 0;
}

fn find_piece(board: &Board) -> Option<(usize, usize)> {
    let size = board.len();
    for y in 0..size {
        for x in 0..size {
            if let Cell::Piece = board[y][x] {
                return Some((x, y));
            }
        }
    }

    return None;
}
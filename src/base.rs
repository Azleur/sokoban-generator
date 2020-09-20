use std::char;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Floor,
    Wall,
    Piece,
    Goal,
    Zone(u32),
}

fn cell_symbol(cell: &Cell) -> char {
    match cell {
        Cell::Floor => '.',
        Cell::Wall => 'X',
        Cell::Piece => '#',
        Cell::Goal => '@',
        Cell::Zone(num) => char::from_digit(*num, 10).unwrap(),
    }
}

pub fn is_floor(cell: Cell) -> bool {
    return match cell {
        Cell::Floor => true,
        _ => false,
    };
}

pub fn is_wall(cell: Cell) -> bool {
    return match cell {
        Cell::Wall => true,
        _ => false,
    };
}

pub fn is_goal(cell: Cell) -> bool {
    return match cell {
        Cell::Goal => true,
        _ => false,
    };
}

pub type Board = Vec<Vec<Cell>>;

pub fn print_board(board: &Board) {
    for row in board {
        for cell in row {
            print!("{} ", cell_symbol(cell));
        }
        print!("\n");
    }
}

// pub trait Iter = Iterator<Item = Board>;
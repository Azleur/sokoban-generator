use std::char;
use crossterm::style::{Colorize, StyledContent};

use crate::base::{Cell, Board};

pub fn color_cell_symbol(cell: &Cell) -> StyledContent<char> {
    match cell {
        Cell::Floor => '.'.dark_grey(),
        Cell::Wall => 'X'.grey(),
        Cell::Piece => '#'.cyan(),
        Cell::Goal => '@'.magenta(),
        Cell::Zone(num) => {
            let symbol = char::from_digit(*num, 10).unwrap();
            symbol.yellow()
        },
    }
}

pub fn color_print_board(board: &Board) {
    for row in board {
        for cell in row {
            print!("{} ", color_cell_symbol(cell));
        }
        print!("\r\n");
    }
}
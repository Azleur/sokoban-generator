use std::{process, env};

use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

use sokoban_generator::base::{Cell, print_board, find_cell};
use sokoban_generator::iters::{empty, filled};

use sokoban_generator::play::{Direction, move_piece};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
    }
    let size = args[1].parse::<u8>().unwrap();

    let empty_board = empty::random(size).next().unwrap();
    let mut board = filled::random(empty_board).next().unwrap();
    let mut round = 0;
    let mut old_pos = find_cell(&board, Cell::Piece).unwrap();
    let mut new_pos = old_pos;

    enable_raw_mode().unwrap();

    loop {
        print!("---------- [{}] ----------\r\n", round);
        print!("Arrows to move; q to quit.\r\n");
        print!("#: box; @: goal.\r\n");
        print!("--------------------------\r\n");
        if old_pos != new_pos {
            print!("({}, {}) -> ({}, {})\r\n", old_pos.0, old_pos.1, new_pos.0, new_pos.1);
        } else {
            print!("NO MOVE\r\n");
        }
        print!("--------------------------\r\n");
        print_board(&board);

        round += 1;
        old_pos = new_pos;
        
        let mut direction = Direction::Up;

        match read().unwrap() {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Up => direction = Direction::Up,
                    KeyCode::Down => direction = Direction::Down,
                    KeyCode::Left => direction = Direction::Left,
                    KeyCode::Right => direction = Direction::Right,
                    KeyCode::Char('q') => break,
                    _ => (),
                }
            },
            _ => (),
        }

        let stats = move_piece(&mut board, direction).unwrap();
        new_pos = stats.piece_pos;
        if stats.victory {
            print!("You won!\r\n");
            break;
        }
    }

    disable_raw_mode().unwrap();
    println!("Bye!");
}

fn usage() -> ! {
    println!("Usage: game <size>");
    process::exit(1);
}
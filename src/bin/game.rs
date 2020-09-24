use std::{process, env};
use std::io::stdout;

use crossterm::ExecutableCommand;
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode};
use crossterm::cursor::MoveTo;
use crossterm::style::Colorize;

use sokoban_generator::base::{Cell, Board, find_cell};
use sokoban_generator::iters::holistic;
use sokoban_generator::colorprint::{color_cell_symbol, color_print_board};

use sokoban_generator::play::{Direction, move_piece, explore_space};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
    }
    let size = args[1].parse::<u8>().unwrap();

    let mut stdout = stdout();

    let mut iter = holistic::solvable_random(size);

    let mut board = iter.next().unwrap();
    let mut state = GameState::fresh(board);

    enable_raw_mode().unwrap();
    stdout.execute(Clear(ClearType::All)).unwrap();

    loop {
        stdout.execute(MoveTo(0, 0)).unwrap();
        let round_display = format!("{:02}", state.round).yellow();
        print!("------------- [{}] -------------\r\n", round_display);
        print!("{} to move.\r\n", "Arrows".yellow());
        print!("{} to quit.\r\n", "q".yellow());
        print!("{} to restart (randomizes board).\r\n", "r".yellow());
        print!("{}: box; {}: goal.\r\n", color_cell_symbol(&Cell::Piece), color_cell_symbol(&Cell::Goal));
        print!("--------------------------------\r\n");
        if state.victory {
            print!("VICTORY!                    \r\n");
        } else if state.old_pos != state.new_pos {
            print!("({}, {}) -> ({}, {})\r\n", state.old_pos.0, state.old_pos.1, state.new_pos.0, state.new_pos.1);
        } else {
            print!("Didn't move.                \r\n");
        }
        print!("--------------------------------\r\n");
        print!("Best:  {}   \r\n", state.best_moves);
        print!("Moves: {}. ", state.current_moves);
        if state.victory || state.can_win {
            print!("Remaining: {}.", state.remaining_moves);
        } else {
            print!("[CAN'T WIN]");
        }
        print!("      \r\n");
        println!("--------------------------------\r\n");
        color_print_board(&state.board);

        let mut direction = Direction::Up;
        match read().unwrap() {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Up    | KeyCode::Char('w') => direction = Direction::Up,
                    KeyCode::Left  | KeyCode::Char('a') => direction = Direction::Left,
                    KeyCode::Down  | KeyCode::Char('s') => direction = Direction::Down,
                    KeyCode::Right | KeyCode::Char('d') => direction = Direction::Right,
                    KeyCode::Char('r') => {
                        board = iter.next().unwrap();
                        state = GameState::fresh(board);
                        continue;
                    },
                    KeyCode::Char('q') => break,
                    _ => (),
                }
            },
            _ => (),
        }
        state.play(direction);
    }

    disable_raw_mode().unwrap();
}

fn usage() -> ! {
    println!("Usage: game <size>");
    process::exit(1);
}

struct GameState {
    board: Board,
    round: usize,
    old_pos: (u8, u8),
    new_pos: (u8, u8),
    current_moves: usize,
    best_moves: usize,
    remaining_moves: usize,
    can_win: bool,
    victory: bool,
}

impl GameState {
    fn fresh(board: Board) -> Self {
        let pos = find_cell(&board, Cell::Piece).unwrap();
        let stats = explore_space(&board);
        return GameState {
            board: board,
            round: 0,
            old_pos: pos,
            new_pos: pos,
            current_moves: 0,
            best_moves: stats.num_moves,
            remaining_moves: stats.num_moves,
            can_win: true,
            victory: false,
        };
    }

    fn play(&mut self, direction: Direction) {
        if self.victory { 
            return;
        }

        self.round += 1;

        let move_stats = move_piece(&mut self.board, direction).unwrap();
        self.new_pos = move_stats.piece_pos;
        self.victory = move_stats.victory;
        if self.old_pos != self.new_pos {
            self.current_moves += 1;
        }

        let explore_stats = explore_space(&self.board);
        self.can_win = explore_stats.solvable;
        self.remaining_moves = explore_stats.num_moves;
    }
}
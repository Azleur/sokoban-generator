use std::{process, env};

use sokoban_generator::iters::{empty, filled};
use sokoban_generator::colorprint::color_print_board;
use sokoban_generator::play::explore_space;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
    }
    let size = args[1].parse::<u8>().unwrap();

    let empty_board = empty::random(size).next().unwrap();
    let board = filled::random(empty_board).next().unwrap();
    
    println!("---------- BOARD ----------");
    color_print_board(&board);
    println!("---------------------------");

    let stats = explore_space(&board);
    
    println!("---------- STATS ----------");
    println!("{:?}", stats);
    println!("---------------------------");
}

fn usage() -> ! {
    println!("Usage: solve_single <size>");
    process::exit(1);
}
use std::{env, process};

use base::{print_board};

mod base;
mod filter;
mod empty;
mod tools;
mod filled;
mod play;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        usage();
    }

    let min = args[1].parse::<usize>().unwrap();
    let max = args[2].parse::<usize>().unwrap();

    for size in min..=max {
        println!("================================================================");
        println!("=========================== SIZE {} ============================", size);
        println!("================================================================");

        for empty_board in empty::Maker::new(size) {
            println!("================================");

            println!("-------- EMPTY BOARD --------");
            print_board(&empty_board);

            println!("-------- FILLED BOARDS --------");
            let mut filter = filter::Symmetries::new();
            let mut filled_boards = filled::add_box_goal(&empty_board);
            filled_boards.retain(|board| filter.accept(board));
            for filled_board in filled_boards {
                println!("--------");
                print_board(&filled_board);
            }
        }
    }
}

fn usage() -> ! {
    println!("Usage: sokoban-generator <min-size> <max-size>");
    process::exit(1);
}
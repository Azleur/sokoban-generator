use std::{process, env, time};

use sokoban_generator::base::print_board;
use sokoban_generator::empty::RngMaker;
use sokoban_generator::filter;
use sokoban_generator::filled;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
    }

    let size = args[1].parse::<u8>().unwrap();

    let timer = time::Instant::now();

    let mut iterator = RngMaker::new(size);
    let base_case = iterator.next().unwrap();

    println!("-------- EMPTY BOARD --------");
    print_board(&base_case);

    println!("-------- FILLED BOARDS --------");
    let mut filter = filter::Symmetries::new();
    let mut filled_boards = filled::add_box_goal(&base_case);
    filled_boards.retain(|board| filter.accept(board));
    for (idx, filled_board) in filled_boards.iter().enumerate() {
        println!("---- [{} ({:?})] ----", idx, timer.elapsed());
        print_board(&filled_board);
    }
}

fn usage() -> ! {
    println!("Usage: single <size>");
    process::exit(1);
}
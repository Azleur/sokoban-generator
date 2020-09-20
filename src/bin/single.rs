use std::{process, env, time};

use sokoban_generator::base::print_board;
use sokoban_generator::iters::{empty, filled};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
    }

    let size = args[1].parse::<u8>().unwrap();

    let timer = time::Instant::now();
    let base_case = empty::random(size).next().unwrap();

    println!("-------- EMPTY BOARD --------");
    print_board(&base_case);

    println!("-------- FILLED BOARDS --------");
    for (idx, filled_board) in filled::random(base_case).enumerate() {
        println!("---- [{} ({:?})] ----", idx, timer.elapsed());
        print_board(&filled_board);
    }
}

fn usage() -> ! {
    println!("Usage: single <size>");
    process::exit(1);
}
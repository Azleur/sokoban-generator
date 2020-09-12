use std::{env, process, time};

use sokoban_generator::base::{Board, print_board};
use sokoban_generator::empty::{SerialMaker, RngMaker};

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        usage();
    }

    let method = args[1].trim().to_lowercase();
    let min = args[2].parse::<u8>().unwrap();
    let max = args[3].parse::<u8>().unwrap();

    if method != "serial" && method != "random" {
        usage();
    }

    let timer = time::Instant::now();
    for size in min..=max {
        println!("================================================================");
        println!("=========================== SIZE {} ============================", size);
        println!("================================================================");

        let iterator: Box<dyn Iterator<Item = Board>> = if method == "serial" { Box::new(SerialMaker::new(size)) } else { Box::new(RngMaker::new(size)) };
        for (idx, empty_board) in iterator.enumerate() {
            println!("================ [{} ({:?})] ================", idx, timer.elapsed());

            // println!("-------- EMPTY BOARD --------");
            print_board(&empty_board);

            // println!("-------- FILLED BOARDS --------");
            // let mut filter = filter::Symmetries::new();
            // let mut filled_boards = filled::add_box_goal(&empty_board);
            // filled_boards.retain(|board| filter.accept(board));
            // for filled_board in filled_boards {
            //     println!("--------");
            //     print_board(&filled_board);
            // }
        }
    }
}

fn usage() -> ! {
    println!("Usage: range (serial|random) <min-size> <max-size>");
    process::exit(1);
}
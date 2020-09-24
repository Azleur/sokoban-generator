use std::{f64, fmt, time};

use sokoban_generator::base::Board;
use sokoban_generator::iters::holistic;
use sokoban_generator::play::explore_space;

const MIN_SIZE : u8 = 4;
const MAX_SIZE : u8 = 7;
const NUM_ITERS: usize = 1000;

fn main() {
    println!("sizes: {} to {}. Iterations: {}.", MIN_SIZE, MAX_SIZE, NUM_ITERS);

    let general_timer = time::Instant::now();
    for size in MIN_SIZE..=MAX_SIZE {
        println!("-------------------- SIZE: {:02} [{:?}] --------------------", size, general_timer.elapsed());

        let phase_timer = time::Instant::now();

        let mut iters: Vec<(&'static str, Box<dyn Iterator<Item = Board>>, StatsCollector, StatsCollector)> = vec![
            ("    SERIAL     ", Box::new(holistic::serial(size))         , StatsCollector::new(), StatsCollector::new()),
            ("    RANDOM     ", Box::new(holistic::random(size))         , StatsCollector::new(), StatsCollector::new()),
            ("SOLVABLE SERIAL", Box::new(holistic::solvable_serial(size)), StatsCollector::new(), StatsCollector::new()),
            ("SOLVABLE RANDOM", Box::new(holistic::solvable_random(size)), StatsCollector::new(), StatsCollector::new()),
        ];
    
        'iters: for _iteration in 0..NUM_ITERS {
            for (_name, iter, stats_solved, stats_moves) in &mut iters {
                if let Some(sample) = iter.next() {
                    let explore_stats = explore_space(&sample);
                    stats_solved.observe(explore_stats.solvable as usize as f64);
                    stats_moves.observe(explore_stats.num_moves as f64);
                } else {
                    break 'iters;
                }
            }
        }

        println!("Duration: {:?}", phase_timer.elapsed());

        for (name, _iter, stats_solved, stats_moves) in &iters {
            println!("[{}] solved: {}; moves: {}", name, stats_solved, stats_moves);
        }
    }
}

#[derive(Debug)]
struct StatsCollector {
    min: f64,
    max: f64,
    avg: f64,
    n:   u32,
}

impl StatsCollector {
    fn new() -> Self {
        StatsCollector {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            avg: 0.0,
            n: 0,
        }
    }

    fn observe(&mut self, sample: f64) {
        self.n += 1;
        self.min = f64::min(self.min, sample);
        self.max = f64::max(self.max, sample);
        self.avg += (sample - self.avg) / (self.n as f64);
    }
}

impl fmt::Display for StatsCollector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:.3}; {:.3}; {:.3}]", self.min, self.avg, self.max)
    }
}
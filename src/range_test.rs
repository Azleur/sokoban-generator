use std::{u64, u128};

fn test() {
  println!("==== TESTING RANGES ====");
  for i in 1..=11 {
    let big: u128 = 1 << (i * i);
    println!("1 << ({0:02} * {0:02}): {1}", i, big);
  }
  println!("      u64  MAX: {}", u64::MAX);
  println!("      u128 MAX: {}", u128::MAX);
}
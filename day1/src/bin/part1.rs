use std::io;
use std::io::prelude::*;

fn compute_one(x: u64) -> u64 {
    x / 3 - 2
}

fn main() {
    let stdin = io::stdin();
    let stdlocked = stdin.lock();

    let result: u64 = stdlocked.lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| x.parse::<u64>().ok())
        .map(compute_one)
        .sum();
    println!("{}", result);
}

use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let stdlocked = stdin.lock();

    let result: u64 = stdlocked
        .lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| x.parse::<u64>().ok())
        .map(|x| x / 3 - 2)
        .sum();
    println!("{}", result);
}

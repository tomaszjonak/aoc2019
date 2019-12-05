use day2;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let stdlocked = stdin.lock();

    let mem: Vec<u64> = stdlocked
        .split(',' as u8)
        .filter_map(|x| x.ok())
        .map(|x| String::from_utf8(x).unwrap().parse::<u64>().unwrap())
        .collect();

    println!("{:?}", mem);

    let result = day2::vm(&mem);
    println!("{:?}", result)
}

use day2;
use itertools::Itertools;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let stdlocked = stdin.lock();

    let mut mem: Vec<u64> = stdlocked
        .split(',' as u8)
        .filter_map(|x| x.ok())
        .map(|x| String::from_utf8(x).unwrap().parse::<u64>().unwrap())
        .collect();

    let expected_mem0 = 19690720u64;
    for (noun, verb) in (0..99).into_iter().cartesian_product((0..99).into_iter()) {
        mem[1] = noun;
        mem[2] = verb;
        let res = day2::vm(&mem);
        if res[0] == expected_mem0 {
            let ans = 100 * noun + verb;
            println!("Answer found: {}", ans);
            break;
        }
    }
    println!("Finished")
}

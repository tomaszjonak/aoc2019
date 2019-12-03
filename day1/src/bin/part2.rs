use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

struct Cached(HashMap<u64,u64>);

impl Cached {
    pub fn new() -> Self { Self(HashMap::new()) }
    pub fn compute(&mut self, x: u64) -> u64 {
        let mut x = x;
        if let Some(v) = self.0.get(&x) {
            return *v;
        }

        let mut v = vec![x];
        x = compute_one(x);
        while x > 0 {
            v.push(x);
            if let Some(&val) = self.0.get(&x) {
                v.push(val);
                break;
            }
            x = compute_one(x);
        }

        let mut sum: u64 = 0;
        for (i, &val) in v.iter().enumerate().rev() {
            if val == 0 {
                break;
            }

            if i == 0 {
                break;
            }

            sum += val;
            self.0.insert(v[i - 1], sum);
        }
        sum
    }
}

fn compute_one(x: u64) -> u64 {
    if x  < 6 {
        return 0;
    }
    (x / 3) - 2
}

/*
c.compute(1969) = 654 + 216 + 70 + 21 + 5 = 966
c.compute(100756) = 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346
c.compute(2) = 0
c.compute(1) = 0
c.compute(0) = 0
*/

#[cfg(test)]
mod tests {
    #[test]
    fn test_cache() {
        let mut c = super::Cached::new();
        assert_eq!(c.compute(0), 0);
        assert_eq!(c.compute(1), 0);
        assert_eq!(c.compute(2), 0);
        assert_eq!(c.compute(3), 0);
        assert_eq!(c.compute(9), 1);
        assert_eq!(c.compute(1969), 966);
        assert_eq!(c.compute(100756), 50346);
    }
}

fn main() {
    let stdin = io::stdin();
    let stdlocked = stdin.lock();

    let mut c = Cached::new();

    let result: u64 = stdlocked.lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| x.parse::<u64>().ok())
        .map(|x| c.compute(x))
        .sum();
    println!("{}", result);
}

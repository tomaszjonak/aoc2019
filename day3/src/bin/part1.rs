use day3;
use itertools::Itertools;
use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();

    let res = calculate(stdin.lock()).unwrap();
    println!("{}", res)
}

fn calculate(r: impl BufRead) -> Option<i32> {
    let start = day3::Point::new(0, 0);

    let mut data: Vec<_> = r
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let res: Vec<_> = line
                .split(',')
                .filter(|x| x.len() > 1)
                .map(|x| {
                    let (dir, value) = x.split_at(1);
                    day3::Direction::from((dir.parse::<char>().unwrap(), value.parse::<i32>().unwrap()))
                })
                .collect();
            res
        })
        .map(|line| {
            let mut vert: Vec<day3::Line> = Vec::new();
            let mut hor: Vec<day3::Line> = Vec::new();

            let mut lit = line.into_iter();
            let mut l: day3::Line = (start, lit.next().unwrap()).into();
            for dir in lit {
                l = (&l, dir).into();
                match l {
                    day3::Line::Vertical(_, _) => vert.push(l),
                    day3::Line::Horizontal(_, _) => hor.push(l),
                }
            }
            (vert, hor)
        })
        .collect();

    let w1 = data.pop().unwrap();
    let w2 = data.pop().unwrap();

    let i1 = w1.0.into_iter().cartesian_product(w2.1.into_iter());
    let i2 = w2.0.into_iter().cartesian_product(w1.1.into_iter());
    i1.chain(i2)
        .filter_map(|(l1, l2)| l1.intersects(l2))
        .map(|intersection_point| start.manhattan_dist(&intersection_point))
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn tc0() {
        let r = io::Cursor::new("R8,U5,L5,D3\nU7,R6,D4,L4");
        let res = calculate(r).unwrap();
        assert_eq!(res, 6);
    }

    #[test]
    fn tc1() {
        let r = io::Cursor::new("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83");
        let res = calculate(r).unwrap();
        assert_eq!(res, 159);
    }

    #[test]
    fn tc2() {
        let r = io::Cursor::new("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let res = calculate(r).unwrap();
        assert_eq!(res, 135);
    }

    #[test]
    fn tc3() {
        let f = File::open("input.txt").unwrap();
        let reader = BufReader::new(f);
        let res = calculate(reader);
        println!("{:?}", res);
    }
}

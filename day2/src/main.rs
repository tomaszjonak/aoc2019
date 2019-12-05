use std::io;
use std::io::prelude::*;

#[derive(PartialEq)]
enum Opcode {
    Add,
    Mul,
    Halt,
    Invalid(u64)
}

impl std::convert::From<u64> for Opcode {
    fn from(i: u64) -> Self {
        match i {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            99 => Opcode::Halt,
            x => Opcode::Invalid(x),
        }
    }
}

fn vm(mem: &[u64]) -> Vec<u64> {
    let mut res = mem.to_vec();

    let mut op_ptr = 0;
    let mut op = Opcode::Invalid(100);
    while op != Opcode::Halt {
        if op_ptr > mem.len() {
            panic!("Out of bounds (len: {}, op_ptr: {})", mem.len(), op_ptr);
        }
        op = res[op_ptr].into();
        match op {
            Opcode::Add => {
                let (dest, lhs, rhs) = (res[op_ptr+3] as usize, res[op_ptr+1] as usize, res[op_ptr+2] as usize);
                res[dest] = res[lhs] + res[rhs];
            },
            Opcode::Mul => {
                let (dest, lhs, rhs) = (res[op_ptr+3] as usize, res[op_ptr+1] as usize, res[op_ptr+2] as usize);
                res[dest] = res[lhs] * res[rhs];
            },
            Opcode::Halt => break,
            Opcode::Invalid(x) => panic!("Invalid opcode encountered: {}", x),
        }
        op_ptr += 4;
    }

    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_vm_addition() {
        let mem = vec![1,0,0,0,99];
        let expected = vec![2,0,0,0,99];
        let result = super::vm(&mem);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vm_multiplication() {
        let mem = vec![2,3,0,3,99];
        let expected = vec![2,3,0,6,99];
        let result = super::vm(&mem);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vm_compound() {
        let mem = vec![1,1,1,4,99,5,6,0,99];
        let expected = vec![30,1,1,4,2,5,6,0,99];
        let result = super::vm(&mem);
        assert_eq!(result, expected);
    }
}

fn main() {
    let stdin = io::stdin();
    let stdlocked = stdin.lock();

    let mem: Vec<u64> = stdlocked.split(',' as u8)
        .filter_map(|x| x.ok())
        .map(|x| String::from_utf8(x).unwrap().parse::<u64>().unwrap())
        .collect();

    let result = vm(&mem);
    println!("{:?}", result)
}

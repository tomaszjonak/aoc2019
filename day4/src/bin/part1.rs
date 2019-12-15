use std::ops::Rem;

fn main() {
    let mut count = 0;
    for v in 130254..=678275 {
        if is_nondecreasing(v) && has_doubledigit(v) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn has_doubledigit(x: u32) -> bool {
    let mut last = x; 
    for _ in (0..6) {
        let cur = last / 10;
        if (last.rem(10) as i32) - (cur.rem(10) as i32) == 0 {
            return true
        }
        last = cur;
    }
    false
}

fn is_nondecreasing(x: u32) -> bool {
    let mut last = x;
    for _ in (0..6) {
        let cur = last / 10;
        if last.rem(10) < cur.rem(10) {
            return false
        }
        last = cur;
    }
    true
}

fn digit_at_index(x:u32, i:u32) -> u32 {
    (x / 10u32.pow(i)).rem(10)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn digit_at_ind() {
        assert_eq!(digit_at_index(123456, 4), 2);
    }

    #[test]
    fn test_nondecreasing() {
        assert_eq!(is_nondecreasing(123456), true);
        assert_eq!(is_nondecreasing(123465), false);
        assert_eq!(is_nondecreasing(133355), true);
        assert_eq!(is_nondecreasing(433355), false);
    }

    #[test]
    fn test_doubledigit() {
        assert_eq!(has_doubledigit(123456), false);
        assert_eq!(has_doubledigit(123466), true);
        assert_eq!(has_doubledigit(123356), true);
        assert_eq!(has_doubledigit(113456), true);
    }
}
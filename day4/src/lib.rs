use std::ops::Rem;

pub fn has_exclusive_doubledigit(x: u32) -> bool {
    let mut count = 0;
    let mut in_sequence = false;
    let mut last = x;
    for _ in (0..6) {
        let cur = last / 10;
        if (last.rem(10) as i32) - (cur.rem(10) as i32) == 0 {
            if in_sequence {
                count += 1;
            } else {
                in_sequence = true;
                count = 1;
            }
        } else {
            if count == 1 {
                return true;
            }
            in_sequence = false;
        }
        last = cur;
    }

    false
}

pub fn has_doubledigit(x: u32) -> bool {
    let mut last = x;
    for _ in (0..6) {
        let cur = last / 10;
        if (last.rem(10) as i32) - (cur.rem(10) as i32) == 0 {
            return true;
        }
        last = cur;
    }
    false
}

pub fn is_nondecreasing(x: u32) -> bool {
    let mut last = x;
    for _ in (0..6) {
        let cur = last / 10;
        if last.rem(10) < cur.rem(10) {
            return false;
        }
        last = cur;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_has_exclusive_doubledigit() {
        assert_eq!(has_exclusive_doubledigit(123456), false);
        assert_eq!(has_exclusive_doubledigit(122256), false);
        assert_eq!(has_exclusive_doubledigit(122456), true);
        assert_eq!(has_exclusive_doubledigit(112345), true);
        assert_eq!(has_exclusive_doubledigit(123345), true);
        assert_eq!(has_exclusive_doubledigit(123455), true);
        assert_eq!(has_exclusive_doubledigit(111122), true);
        assert_eq!(has_exclusive_doubledigit(112233), true);
    }
}

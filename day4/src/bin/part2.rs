use day4;

fn main() {
    let mut count = 0;
    for v in 130254..=678275 {
        if day4::is_nondecreasing(v) && day4::has_exclusive_doubledigit(v) {
            count += 1;
        }
    }
    println!("{}", count);
}

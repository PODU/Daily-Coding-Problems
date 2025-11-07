// Max of two numbers without if-else/branch/comparison via bit manipulation.
// d=a-b; mask = d>>63 (arithmetic sign extend); max = a - (d & mask). Time O(1), Space O(1).
fn max_no_branch(a: i64, b: i64) -> i64 {
    let d = a.wrapping_sub(b);
    let mask = d >> 63; // all 1s if d<0, else 0
    a.wrapping_sub(d & mask)
}

fn main() {
    println!("{}", max_no_branch(3, 7));
    println!("{}", max_no_branch(10, -4));
}

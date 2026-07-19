// Day 1843: Max of two numbers with no branching/comparison; uses sign bit of the difference.
// max(a,b) = a - ((a-b) & ((a-b) >> 63)). Time O(1), Space O(1).

fn max_no_branch(a: i64, b: i64) -> i64 {
    let d = a.wrapping_sub(b);
    a - (d & (d >> 63)) // arithmetic shift: 0 if a>=b, -1 if a<b
}

fn main() {
    println!("{}", max_no_branch(5, 9)); // 9
    println!("{}", max_no_branch(12, 4)); // 12
    println!("{}", max_no_branch(-3, -7)); // -3
}

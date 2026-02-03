// Branchless max: use 64-bit diff to avoid overflow; sign bit via arithmetic
// right shift selects the larger. No if/branch/compare. Time O(1), Space O(1).
fn max_no_branch(a: i32, b: i32) -> i32 {
    let diff: i64 = a as i64 - b as i64; // a - b without i32 overflow
    let sign: i64 = diff >> 63;          // -1 if diff<0 else 0
    (a as i64 - (diff & sign)) as i32    // a>=b -> a ; a<b -> b
}

fn main() {
    println!("max(3, 7) = {}", max_no_branch(3, 7));
    println!("max(10, -5) = {}", max_no_branch(10, -5));
}

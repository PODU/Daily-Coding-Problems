// Max of two without if/comparison: subtract the masked difference using the sign bit.
// max(a,b) = a - ((a-b) & ((a-b)>>63)). Time O(1), Space O(1).
fn max_no_branch(a: i64, b: i64) -> i64 {
    let diff = a - b;
    a - (diff & (diff >> 63))
}

fn main() {
    println!("{}", max_no_branch(3, 7));   // 7
    println!("{}", max_no_branch(10, -5)); // 10
}

// Day 1268: Select x if b==1 else y using only arithmetic/bit ops (no branches).
// y ^ ((x ^ y) & -b): -b is all-ones when b==1, all-zeros when b==0. O(1).
fn select(x: i32, y: i32, b: i32) -> i32 {
    y ^ ((x ^ y) & b.wrapping_neg())
}

fn main() {
    let (x, y) = (5, 10);
    println!("b=1 -> {}", select(x, y, 1)); // 5
    println!("b=0 -> {}", select(x, y, 0)); // 10
}

// Branchless select: mask = -b (all 1s if b=1, all 0s if b=0); pick x or y. O(1).
fn select(x: i32, y: i32, b: i32) -> i32 {
    let mask = b.wrapping_neg(); // 0xFFFFFFFF if b==1, 0 if b==0
    (x & mask) | (y & !mask)
}

fn main() {
    println!("{}", select(42, 17, 1)); // 42
    println!("{}", select(42, 17, 0)); // 17
}

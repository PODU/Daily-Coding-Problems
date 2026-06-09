// Branchless select: mask = -b (all-ones if b=1 else 0); result = (x & mask) | (y & !mask). O(1) time/space.
fn select(b: i32, x: i32, y: i32) -> i32 {
    let mask = b.wrapping_neg(); // 0xFFFFFFFF if b=1, 0 if b=0
    (x & mask) | (y & !mask)
}

fn main() {
    println!("{}", select(1, 42, 99));
    println!("{}", select(0, 42, 99));
}

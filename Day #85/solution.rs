// Day 85: Select x if b==1 else y using only bit ops. mask = -b (all 1s or all 0s).
// Time O(1), Space O(1).
fn select(x: i32, y: i32, b: i32) -> i32 {
    let mask = b.wrapping_neg(); // b=1 -> 0xFFFFFFFF, b=0 -> 0x00000000
    (x & mask) | (y & !mask)
}

fn main() {
    println!("{}", select(5, 10, 1)); // 5
    println!("{}", select(5, 10, 0)); // 10
}

// Branchless select via bitwise mask: y ^ ((-b) & (x ^ y)). O(1) time, O(1) space.

fn select(x: i32, y: i32, b: i32) -> i32 {
    y ^ ((-b) & (x ^ y))
}

fn main() {
    println!("b=1 -> {}", select(5, 9, 1));
    println!("b=0 -> {}", select(5, 9, 0));
}

// Day 128: Tower of Hanoi - print all moves.
// Classic recursion. O(2^n) moves (optimal), O(n) recursion depth.
fn hanoi(n: u32, from: u32, via: u32, to: u32) {
    if n == 0 {
        return;
    }
    hanoi(n - 1, from, to, via);
    println!("Move {} to {}", from, to);
    hanoi(n - 1, via, from, to);
}

fn main() {
    hanoi(3, 1, 2, 3);
}

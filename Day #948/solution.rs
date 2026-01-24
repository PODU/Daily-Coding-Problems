// Day 948: Tower of Hanoi - print all moves to move n disks from rod 1 to rod 3.
// Classic recursion. Time O(2^n) moves, Space O(n) recursion depth.

fn hanoi(n: u32, from: u32, to: u32, aux: u32) {
    if n == 0 {
        return;
    }
    hanoi(n - 1, from, aux, to);
    println!("Move {} to {}", from, to);
    hanoi(n - 1, aux, to, from);
}

fn main() {
    hanoi(3, 1, 3, 2);
}

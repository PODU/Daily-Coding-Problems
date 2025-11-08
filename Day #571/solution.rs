// Tower of Hanoi: classic recursion. Move n disks from rod `from` to `to` using `via`.
// Prints 2^n - 1 moves. O(2^n) time, O(n) recursion depth.
fn hanoi(n: u32, from: u32, to: u32, via: u32) {
    if n == 0 {
        return;
    }
    hanoi(n - 1, from, via, to);
    println!("Move {} to {}", from, to);
    hanoi(n - 1, via, to, from);
}

fn main() {
    hanoi(3, 1, 3, 2);
}

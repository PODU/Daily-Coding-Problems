// Reconstruct a permutation of [0..N] from +/- signs. Two-pointer, O(N).
// '+' takes the current low, '-' takes the current high; produces a valid order.
fn reconstruct(signs: &[char]) -> Vec<i32> {
    let l = signs.len() + 1;
    let n = (l - 1) as i32;
    let mut res = vec![0i32; l];
    let (mut low, mut high) = (0i32, n);
    for (j, &s) in signs.iter().enumerate() {
        if s == '+' {
            res[j] = low;
            low += 1;
        } else {
            res[j] = high;
            high -= 1;
        }
    }
    res[l - 1] = low;
    res
}

fn main() {
    // input [None, +, +, -, +] -> constraints (+, +, -, +)
    let signs = ['+', '+', '-', '+'];
    println!("{:?}", reconstruct(&signs)); // a valid reconstruction, e.g. [0, 1, 4, 2, 3]
}

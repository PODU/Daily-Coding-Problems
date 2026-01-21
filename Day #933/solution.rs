// Day 933: Reconstruct a permutation of [0..N] consistent with +/- signs.
// Two-pointer: '+' takes the current low, '-' takes the current high. O(n) time, O(n) space.
// Many arrangements are valid; README shows [1,2,3,0,4], this prints another valid one.

// signs[0] is the sentinel for None; signs[i] in {'+','-'} for i>=1.
fn reconstruct(signs: &[char]) -> Vec<i32> {
    let n = signs.len();
    let mut lo: i32 = 0;
    let mut hi: i32 = (n as i32) - 1;
    let mut res = Vec::with_capacity(n);
    for &c in &signs[1..] {
        if c == '+' {
            res.push(lo);
            lo += 1;
        } else {
            res.push(hi);
            hi -= 1;
        }
    }
    res.push(lo); // lo == hi
    res
}

fn main() {
    let signs = ['?', '+', '+', '-', '+']; // [None, +, +, -, +]
    println!("{:?}", reconstruct(&signs)); // e.g. [0, 1, 4, 2, 3] (a valid reconstruction)
}

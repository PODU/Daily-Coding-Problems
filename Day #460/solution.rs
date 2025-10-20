// Day 460: Min flips so every 'x' precedes every 'y'.
// One-pass DP: dp = min(flip this x->y, flip all prior y->x). Time O(n), Space O(1).
fn min_flips(s: &str) -> i32 {
    let (mut dp, mut y) = (0i32, 0i32);
    for c in s.chars() {
        if c == 'y' {
            y += 1;
        } else {
            dp = (dp + 1).min(y);
        }
    }
    dp
}

fn main() {
    println!("{}", min_flips("xyxxxyxyy")); // 2
}

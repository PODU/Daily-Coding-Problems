// Min flips so all x precede all y. DP: at each char, flips = min(yCount, flips+1):
// flipping current 'y'->'x' costs all prior y's; flipping current 'x'->'y' costs flips+1. Time O(n), space O(1).

fn min_flips(s: &str) -> i32 {
    let mut flips = 0;
    let mut y = 0;
    for c in s.chars() {
        if c == 'y' {
            y += 1;
        } else {
            flips = y.min(flips + 1);
        }
    }
    flips
}

fn main() {
    println!("{}", min_flips("xyxxxyxyy")); // 2
}

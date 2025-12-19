// Day 766: Minimum flips so all 'x' come before all 'y'.
// One-pass DP: flips = min(flips+1, count_y). O(n) time, O(1) space.
fn min_flips(s: &str) -> i32 {
    let mut flips = 0;
    let mut count_y = 0;
    for c in s.chars() {
        if c == 'y' {
            count_y += 1;
        } else {
            flips = std::cmp::min(flips + 1, count_y);
        }
    }
    flips
}

fn main() {
    println!("{}", min_flips("xyxxxyxyy")); // 2
}

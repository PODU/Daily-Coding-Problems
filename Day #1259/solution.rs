// Min flips so all x before all y: single-pass DP. flips=min(flips+1, yCount) on 'x', yCount++ on 'y'. O(n) time, O(1) space.

fn min_flips(s: &str) -> i32 {
    let mut flips = 0;
    let mut y_count = 0;
    for c in s.chars() {
        if c == 'y' {
            y_count += 1;
        } else {
            flips = std::cmp::min(flips + 1, y_count);
        }
    }
    flips
}

fn main() {
    println!("{}", min_flips("xyxxxyxyy"));
}

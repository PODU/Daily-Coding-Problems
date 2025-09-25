// Min flips so all x's precede all y's. Greedy: res=min(res+1, yCount).
// Time O(n), Space O(1).
fn min_flips(s: &str) -> i32 {
    let mut res = 0;
    let mut y_count = 0;
    for ch in s.chars() {
        if ch == 'y' {
            y_count += 1;
        } else {
            res = (res + 1).min(y_count);
        }
    }
    res
}

fn main() {
    println!("{}", min_flips("xyxxxyxyy"));
}

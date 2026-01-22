// Day 937: Valid parenthesis string with '*' (= '(' , ')' or empty).
// Greedy: track [lo,hi] range of possible open counts. Valid iff lo can reach 0. O(n) time, O(1) space.

fn is_valid(s: &str) -> bool {
    let (mut lo, mut hi) = (0i32, 0i32);
    for c in s.chars() {
        match c {
            '(' => { lo += 1; hi += 1; }
            ')' => { lo -= 1; hi -= 1; }
            _ => { lo -= 1; hi += 1; } // '*'
        }
        if hi < 0 {
            return false;
        }
        if lo < 0 {
            lo = 0;
        }
    }
    lo == 0
}

fn main() {
    let inputs = ["(()*", "(*)", ")*("];
    let bal: Vec<&str> = inputs.iter().cloned().filter(|s| is_valid(s)).collect();
    let notbal: Vec<&str> = inputs.iter().cloned().filter(|s| !is_valid(s)).collect();
    println!("{} are balanced. {} is not balanced.", bal.join(" and "), notbal.join(" and "));
    // (()* and (*) are balanced. )*( is not balanced.
}

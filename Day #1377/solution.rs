// Balanced parens with '*' wildcard: greedy track [lo,hi] of possible open counts.
// Time O(n), Space O(1).
fn is_valid(s: &str) -> bool {
    let (mut lo, mut hi) = (0i32, 0i32);
    for c in s.chars() {
        match c {
            '(' => { lo += 1; hi += 1; }
            ')' => { lo -= 1; hi -= 1; }
            _ => { lo -= 1; hi += 1; }
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
    let tests = ["(()*", "(*)", ")*("];
    let bal: Vec<&str> = tests.iter().cloned().filter(|s| is_valid(s)).collect();
    let not_bal: Vec<&str> = tests.iter().cloned().filter(|s| !is_valid(s)).collect();
    println!("{} are balanced. {} is not balanced.", bal.join(" and "), not_bal.join(" and "));
}

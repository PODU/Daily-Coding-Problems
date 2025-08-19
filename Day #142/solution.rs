// Balanced parens with '*' wildcard: track range [lo,hi] of possible open counts. O(n) time, O(1) space.

fn is_balanced(s: &str) -> bool {
    let (mut lo, mut hi) = (0i32, 0i32);
    for c in s.chars() {
        match c {
            '(' => { lo += 1; hi += 1; }
            ')' => { lo -= 1; hi -= 1; }
            _ => { lo -= 1; hi += 1; } // '*' as ')', '(' or empty
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
    let (a, b, c) = ("(()*", "(*)", ")*(");
    let sa = if is_balanced(a) { "(()*" } else { "" };
    let sb = if is_balanced(b) { "(*)" } else { "" };
    let sc = if !is_balanced(c) { ")*(" } else { "" };
    println!("{} and {} are balanced. {} is not balanced.", sa, sb, sc);
}

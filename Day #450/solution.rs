// Day 450: Balanced parentheses with '*' wildcards via greedy low/high open
// count. O(n) time, O(1) space.

fn is_balanced(s: &str) -> bool {
    let mut low = 0i32;
    let mut high = 0i32;
    for c in s.chars() {
        match c {
            '(' => {
                low += 1;
                high += 1;
            }
            ')' => {
                low -= 1;
                high -= 1;
            }
            _ => {
                low -= 1;
                high += 1;
            }
        }
        if high < 0 {
            return false;
        }
        if low < 0 {
            low = 0;
        }
    }
    low == 0
}

fn main() {
    let s = "(()*";
    println!("{}", if is_balanced(s) { "balanced" } else { "not balanced" }); // balanced
}

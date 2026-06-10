// Greedy: track low/high possible open-paren counts in one pass.
// Time O(n), Space O(1). Balanced iff low reaches 0 at end and high never < 0.

fn is_balanced(s: &str) -> bool {
    let mut low: i32 = 0;
    let mut high: i32 = 0;
    for c in s.chars() {
        match c {
            '(' => { low += 1; high += 1; }
            ')' => { low -= 1; high -= 1; }
            _ => { low -= 1; high += 1; }
        }
        if high < 0 { return false; }
        if low < 0 { low = 0; }
    }
    low == 0
}

fn main() {
    let (a, b, c) = ("(()*", "(*)", ")*(");
    let (ra, rb, rc) = (is_balanced(a), is_balanced(b), is_balanced(c));
    let first = if ra && rb { "balanced" } else { "not balanced" };
    let second = if rc { "balanced" } else { "not balanced" };
    println!("{} and {} are {}. {} is {}.", a, b, first, c, second);
}

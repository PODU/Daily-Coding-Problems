// Scan parenthesis string, track open-paren depth, record maximum. Time O(n), Space O(1).
fn tree_depth(s: &str) -> i32 {
    let mut depth = 0;
    let mut max_depth = 0;
    for c in s.chars() {
        if c == '(' {
            depth += 1;
            if depth > max_depth {
                max_depth = depth;
            }
        } else if c == ')' {
            depth -= 1;
        }
    }
    max_depth
}

fn main() {
    println!("{}", tree_depth("((((00)0)0)0)"));
}

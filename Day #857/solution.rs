// Day 857: Depth of tree from (lr) string representation.
// Approach: depth equals the maximum nesting level of parentheses.
// Time: O(n), Space: O(1).
fn depth(s: &str) -> i32 {
    let mut cur = 0;
    let mut mx = 0;
    for c in s.chars() {
        if c == '(' {
            cur += 1;
            if cur > mx {
                mx = cur;
            }
        } else if c == ')' {
            cur -= 1;
        }
    }
    mx
}

fn main() {
    println!("{}", depth("(00)"));          // 1
    println!("{}", depth("((00)(00))"));    // 2
    println!("{}", depth("((((00)0)0)0)")); // 4
}

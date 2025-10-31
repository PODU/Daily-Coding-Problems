// Tree depth = max nesting level of '(' in the representation. O(n) time, O(1) space.
fn tree_depth(s: &str) -> i32 {
    let (mut depth, mut best) = (0, 0);
    for c in s.chars() {
        match c {
            '(' => {
                depth += 1;
                if depth > best {
                    best = depth;
                }
            }
            ')' => depth -= 1,
            _ => {}
        }
    }
    best
}

fn main() {
    println!("{}", tree_depth("((((00)0)0)0)")); // 4
}

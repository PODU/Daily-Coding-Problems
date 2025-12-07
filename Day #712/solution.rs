// Day 712: Balanced brackets check using a stack. Time O(n), space O(n).
fn balanced(s: &str) -> bool {
    let mut st = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => st.push(c),
            ')' => if st.pop() != Some('(') { return false; },
            ']' => if st.pop() != Some('[') { return false; },
            '}' => if st.pop() != Some('{') { return false; },
            _ => {}
        }
    }
    st.is_empty()
}

fn main() {
    println!("{}", balanced("([])[]({})"));
    println!("{}", balanced("([)]"));
    println!("{}", balanced("((()"));
}

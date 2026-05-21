// Balanced brackets check using a stack.
// Time O(n), Space O(n).
fn is_balanced(s: &str) -> bool {
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
    println!("{}", is_balanced("([])[]({})"));
}

// Day 222: Normalize an absolute path (resolve . and ..).
// Approach: split on '/', push names onto a stack, pop on "..", skip "." and "". Time O(n), Space O(n).
fn simplify_path(path: &str) -> String {
    let mut st: Vec<&str> = Vec::new();
    for tok in path.split('/') {
        if tok.is_empty() || tok == "." {
            continue;
        }
        if tok == ".." {
            st.pop();
        } else {
            st.push(tok);
        }
    }
    if st.is_empty() {
        return "/".to_string();
    }
    format!("/{}/", st.join("/")) // trailing slash (directory form)
}

fn main() {
    println!("\"{}\"", simplify_path("/usr/bin/../bin/./scripts/../")); // "/usr/bin/"
}

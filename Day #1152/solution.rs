// Day 1152: Simplify absolute Unix path.
// Stack of components; '.' ignored, '..' pops. Time O(n), Space O(n).
fn simplify(path: &str) -> String {
    let mut st: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => continue,
            ".." => {
                st.pop();
            }
            _ => st.push(part),
        }
    }
    let mut res = String::from("/");
    for p in &st {
        res.push_str(p);
        res.push('/');
    }
    res // trailing slash preserved
}

fn main() {
    println!("{}", simplify("/usr/bin/../bin/./scripts/../")); // /usr/bin/
}

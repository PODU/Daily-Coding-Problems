// Day 961: Normalize an absolute Unix path resolving "." and "..".
// Approach: split by '/', use a stack. Time O(n), Space O(n).

fn simplify_path(path: &str) -> String {
    let mut stack: Vec<&str> = Vec::new();
    for seg in path.split('/') {
        match seg {
            "" | "." => continue,
            ".." => { stack.pop(); }
            _ => stack.push(seg),
        }
    }
    let mut res = format!("/{}", stack.join("/"));
    if path.ends_with('/') && res != "/" {
        res.push('/');
    }
    res
}

fn main() {
    println!("\"{}\"", simplify_path("/usr/bin/../bin/./scripts/../"));
}

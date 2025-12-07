// Day 713: Normalize absolute Unix path. Split on '/', use a stack: skip ""/".",
// pop on "..". Preserve a trailing slash if the input had one. Time O(n).
fn normalize(path: &str) -> String {
    let mut stk: Vec<&str> = Vec::new();
    for comp in path.split('/') {
        if comp.is_empty() || comp == "." {
            continue;
        }
        if comp == ".." {
            stk.pop();
        } else {
            stk.push(comp);
        }
    }
    let mut res = format!("/{}", stk.join("/"));
    let trailing = path.len() > 1 && path.ends_with('/');
    if trailing && res != "/" && !res.ends_with('/') {
        res.push('/');
    }
    res
}

fn main() {
    println!("\"{}\"", normalize("/usr/bin/../bin/./scripts/../"));
}

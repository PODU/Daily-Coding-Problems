// Simplify Unix absolute path: split on '/', push names on a stack, skip ''/'.', pop on '..'. Time O(n), Space O(n).
// Build "/a/b" from the stack; if input ended with '/' and result isn't root, append a trailing '/'.

fn simplify_path(path: &str) -> String {
    let mut stack: Vec<&str> = Vec::new();
    for token in path.split('/') {
        if token.is_empty() || token == "." {
            continue;
        }
        if token == ".." {
            stack.pop();
        } else {
            stack.push(token);
        }
    }
    let mut result = String::from("/");
    result.push_str(&stack.join("/"));
    let ends_with_slash = path.ends_with('/');
    if ends_with_slash && result != "/" {
        result.push('/');
    }
    result
}

fn main() {
    println!("{}", simplify_path("/usr/bin/../bin/./scripts/../"));
}

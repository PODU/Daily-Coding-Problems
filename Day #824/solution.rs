// Merge two binary trees recursively; node value = sum, missing nodes taken from the other.
// Time: O(n), Space: O(h) recursion.

type Link = Option<Box<Node>>;

struct Node {
    val: i64,
    left: Link,
    right: Link,
}

fn node(val: i64, left: Link, right: Link) -> Link {
    Some(Box::new(Node { val, left, right }))
}

fn merge(a: Link, b: Link) -> Link {
    match (a, b) {
        (None, b) => b,
        (a, None) => a,
        (Some(x), Some(y)) => node(
            x.val + y.val,
            merge(x.left, y.left),
            merge(x.right, y.right),
        ),
    }
}

fn preorder(n: &Link, out: &mut Vec<i64>) {
    if let Some(b) = n {
        out.push(b.val);
        preorder(&b.left, out);
        preorder(&b.right, out);
    }
}

fn main() {
    let t1 = node(1, node(3, node(5, None, None), None), node(2, None, None));
    let t2 = node(
        2,
        node(1, None, node(4, None, None)),
        node(3, None, node(7, None, None)),
    );
    let m = merge(t1, t2);
    let mut out: Vec<i64> = Vec::new();
    preorder(&m, &mut out);
    let parts: Vec<String> = out.iter().map(|v| v.to_string()).collect();
    println!("[{}]", parts.join(", "));
}

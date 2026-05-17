// Invert (mirror) a binary tree by swapping left/right children of every node.
// Time O(n), Space O(h) recursion.
type Link = Option<Box<Node>>;

struct Node {
    val: char,
    l: Link,
    r: Link,
}

impl Node {
    fn new(val: char, l: Link, r: Link) -> Link {
        Some(Box::new(Node { val, l, r }))
    }
}

fn invert(node: &mut Link) {
    if let Some(n) = node {
        std::mem::swap(&mut n.l, &mut n.r);
        invert(&mut n.l);
        invert(&mut n.r);
    }
}

fn preorder(node: &Link, out: &mut Vec<char>) {
    if let Some(n) = node {
        out.push(n.val);
        preorder(&n.l, out);
        preorder(&n.r, out);
    }
}

fn main() {
    let mut a = Node::new(
        'a',
        Node::new('b', Node::new('d', None, None), Node::new('e', None, None)),
        Node::new('c', Node::new('f', None, None), None),
    );
    let mut before = Vec::new();
    preorder(&a, &mut before);
    invert(&mut a);
    let mut after = Vec::new();
    preorder(&a, &mut after);
    let join = |v: &Vec<char>| v.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" ");
    println!("before (preorder): {}", join(&before));
    println!("after  (preorder): {}", join(&after));
}

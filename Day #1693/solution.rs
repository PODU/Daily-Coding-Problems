// Ternary Search Tree: node has char + left/mid/right + isEnd. Compare char: <left, >right, ==mid & advance.
// Insert/search: O(L * log A) average where L=key length, A=alphabet size.

struct Node {
    c: u8,
    is_end: bool,
    left: Option<Box<Node>>,
    mid: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(c: u8) -> Box<Node> {
        Box::new(Node { c, is_end: false, left: None, mid: None, right: None })
    }
}

fn insert(node: Option<Box<Node>>, s: &[u8], i: usize) -> Option<Box<Node>> {
    let ch = s[i];
    let mut n = node.unwrap_or_else(|| Node::new(ch));
    if ch < n.c {
        n.left = insert(n.left.take(), s, i);
    } else if ch > n.c {
        n.right = insert(n.right.take(), s, i);
    } else if i + 1 < s.len() {
        n.mid = insert(n.mid.take(), s, i + 1);
    } else {
        n.is_end = true;
    }
    Some(n)
}

fn search(node: &Option<Box<Node>>, s: &[u8], i: usize) -> bool {
    match node {
        None => false,
        Some(n) => {
            let ch = s[i];
            if ch < n.c {
                search(&n.left, s, i)
            } else if ch > n.c {
                search(&n.right, s, i)
            } else if i + 1 == s.len() {
                n.is_end
            } else {
                search(&n.mid, s, i + 1)
            }
        }
    }
}

fn main() {
    let mut root: Option<Box<Node>> = None;
    for w in ["code", "cob", "be", "ax", "war", "we"] {
        root = insert(root.take(), w.as_bytes(), 0);
    }

    for q in ["code", "cob", "cod", "war", "wa"] {
        println!("{}", search(&root, q.as_bytes(), 0));
    }
}

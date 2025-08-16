// Day 133: Inorder successor in a BST using parent pointers.
// If right subtree exists, leftmost of it; else climb until node is a left child. O(h) time.
// Arena (index) representation to model parent pointers safely in Rust.
struct Node {
    val: i32,
    left: Option<usize>,
    right: Option<usize>,
    parent: Option<usize>,
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn new() -> Self {
        Tree { nodes: Vec::new() }
    }
    fn add(&mut self, val: i32) -> usize {
        self.nodes.push(Node { val, left: None, right: None, parent: None });
        self.nodes.len() - 1
    }
    fn set_left(&mut self, p: usize, c: usize) {
        self.nodes[p].left = Some(c);
        self.nodes[c].parent = Some(p);
    }
    fn set_right(&mut self, p: usize, c: usize) {
        self.nodes[p].right = Some(c);
        self.nodes[c].parent = Some(p);
    }
    fn successor(&self, mut node: usize) -> Option<usize> {
        if let Some(mut c) = self.nodes[node].right {
            while let Some(l) = self.nodes[c].left {
                c = l;
            }
            return Some(c);
        }
        let mut p = self.nodes[node].parent;
        while let Some(pp) = p {
            if self.nodes[pp].right == Some(node) {
                node = pp;
                p = self.nodes[pp].parent;
            } else {
                break;
            }
        }
        p
    }
}

fn main() {
    let mut t = Tree::new();
    let root = t.add(10);
    let n5 = t.add(5);
    let n30 = t.add(30);
    let n22 = t.add(22);
    let n35 = t.add(35);
    t.set_left(root, n5);
    t.set_right(root, n30);
    t.set_left(n30, n22);
    t.set_right(n30, n35);

    match t.successor(n22) {
        Some(i) => println!("{}", t.nodes[i].val), // 30
        None => println!("null"),
    }
}

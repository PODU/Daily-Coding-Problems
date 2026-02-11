// Inorder successor in a BST using parent pointers.
// If node has right subtree -> leftmost of right subtree; else walk up parents
// until coming from a left child. Time O(h), Space O(1).
// Uses arena (Vec) with index "pointers" to model parent links safely in Rust.

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
    fn inorder_successor(&self, node: usize) -> Option<usize> {
        if let Some(r) = self.nodes[node].right {
            let mut cur = r;
            while let Some(l) = self.nodes[cur].left {
                cur = l;
            }
            return Some(cur);
        }
        let mut cur = node;
        let mut p = self.nodes[node].parent;
        while let Some(pi) = p {
            if self.nodes[pi].right == Some(cur) {
                cur = pi;
                p = self.nodes[pi].parent;
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

    t.nodes[root].left = Some(n5);
    t.nodes[n5].parent = Some(root);
    t.nodes[root].right = Some(n30);
    t.nodes[n30].parent = Some(root);
    t.nodes[n30].left = Some(n22);
    t.nodes[n22].parent = Some(n30);
    t.nodes[n30].right = Some(n35);
    t.nodes[n35].parent = Some(n30);

    match t.inorder_successor(n22) {
        Some(s) => println!("{}", t.nodes[s].val),
        None => println!("null"),
    }
}

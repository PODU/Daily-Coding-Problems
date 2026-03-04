// Day 1147: Locking in a binary tree.
// Arena of nodes (indices) with parent links + locked-descendant counts; ops walk ancestors. O(h).
struct Node {
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    locked: bool,
    locked_desc: i32,
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn new() -> Self { Tree { nodes: Vec::new() } }
    fn add(&mut self, parent: Option<usize>) -> usize {
        self.nodes.push(Node { parent, left: None, right: None, locked: false, locked_desc: 0 });
        self.nodes.len() - 1
    }
    fn is_locked(&self, i: usize) -> bool { self.nodes[i].locked }
    fn can_lock(&self, i: usize) -> bool {
        if self.nodes[i].locked || self.nodes[i].locked_desc > 0 {
            return false;
        }
        let mut a = self.nodes[i].parent;
        while let Some(p) = a {
            if self.nodes[p].locked { return false; }
            a = self.nodes[p].parent;
        }
        true
    }
    fn lock(&mut self, i: usize) -> bool {
        if !self.can_lock(i) { return false; }
        self.nodes[i].locked = true;
        let mut a = self.nodes[i].parent;
        while let Some(p) = a {
            self.nodes[p].locked_desc += 1;
            a = self.nodes[p].parent;
        }
        true
    }
    fn unlock(&mut self, i: usize) -> bool {
        if !self.nodes[i].locked { return false; }
        self.nodes[i].locked = false;
        let mut a = self.nodes[i].parent;
        while let Some(p) = a {
            self.nodes[p].locked_desc -= 1;
            a = self.nodes[p].parent;
        }
        true
    }
}

fn main() {
    let mut t = Tree::new();
    let root = t.add(None);
    let l = t.add(Some(root));
    let _r = t.add(Some(root));
    let ll = t.add(Some(l));
    t.nodes[root].left = Some(l);
    t.nodes[l].left = Some(ll);
    println!("{}", t.lock(l));    // true
    println!("{}", t.lock(ll));   // false
    println!("{}", t.lock(root)); // false
    println!("{}", t.unlock(l));  // true
    println!("{}", t.lock(ll));   // true
    let _ = t.is_locked(ll);
}

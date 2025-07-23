// Binary tree locking: each node has a parent pointer and locked_descendant_count.
// lock/unlock check ancestors (O(h)) + descendant count, then update ancestors (O(h)).
// Uses an arena (Vec of nodes) with index links so parent walks are safe in Rust.

struct Node {
    parent: Option<usize>,
    locked: bool,
    locked_descendant_count: i32,
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn new() -> Self {
        Tree { nodes: Vec::new() }
    }

    fn add(&mut self, parent: Option<usize>) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node {
            parent,
            locked: false,
            locked_descendant_count: 0,
        });
        id
    }

    fn is_locked(&self, id: usize) -> bool {
        self.nodes[id].locked
    }

    fn can_lock(&self, id: usize) -> bool {
        if self.nodes[id].locked_descendant_count > 0 {
            return false;
        }
        let mut cur = self.nodes[id].parent;
        while let Some(p) = cur {
            if self.nodes[p].locked {
                return false;
            }
            cur = self.nodes[p].parent;
        }
        true
    }

    fn lock(&mut self, id: usize) -> bool {
        if self.nodes[id].locked || !self.can_lock(id) {
            return false;
        }
        self.nodes[id].locked = true;
        let mut cur = self.nodes[id].parent;
        while let Some(p) = cur {
            self.nodes[p].locked_descendant_count += 1;
            cur = self.nodes[p].parent;
        }
        true
    }

    fn unlock(&mut self, id: usize) -> bool {
        if !self.nodes[id].locked {
            return false;
        }
        self.nodes[id].locked = false;
        let mut cur = self.nodes[id].parent;
        while let Some(p) = cur {
            self.nodes[p].locked_descendant_count -= 1;
            cur = self.nodes[p].parent;
        }
        true
    }
}

fn main() {
    let mut tree = Tree::new();
    let root = tree.add(None);
    let node1 = tree.add(Some(root));
    let node2 = tree.add(Some(root));
    let _node3 = tree.add(Some(node1));
    let _node4 = tree.add(Some(node1));

    let _ = root;
    println!("node2.lock() = {}", tree.lock(node2));
    println!("root.lock() = {}", tree.lock(root));
    println!("node2.unlock() = {}", tree.unlock(node2));
    println!("root.lock() = {}", tree.lock(root));
}

// Locking binary tree: each node tracks lockedDescendants count; lock/unlock check
// ancestors + descendant count. All ops O(h) where h is tree height.
// Uses an arena (Vec) with index "pointers" to satisfy Rust's ownership model.
use std::cell::RefCell;

#[derive(Default)]
struct Node {
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    locked: bool,
    locked_descendants: i32,
}

struct Tree {
    nodes: Vec<RefCell<Node>>,
}

impl Tree {
    fn new() -> Self {
        Tree { nodes: Vec::new() }
    }

    fn add(&mut self, parent: Option<usize>) -> usize {
        let mut n = Node::default();
        n.parent = parent;
        self.nodes.push(RefCell::new(n));
        self.nodes.len() - 1
    }

    fn is_locked(&self, id: usize) -> bool {
        self.nodes[id].borrow().locked
    }

    fn any_ancestor_locked(&self, id: usize) -> bool {
        let mut cur = self.nodes[id].borrow().parent;
        while let Some(p) = cur {
            if self.nodes[p].borrow().locked {
                return true;
            }
            cur = self.nodes[p].borrow().parent;
        }
        false
    }

    fn lock(&self, id: usize) -> bool {
        {
            let n = self.nodes[id].borrow();
            if n.locked || n.locked_descendants > 0 {
                return false;
            }
        }
        if self.any_ancestor_locked(id) {
            return false;
        }
        self.nodes[id].borrow_mut().locked = true;
        let mut cur = self.nodes[id].borrow().parent;
        while let Some(p) = cur {
            self.nodes[p].borrow_mut().locked_descendants += 1;
            cur = self.nodes[p].borrow().parent;
        }
        true
    }

    fn unlock(&self, id: usize) -> bool {
        if !self.nodes[id].borrow().locked {
            return false;
        }
        self.nodes[id].borrow_mut().locked = false;
        let mut cur = self.nodes[id].borrow().parent;
        while let Some(p) = cur {
            self.nodes[p].borrow_mut().locked_descendants -= 1;
            cur = self.nodes[p].borrow().parent;
        }
        true
    }
}

fn main() {
    let mut t = Tree::new();
    let root = t.add(None);
    let a = t.add(Some(root));
    let b = t.add(Some(root));
    let c = t.add(Some(a));
    let d = t.add(Some(a));
    t.nodes[root].borrow_mut().left = Some(a);
    t.nodes[root].borrow_mut().right = Some(b);
    t.nodes[a].borrow_mut().left = Some(c);
    t.nodes[a].borrow_mut().right = Some(d);
    let _ = d; // d unused beyond structure

    println!("lock c (leaf)      -> {} (expect true)", t.lock(c));
    println!("lock a (ancestor)  -> {} (expect false)", t.lock(a));
    println!("lock root          -> {} (expect false)", t.lock(root));
    println!("unlock c           -> {} (expect true)", t.unlock(c));
    println!("lock a             -> {} (expect true)", t.lock(a));
    println!("lock c (desc lock) -> {} (expect false)", t.lock(c));
    println!("unlock a           -> {} (expect true)", t.unlock(a));
    println!("is_locked(a)       -> {}", t.is_locked(a));
}

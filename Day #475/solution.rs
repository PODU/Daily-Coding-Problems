// Tree locking with parent pointers + per-node locked_descendant_count.
// lock/unlock are O(h): walk ancestors once to check, once to update counts.
// Uses an arena (Vec of nodes) with index "pointers" to model parent links safely.

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

    #[allow(dead_code)]
    fn is_locked(&self, id: usize) -> bool {
        self.nodes[id].locked
    }

    fn any_ancestor_locked(&self, id: usize) -> bool {
        let mut p = self.nodes[id].parent;
        while let Some(pi) = p {
            if self.nodes[pi].locked {
                return true;
            }
            p = self.nodes[pi].parent;
        }
        false
    }

    fn lock(&mut self, id: usize) -> bool {
        if self.nodes[id].locked {
            return false;
        }
        if self.nodes[id].locked_descendant_count > 0 {
            return false; // a descendant is locked
        }
        if self.any_ancestor_locked(id) {
            return false; // an ancestor is locked
        }
        self.nodes[id].locked = true;
        let mut p = self.nodes[id].parent;
        while let Some(pi) = p {
            self.nodes[pi].locked_descendant_count += 1;
            p = self.nodes[pi].parent;
        }
        true
    }

    fn unlock(&mut self, id: usize) -> bool {
        if !self.nodes[id].locked {
            return false;
        }
        self.nodes[id].locked = false;
        let mut p = self.nodes[id].parent;
        while let Some(pi) = p {
            self.nodes[pi].locked_descendant_count -= 1;
            p = self.nodes[pi].parent;
        }
        true
    }
}

fn cap(b: bool) -> &'static str {
    if b {
        "True"
    } else {
        "False"
    }
}

fn main() {
    let mut t = Tree::new();
    let n1 = t.add(None);
    let n2 = t.add(Some(n1));
    let _n3 = t.add(Some(n1));
    let n4 = t.add(Some(n2));
    let n5 = t.add(Some(n2));

    println!("lock(node4): {}", cap(t.lock(n4)));     // True
    println!("lock(node2): {}", cap(t.lock(n2)));     // False (descendant locked)
    println!("unlock(node4): {}", cap(t.unlock(n4))); // True
    println!("lock(node2): {}", cap(t.lock(n2)));     // True
    println!("lock(node1): {}", cap(t.lock(n1)));     // False (descendant locked)
    println!("lock(node5): {}", cap(t.lock(n5)));     // False (ancestor locked)
    println!("unlock(node2): {}", cap(t.unlock(n2))); // True
    println!("lock(node1): {}", cap(t.lock(n1)));     // True
}

// Day 1625: Inorder successor in BST using parent pointers.
// If right subtree exists, leftmost of it; else climb until node is left child. O(h).
// Uses an arena (Vec) with indices to model parent pointers safely.
struct Tree {
    val: Vec<i32>,
    left: Vec<Option<usize>>,
    right: Vec<Option<usize>>,
    parent: Vec<Option<usize>>,
}

impl Tree {
    fn new() -> Self {
        Tree { val: vec![], left: vec![], right: vec![], parent: vec![] }
    }
    fn alloc(&mut self, v: i32, p: Option<usize>) -> usize {
        self.val.push(v);
        self.left.push(None);
        self.right.push(None);
        self.parent.push(p);
        self.val.len() - 1
    }
    fn insert(&mut self, root: Option<usize>, v: i32) -> usize {
        match root {
            None => self.alloc(v, None),
            Some(r) => {
                let mut cur = r;
                loop {
                    if v < self.val[cur] {
                        match self.left[cur] {
                            Some(n) => cur = n,
                            None => { let n = self.alloc(v, Some(cur)); self.left[cur] = Some(n); return r; }
                        }
                    } else {
                        match self.right[cur] {
                            Some(n) => cur = n,
                            None => { let n = self.alloc(v, Some(cur)); self.right[cur] = Some(n); return r; }
                        }
                    }
                }
            }
        }
    }
    fn find(&self, root: Option<usize>, v: i32) -> Option<usize> {
        let mut cur = root;
        while let Some(c) = cur {
            if self.val[c] == v { return Some(c); }
            cur = if v < self.val[c] { self.left[c] } else { self.right[c] };
        }
        None
    }
    fn successor(&self, node: usize) -> Option<usize> {
        if let Some(mut cur) = self.right[node] {
            while let Some(l) = self.left[cur] { cur = l; }
            return Some(cur);
        }
        let mut cur = node;
        while let Some(p) = self.parent[cur] {
            if self.right[p] == Some(cur) { cur = p; } else { return Some(p); }
        }
        None
    }
}

fn main() {
    let mut t = Tree::new();
    let mut root: Option<usize> = None;
    for v in [10, 5, 30, 22, 35] {
        root = Some(t.insert(root, v));
    }
    let n = t.find(root, 22).unwrap();
    match t.successor(n) {
        Some(s) => println!("The inorder successor of 22 is {}.", t.val[s]),
        None => println!("The inorder successor of 22 is None."),
    }
}

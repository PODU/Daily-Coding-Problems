// Day 777: Ternary Search Tree with insert and search.
// Each node has left/mid/right children. O(L * log A) per op (L=word length).
struct Node {
    c: u8,
    end: bool,
    l: Option<Box<Node>>,
    m: Option<Box<Node>>,
    r: Option<Box<Node>>,
}

impl Node {
    fn new(c: u8) -> Box<Node> {
        Box::new(Node { c, end: false, l: None, m: None, r: None })
    }
}

#[derive(Default)]
struct TST {
    root: Option<Box<Node>>,
}

fn insert(node: &mut Option<Box<Node>>, w: &[u8], i: usize) {
    let c = w[i];
    if node.is_none() {
        *node = Some(Node::new(c));
    }
    let n = node.as_mut().unwrap();
    if c < n.c {
        insert(&mut n.l, w, i);
    } else if c > n.c {
        insert(&mut n.r, w, i);
    } else if i + 1 < w.len() {
        insert(&mut n.m, w, i + 1);
    } else {
        n.end = true;
    }
}

impl TST {
    fn insert(&mut self, w: &str) {
        if !w.is_empty() {
            insert(&mut self.root, w.as_bytes(), 0);
        }
    }

    fn search(&self, w: &str) -> bool {
        if w.is_empty() {
            return false;
        }
        let w = w.as_bytes();
        let mut cur = self.root.as_deref();
        let mut i = 0;
        while let Some(n) = cur {
            let c = w[i];
            if c < n.c {
                cur = n.l.as_deref();
            } else if c > n.c {
                cur = n.r.as_deref();
            } else if i + 1 == w.len() {
                return n.end;
            } else {
                cur = n.m.as_deref();
                i += 1;
            }
        }
        false
    }
}

fn main() {
    let mut t = TST::default();
    for w in ["code", "cob", "be", "ax", "war", "we"] {
        t.insert(w);
    }
    println!("{} {} {} {}", t.search("cob"), t.search("code"), t.search("cod"), t.search("we"));
    // true true false true
}

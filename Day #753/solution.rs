// Day 753: Ternary Search Tree with insert and search.
// Insert/Search: O(L + log n) average, O(L * n) worst; L = key length.
type Link = Option<Box<TstNode>>;

struct TstNode {
    c: u8,
    is_end: bool,
    left: Link,
    mid: Link,
    right: Link,
}

impl TstNode {
    fn new(c: u8) -> Box<TstNode> {
        Box::new(TstNode { c, is_end: false, left: None, mid: None, right: None })
    }
}

struct Tst {
    root: Link,
}

impl Tst {
    fn new() -> Tst { Tst { root: None } }

    fn insert(&mut self, s: &str) {
        if !s.is_empty() {
            let bytes = s.as_bytes();
            Self::insert_rec(&mut self.root, bytes, 0);
        }
    }

    fn insert_rec(link: &mut Link, s: &[u8], i: usize) {
        let c = s[i];
        if link.is_none() {
            *link = Some(TstNode::new(c));
        }
        let node = link.as_mut().unwrap();
        if c < node.c {
            Self::insert_rec(&mut node.left, s, i);
        } else if c > node.c {
            Self::insert_rec(&mut node.right, s, i);
        } else if i + 1 < s.len() {
            Self::insert_rec(&mut node.mid, s, i + 1);
        } else {
            node.is_end = true;
        }
    }

    fn search(&self, s: &str) -> bool {
        let bytes = s.as_bytes();
        let mut cur = &self.root;
        let mut i = 0;
        while let Some(node) = cur {
            let c = bytes[i];
            if c < node.c {
                cur = &node.left;
            } else if c > node.c {
                cur = &node.right;
            } else {
                if i + 1 == bytes.len() {
                    return node.is_end;
                }
                cur = &node.mid;
                i += 1;
            }
        }
        false
    }
}

fn main() {
    let mut tst = Tst::new();
    for w in ["code", "cob", "be", "ax", "war", "we"] {
        tst.insert(w);
    }
    for q in ["code", "cob", "cod", "ax", "hello"] {
        println!("{}: {}", q, tst.search(q));
    }
}

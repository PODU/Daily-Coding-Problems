// Ternary Search Tree with insert/search. Each node: char + left/mid/right + end flag.
// Time: O(L * log A) per op, Space: O(total chars).
struct Node {
    c: u8,
    end: bool,
    left: Option<Box<Node>>,
    mid: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(c: u8) -> Box<Node> {
        Box::new(Node { c, end: false, left: None, mid: None, right: None })
    }
}

#[derive(Default)]
struct Tst {
    root: Option<Box<Node>>,
}

impl Tst {
    fn insert(&mut self, w: &str) {
        if !w.is_empty() {
            Self::ins(&mut self.root, w.as_bytes(), 0);
        }
    }

    fn ins(link: &mut Option<Box<Node>>, w: &[u8], i: usize) {
        let ch = w[i];
        if link.is_none() {
            *link = Some(Node::new(ch));
        }
        let node = link.as_mut().unwrap();
        if ch < node.c {
            Self::ins(&mut node.left, w, i);
        } else if ch > node.c {
            Self::ins(&mut node.right, w, i);
        } else if i + 1 < w.len() {
            Self::ins(&mut node.mid, w, i + 1);
        } else {
            node.end = true;
        }
    }

    fn search(&self, w: &str) -> bool {
        let w = w.as_bytes();
        let mut cur = self.root.as_deref();
        let mut i = 0;
        while let Some(node) = cur {
            let ch = w[i];
            if ch < node.c {
                cur = node.left.as_deref();
            } else if ch > node.c {
                cur = node.right.as_deref();
            } else {
                if i + 1 == w.len() {
                    return node.end;
                }
                i += 1;
                cur = node.mid.as_deref();
            }
        }
        false
    }
}

fn main() {
    let mut tst = Tst::default();
    for w in ["code", "cob", "be", "ax", "war", "we"] {
        tst.insert(w);
    }
    for q in ["code", "cob", "ax", "c", "war", "cat"] {
        println!("{} -> {}", q, tst.search(q));
    }
}

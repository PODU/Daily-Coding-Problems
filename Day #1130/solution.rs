// Largest BST subtree size via post-order DFS returning (isBST,size,min,max). O(n) time.
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    l: Link,
    r: Link,
}

impl Node {
    fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, l: None, r: None }))
    }
}

struct Info { is_bst: bool, size: i32, mn: i32, mx: i32 }

fn dfs(n: &Link, best: &mut i32) -> Info {
    match n {
        None => Info { is_bst: true, size: 0, mn: i32::MAX, mx: i32::MIN },
        Some(node) => {
            let node = node.borrow();
            let l = dfs(&node.l, best);
            let r = dfs(&node.r, best);
            if l.is_bst && r.is_bst && node.val > l.mx && node.val < r.mn {
                let sz = l.size + r.size + 1;
                if sz > *best { *best = sz; }
                return Info {
                    is_bst: true,
                    size: sz,
                    mn: node.val.min(l.mn),
                    mx: node.val.max(r.mx),
                };
            }
            Info { is_bst: false, size: 0, mn: 0, mx: 0 }
        }
    }
}

fn main() {
    let root = Node::new(10);
    let l = Node::new(5);
    let rr = Node::new(15);
    l.borrow_mut().l = Some(Node::new(1));
    l.borrow_mut().r = Some(Node::new(8));
    rr.borrow_mut().r = Some(Node::new(7));
    root.borrow_mut().l = Some(l);
    root.borrow_mut().r = Some(rr);

    let mut best = 0;
    let link: Link = Some(root);
    dfs(&link, &mut best);
    println!("{}", best);
}

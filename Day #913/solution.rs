// Complete-tree node count: if left height == right height subtree is perfect (2^h-1), else recurse. O(log^2 n).
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, left: None, right: None }))
    }
}

fn left_height(node: &Link) -> i32 {
    let mut h = 0;
    let mut cur = node.clone();
    while let Some(n) = cur {
        h += 1;
        cur = n.borrow().left.clone();
    }
    h
}

fn right_height(node: &Link) -> i32 {
    let mut h = 0;
    let mut cur = node.clone();
    while let Some(n) = cur {
        h += 1;
        cur = n.borrow().right.clone();
    }
    h
}

fn count_nodes(root: &Link) -> i32 {
    match root {
        None => 0,
        Some(n) => {
            let lh = left_height(root);
            let rh = right_height(root);
            if lh == rh {
                (1 << lh) - 1 // perfect subtree
            } else {
                1 + count_nodes(&n.borrow().left) + count_nodes(&n.borrow().right)
            }
        }
    }
}

fn main() {
    // Complete tree with 6 nodes (values 1..6 in level order):
    //         1
    //       /   \
    //      2     3
    //     / \   /
    //    4   5 6
    let nodes: Vec<Rc<RefCell<Node>>> = (1..=6).map(Node::new).collect();
    nodes[0].borrow_mut().left = Some(nodes[1].clone());
    nodes[0].borrow_mut().right = Some(nodes[2].clone());
    nodes[1].borrow_mut().left = Some(nodes[3].clone());
    nodes[1].borrow_mut().right = Some(nodes[4].clone());
    nodes[2].borrow_mut().left = Some(nodes[5].clone());

    let root: Link = Some(nodes[0].clone());
    println!("{}", count_nodes(&root)); // 6
}

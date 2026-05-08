// Day 1487: In-order traversal in O(1) auxiliary space via Morris traversal.
// We mutate raw pointers to thread the tree, then restore it. Time O(n), Space O(1).
use std::ptr;

struct Node {
    val: i32,
    left: *mut Node,
    right: *mut Node,
}

impl Node {
    fn new(val: i32) -> *mut Node {
        Box::into_raw(Box::new(Node { val, left: ptr::null_mut(), right: ptr::null_mut() }))
    }
}

fn morris_inorder(root: *mut Node) -> Vec<i32> {
    let mut out = Vec::new();
    let mut cur = root;
    unsafe {
        while !cur.is_null() {
            if (*cur).left.is_null() {
                out.push((*cur).val);
                cur = (*cur).right;
            } else {
                let mut pred = (*cur).left;
                while !(*pred).right.is_null() && (*pred).right != cur {
                    pred = (*pred).right;
                }
                if (*pred).right.is_null() {
                    (*pred).right = cur; // thread
                    cur = (*cur).left;
                } else {
                    (*pred).right = ptr::null_mut(); // restore
                    out.push((*cur).val);
                    cur = (*cur).right;
                }
            }
        }
    }
    out
}

fn free_tree(root: *mut Node) {
    if root.is_null() { return; }
    unsafe {
        free_tree((*root).left);
        free_tree((*root).right);
        drop(Box::from_raw(root));
    }
}

fn main() {
    //        4
    //       / \
    //      2   6
    //     / \ / \
    //    1  3 5  7
    let root = Node::new(4);
    unsafe {
        (*root).left = Node::new(2);
        (*root).right = Node::new(6);
        (*(*root).left).left = Node::new(1);
        (*(*root).left).right = Node::new(3);
        (*(*root).right).left = Node::new(5);
        (*(*root).right).right = Node::new(7);
    }

    let res = morris_inorder(root);
    let parts: Vec<String> = res.iter().map(|v| v.to_string()).collect();
    println!("In-order: {}", parts.join(" "));

    free_tree(root);
}

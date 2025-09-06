// Day 223: In-order traversal with O(1) auxiliary algorithmic space (Morris traversal).
// Approach: thread each node to its in-order predecessor temporarily, remove thread after visiting.
// Uses raw pointers to mutate threads in place (no stack/recursion). Time O(n), Space O(1).
struct Node {
    val: i32,
    left: *mut Node,
    right: *mut Node,
}

fn leaf(v: i32) -> *mut Node {
    Box::into_raw(Box::new(Node { val: v, left: std::ptr::null_mut(), right: std::ptr::null_mut() }))
}

fn morris_inorder(root: *mut Node) -> Vec<i32> {
    let mut res = Vec::new();
    let mut cur = root;
    unsafe {
        while !cur.is_null() {
            if (*cur).left.is_null() {
                res.push((*cur).val);
                cur = (*cur).right;
            } else {
                let mut pred = (*cur).left;
                while !(*pred).right.is_null() && (*pred).right != cur {
                    pred = (*pred).right;
                }
                if (*pred).right.is_null() {
                    (*pred).right = cur; // create thread
                    cur = (*cur).left;
                } else {
                    (*pred).right = std::ptr::null_mut(); // remove thread
                    res.push((*cur).val);
                    cur = (*cur).right;
                }
            }
        }
    }
    res
}

fn main() {
    unsafe {
        let root = leaf(4);
        (*root).left = leaf(2);
        (*root).right = leaf(6);
        (*(*root).left).left = leaf(1);
        (*(*root).left).right = leaf(3);
        (*(*root).right).left = leaf(5);
        (*(*root).right).right = leaf(7);
        println!("{:?}", morris_inorder(root)); // [1, 2, 3, 4, 5, 6, 7]
    }
}

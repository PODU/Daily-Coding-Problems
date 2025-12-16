// Day 751: In-order traversal with O(1) extra space via Morris Traversal.
// Time: O(n), Space: O(1) (re-uses null right pointers as temporary threads).
// Implemented over an arena (Vec) to satisfy Rust's borrow rules while keeping
// O(1) auxiliary traversal space (threads stored in the node's right index).
const NIL: usize = usize::MAX;

struct Node {
    val: i32,
    left: usize,
    right: usize,
}

fn morris_inorder(nodes: &mut [Node], root: usize) -> Vec<i32> {
    let mut out = Vec::new();
    let mut cur = root;
    while cur != NIL {
        if nodes[cur].left == NIL {
            out.push(nodes[cur].val);
            cur = nodes[cur].right;
        } else {
            let mut pre = nodes[cur].left;
            while nodes[pre].right != NIL && nodes[pre].right != cur {
                pre = nodes[pre].right;
            }
            if nodes[pre].right == NIL {
                nodes[pre].right = cur; // create thread
                cur = nodes[cur].left;
            } else {
                nodes[pre].right = NIL; // remove thread and visit
                out.push(nodes[cur].val);
                cur = nodes[cur].right;
            }
        }
    }
    out
}

fn main() {
    // indices: 0:4 1:2 2:6 3:1 4:3 5:5 6:7
    let mut nodes = vec![
        Node { val: 4, left: 1, right: 2 },
        Node { val: 2, left: 3, right: 4 },
        Node { val: 6, left: 5, right: 6 },
        Node { val: 1, left: NIL, right: NIL },
        Node { val: 3, left: NIL, right: NIL },
        Node { val: 5, left: NIL, right: NIL },
        Node { val: 7, left: NIL, right: NIL },
    ];
    let res = morris_inorder(&mut nodes, 0);
    let parts: Vec<String> = res.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}

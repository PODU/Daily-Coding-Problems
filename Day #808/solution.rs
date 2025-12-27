// Day 808: In-order traversal of a binary tree using O(1) extra space (Morris).
// Thread predecessor's right pointer to current, then unthread. Time O(N), Space O(1).
// Nodes held in an arena (Vec); links are Option<usize> indices for safe Rust.

struct Node {
    val: i32,
    left: Option<usize>,
    right: Option<usize>,
}

fn morris_inorder(arena: &mut [Node], root: Option<usize>) -> Vec<i32> {
    let mut out = Vec::new();
    let mut cur = root;
    while let Some(c) = cur {
        if arena[c].left.is_none() {
            out.push(arena[c].val);
            cur = arena[c].right;
        } else {
            let mut pred = arena[c].left.unwrap();
            while arena[pred].right.is_some() && arena[pred].right != Some(c) {
                pred = arena[pred].right.unwrap();
            }
            if arena[pred].right.is_none() {
                arena[pred].right = Some(c);
                cur = arena[c].left;
            } else {
                arena[pred].right = None;
                out.push(arena[c].val);
                cur = arena[c].right;
            }
        }
    }
    out
}

fn main() {
    // indices: 0=4,1=2,2=6,3=1,4=3,5=5,6=7
    let mut arena = vec![
        Node { val: 4, left: Some(1), right: Some(2) },
        Node { val: 2, left: Some(3), right: Some(4) },
        Node { val: 6, left: Some(5), right: Some(6) },
        Node { val: 1, left: None, right: None },
        Node { val: 3, left: None, right: None },
        Node { val: 5, left: None, right: None },
        Node { val: 7, left: None, right: None },
    ];
    println!("{:?}", morris_inorder(&mut arena, Some(0))); // [1, 2, 3, 4, 5, 6, 7]
}

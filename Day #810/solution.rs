// Day 810: Boustrophedon (zigzag) level-order traversal of a binary tree.
// BFS level by level, reversing every other level. Time O(N), Space O(N).
// Nodes held in an arena (Vec); links are Option<usize> indices for safe Rust.

struct Node {
    val: i32,
    left: Option<usize>,
    right: Option<usize>,
}

fn boustrophedon(arena: &[Node], root: Option<usize>) -> Vec<i32> {
    let mut out = Vec::new();
    let mut queue = match root {
        Some(r) => vec![r],
        None => return out,
    };
    let mut ltr = true;
    while !queue.is_empty() {
        let mut next = Vec::new();
        let mut level = Vec::new();
        for &n in &queue {
            level.push(arena[n].val);
            if let Some(l) = arena[n].left {
                next.push(l);
            }
            if let Some(r) = arena[n].right {
                next.push(r);
            }
        }
        if !ltr {
            level.reverse();
        }
        out.extend(level);
        ltr = !ltr;
        queue = next;
    }
    out
}

fn main() {
    // indices: 0=1,1=2,2=3,3=4,4=5,5=6,6=7
    let arena = vec![
        Node { val: 1, left: Some(1), right: Some(2) },
        Node { val: 2, left: Some(3), right: Some(4) },
        Node { val: 3, left: Some(5), right: Some(6) },
        Node { val: 4, left: None, right: None },
        Node { val: 5, left: None, right: None },
        Node { val: 6, left: None, right: None },
        Node { val: 7, left: None, right: None },
    ];
    println!("{:?}", boustrophedon(&arena, Some(0))); // [1, 3, 2, 4, 5, 6, 7]
}

// Cousins: BFS tracking parent & depth; find target's depth+parent, collect nodes at
// same depth with different parent. Time O(n), Space O(n). Arena-based tree.
struct Node {
    val: i32,
    left: Option<usize>,
    right: Option<usize>,
}

fn cousins(arena: &[Node], root: usize, target: i32) -> Vec<i32> {
    // level: Vec of (node_idx, parent_idx)
    let mut level: Vec<(usize, Option<usize>)> = vec![(root, None)];
    while !level.is_empty() {
        let mut target_parent: Option<usize> = None;
        let mut found = false;
        let mut next: Vec<(usize, Option<usize>)> = Vec::new();
        for &(idx, par) in &level {
            if arena[idx].val == target {
                target_parent = par;
                found = true;
            }
            if let Some(l) = arena[idx].left {
                next.push((l, Some(idx)));
            }
            if let Some(r) = arena[idx].right {
                next.push((r, Some(idx)));
            }
        }
        if found {
            return level
                .iter()
                .filter(|&&(_, par)| par != target_parent)
                .map(|&(idx, _)| arena[idx].val)
                .collect();
        }
        level = next;
    }
    Vec::new()
}

fn main() {
    // arena indices: 0=1,1=2,2=3,3=4,4=5,5=6
    let arena = vec![
        Node { val: 1, left: Some(1), right: Some(2) },
        Node { val: 2, left: Some(3), right: Some(4) },
        Node { val: 3, left: None, right: Some(5) },
        Node { val: 4, left: None, right: None },
        Node { val: 5, left: None, right: None },
        Node { val: 6, left: None, right: None },
    ];
    println!("{:?}", cousins(&arena, 0, 4));
}

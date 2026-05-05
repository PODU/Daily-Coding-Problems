// Day 1478: Return a deepest node of a binary tree.
// Single DFS returning (depth, value); keep the deeper subtree's result.
// Time O(N), Space O(H).

struct Node {
    val: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn dfs(node: &Node) -> (i32, char) {
    let (ld, lv) = match &node.left {
        Some(l) => dfs(l),
        None => (0, node.val),
    };
    let (rd, rv) = match &node.right {
        Some(r) => dfs(r),
        None => (0, node.val),
    };
    if node.left.is_none() && node.right.is_none() {
        (0, node.val)
    } else if ld >= rd {
        (ld + 1, if node.left.is_some() { lv } else { node.val })
    } else {
        (rd + 1, rv)
    }
}

fn main() {
    let root = Node {
        val: 'a',
        left: Some(Box::new(Node {
            val: 'b',
            left: Some(Box::new(Node { val: 'd', left: None, right: None })),
            right: None,
        })),
        right: Some(Box::new(Node { val: 'c', left: None, right: None })),
    };
    let (_, v) = dfs(&root);
    println!("{}", v); // d
}

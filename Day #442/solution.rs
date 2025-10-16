// Day 442: Cartesian tree (min-heap ordered, in-order == S) built with a
// monotonic stack in O(n) time, O(n) space. Uses an arena of indices.

struct Node {
    val: i32,
    left: Option<usize>,
    right: Option<usize>,
}

struct Tree {
    nodes: Vec<Node>,
    root: Option<usize>,
}

fn build_cartesian(s: &[i32]) -> Tree {
    let mut nodes: Vec<Node> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    for &v in s {
        let idx = nodes.len();
        nodes.push(Node { val: v, left: None, right: None });
        let mut last: Option<usize> = None;
        while let Some(&top) = stack.last() {
            if nodes[top].val > v {
                last = stack.pop();
            } else {
                break;
            }
        }
        nodes[idx].left = last;
        if let Some(&top) = stack.last() {
            nodes[top].right = Some(idx);
        }
        stack.push(idx);
    }
    let root = if stack.is_empty() { None } else { Some(stack[0]) };
    Tree { nodes, root }
}

fn show(tree: &Tree, n: Option<usize>, prefix: &str, tag: &str) {
    if let Some(i) = n {
        println!("{}{}{}", prefix, tag, tree.nodes[i].val);
        let child_prefix = format!("{}  ", prefix);
        show(tree, tree.nodes[i].left, &child_prefix, "L-- ");
        show(tree, tree.nodes[i].right, &child_prefix, "R-- ");
    }
}

fn main() {
    let tree = build_cartesian(&[3, 2, 6, 1, 9]);
    show(&tree, tree.root, "", "");
    // 1
    //   L-- 2
    //     L-- 3
    //     R-- 6
    //   R-- 9
}

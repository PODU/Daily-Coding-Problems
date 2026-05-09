// Day 1495: Build a min-heap-ordered Cartesian tree whose in-order traversal is S.
// Approach: monotonic stack; pop nodes > current as its left subtree. Time O(n), Space O(n).
// Nodes are stored in an arena (Vec) referenced by index to avoid borrow-checker churn.

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
    for &x in s {
        let cur = nodes.len();
        nodes.push(Node { val: x, left: None, right: None });
        let mut last: Option<usize> = None;
        while let Some(&top) = stack.last() {
            if nodes[top].val > x {
                last = stack.pop();
            } else {
                break;
            }
        }
        nodes[cur].left = last;
        if let Some(&top) = stack.last() {
            nodes[top].right = Some(cur);
        }
        stack.push(cur);
    }
    let root = stack.first().copied();
    Tree { nodes, root }
}

fn inorder(t: &Tree, n: Option<usize>, out: &mut Vec<i32>) {
    if let Some(i) = n {
        inorder(t, t.nodes[i].left, out);
        out.push(t.nodes[i].val);
        inorder(t, t.nodes[i].right, out);
    }
}

fn pretty(t: &Tree, n: Option<usize>, depth: usize) {
    if let Some(i) = n {
        pretty(t, t.nodes[i].right, depth + 1);
        println!("{}{}", " ".repeat(depth * 4), t.nodes[i].val);
        pretty(t, t.nodes[i].left, depth + 1);
    }
}

fn listing(t: &Tree, n: Option<usize>) {
    if let Some(i) = n {
        let (l, r) = (t.nodes[i].left, t.nodes[i].right);
        if l.is_some() || r.is_some() {
            let mut kids: Vec<String> = Vec::new();
            if let Some(li) = l { kids.push(t.nodes[li].val.to_string()); }
            if let Some(ri) = r { kids.push(t.nodes[ri].val.to_string()); }
            println!("  {} -> {}", t.nodes[i].val, kids.join(" "));
        }
        listing(t, l);
        listing(t, r);
    }
}

fn main() {
    let s = [3, 2, 6, 1, 9];
    let t = build_cartesian(&s);

    let mut io = Vec::new();
    inorder(&t, t.root, &mut io);
    let parts: Vec<String> = io.iter().map(|v| v.to_string()).collect();
    println!("In-order traversal: {}", parts.join(" "));

    let root = t.root.unwrap();
    println!("Root: {}", t.nodes[root].val);
    println!("Parent -> children:");
    listing(&t, t.root);

    println!("Tree (rotated 90 deg, root at left):");
    pretty(&t, t.root, 0);
}

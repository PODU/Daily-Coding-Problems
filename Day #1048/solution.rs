// Cartesian tree (min-heap ordered, in-order = S) via linear stack on right spine.
// Build O(n) using arena indices; rotated-90 print + verification. Time O(n), Space O(n).

struct Node {
    val: i32,
    left: Option<usize>,
    right: Option<usize>,
}

struct Tree {
    nodes: Vec<Node>,
    root: Option<usize>,
}

fn build(s: &[i32]) -> Tree {
    let mut nodes: Vec<Node> = Vec::new();
    let mut st: Vec<usize> = Vec::new();
    for &v in s {
        let cur = nodes.len();
        nodes.push(Node { val: v, left: None, right: None });
        let mut last: Option<usize> = None;
        while let Some(&top) = st.last() {
            if nodes[top].val > v {
                last = st.pop();
            } else {
                break;
            }
        }
        nodes[cur].left = last;
        if let Some(&top) = st.last() {
            nodes[top].right = Some(cur);
        }
        st.push(cur);
    }
    let root = st.first().copied();
    Tree { nodes, root }
}

fn print_rotated(t: &Tree, n: Option<usize>, depth: usize) {
    if let Some(idx) = n {
        print_rotated(t, t.nodes[idx].right, depth + 1);
        println!("{}{}", " ".repeat(depth * 4), t.nodes[idx].val);
        print_rotated(t, t.nodes[idx].left, depth + 1);
    }
}

fn inorder(t: &Tree, n: Option<usize>, out: &mut Vec<i32>) {
    if let Some(idx) = n {
        inorder(t, t.nodes[idx].left, out);
        out.push(t.nodes[idx].val);
        inorder(t, t.nodes[idx].right, out);
    }
}

fn min_heap(t: &Tree, n: Option<usize>) -> bool {
    if let Some(idx) = n {
        if let Some(l) = t.nodes[idx].left {
            if t.nodes[l].val <= t.nodes[idx].val {
                return false;
            }
        }
        if let Some(r) = t.nodes[idx].right {
            if t.nodes[r].val <= t.nodes[idx].val {
                return false;
            }
        }
        return min_heap(t, t.nodes[idx].left) && min_heap(t, t.nodes[idx].right);
    }
    true
}

fn main() {
    let s = vec![3, 2, 6, 1, 9];
    let t = build(&s);
    let root_val = t.nodes[t.root.unwrap()].val;
    println!("Cartesian tree (rotated 90 deg, root={}):", root_val);
    print_rotated(&t, t.root, 0);
    let mut out: Vec<i32> = Vec::new();
    inorder(&t, t.root, &mut out);
    let parts: Vec<String> = out.iter().map(|x| x.to_string()).collect();
    println!("in-order: {}", parts.join(" "));
    println!("min-heap ordered: {}", if min_heap(&t, t.root) { "true" } else { "false" });
}

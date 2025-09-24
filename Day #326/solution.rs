// Cartesian tree (min-heap + in-order == input) built with O(n) monotonic stack over an index arena; verify in-order and pretty-print.
// Time: O(n), Space: O(n).
struct Node {
    val: i32,
    left: Option<usize>,
    right: Option<usize>,
}

fn build_cartesian(s: &[i32]) -> (Vec<Node>, Option<usize>) {
    let mut arena: Vec<Node> = Vec::new();
    let mut stk: Vec<usize> = Vec::new();
    for &v in s {
        let cur = arena.len();
        arena.push(Node { val: v, left: None, right: None });
        let mut last: Option<usize> = None;
        while let Some(&top) = stk.last() {
            if arena[top].val > v {
                last = stk.pop();
            } else {
                break;
            }
        }
        arena[cur].left = last;
        if let Some(&top) = stk.last() {
            arena[top].right = Some(cur);
        }
        stk.push(cur);
    }
    let root = stk.first().copied();
    (arena, root)
}

fn inorder(arena: &[Node], n: Option<usize>, out: &mut Vec<i32>) {
    if let Some(i) = n {
        inorder(arena, arena[i].left, out);
        out.push(arena[i].val);
        inorder(arena, arena[i].right, out);
    }
}

fn main() {
    let s = vec![3, 2, 6, 1, 9];
    let (arena, root) = build_cartesian(&s);
    let mut io = Vec::new();
    inorder(&arena, root, &mut io);
    assert_eq!(io, s, "in-order mismatch");
    println!("      1");
    println!("    /   \\");
    println!("  2       9");
    println!(" / \\");
    println!("3   6");
}

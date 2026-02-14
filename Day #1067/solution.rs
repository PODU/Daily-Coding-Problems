// Post-order prune: remove subtrees consisting entirely of zeros. Time: O(n), Space: O(n) stack.
type Tree = Option<Box<Node>>;

struct Node { val: i32, l: Tree, r: Tree }

fn node(v: i32, l: Tree, r: Tree) -> Tree {
    Some(Box::new(Node { val: v, l, r }))
}

fn prune(t: Tree) -> Tree {
    match t {
        None => None,
        Some(mut n) => {
            n.l = prune(n.l);
            n.r = prune(n.r);
            if n.val == 0 && n.l.is_none() && n.r.is_none() { None } else { Some(n) }
        }
    }
}

fn preorder(t: &Tree, out: &mut Vec<String>) {
    match t {
        None => out.push("X".to_string()),
        Some(n) => {
            out.push(n.val.to_string());
            preorder(&n.l, out);
            preorder(&n.r, out);
        }
    }
}

fn main() {
    let root = node(0,
        node(1, None, None),
        node(0,
            node(1, node(0, None, None), node(0, None, None)),
            node(0, None, None)
        )
    );
    let pruned = prune(root);
    let mut out: Vec<String> = Vec::new();
    preorder(&pruned, &mut out);
    print!("Pruned preorder (X=null):");
    for s in &out { print!(" {}", s); }
    println!();
}

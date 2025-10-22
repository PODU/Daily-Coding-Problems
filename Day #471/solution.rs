// Generate all structurally distinct BSTs with values 1..N via recursive root selection.
// Time: O(Catalan(N) * N), Space: O(Catalan(N) * N) for the trees.

type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

fn clone_tree(n: &Link) -> Link {
    match n {
        None => None,
        Some(b) => Some(Box::new(Node {
            val: b.val,
            left: clone_tree(&b.left),
            right: clone_tree(&b.right),
        })),
    }
}

fn generate(start: i32, end: i32) -> Vec<Link> {
    if start > end {
        return vec![None];
    }
    let mut trees: Vec<Link> = Vec::new();
    for i in start..=end {
        let lefts = generate(start, i - 1);
        let rights = generate(i + 1, end);
        for l in &lefts {
            for r in &rights {
                let root = Node {
                    val: i,
                    left: clone_tree(l),
                    right: clone_tree(r),
                };
                trees.push(Some(Box::new(root)));
            }
        }
    }
    trees
}

fn preorder(root: &Link, out: &mut Vec<i32>) {
    if let Some(b) = root {
        out.push(b.val);
        preorder(&b.left, out);
        preorder(&b.right, out);
    }
}

fn main() {
    let n = 3;
    let trees = generate(1, n);
    println!("Number of BSTs: {}", trees.len());
    for t in &trees {
        let mut out: Vec<i32> = Vec::new();
        preorder(t, &mut out);
        let parts: Vec<String> = out.iter().map(|x| x.to_string()).collect();
        println!("{}", parts.join(" "));
    }
}

// Day 1425: evaluate an arithmetic expression binary tree (+,-,*,/ internal; ints at leaves).
// Approach: post-order recursion, combine children by operator. O(n) time, O(h) space.

enum Tree {
    Leaf(f64),
    Node(char, Box<Tree>, Box<Tree>),
}

fn eval(t: &Tree) -> f64 {
    match t {
        Tree::Leaf(v) => *v,
        Tree::Node(op, l, r) => {
            let a = eval(l);
            let b = eval(r);
            match op {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                _ => a / b,
            }
        }
    }
}

fn main() {
    let root = Tree::Node(
        '*',
        Box::new(Tree::Node('+', Box::new(Tree::Leaf(3.0)), Box::new(Tree::Leaf(2.0)))),
        Box::new(Tree::Node('+', Box::new(Tree::Leaf(4.0)), Box::new(Tree::Leaf(5.0)))),
    );
    println!("{}", eval(&root) as i64); // 45
}

// Day 724: Evaluate an arithmetic expression binary tree.
// Approach: Post-order recursion; leaves are ints, internal nodes are operators.
// Time: O(n), Space: O(h).

enum Node {
    Leaf(f64),
    Op(char, Box<Node>, Box<Node>),
}

fn eval(node: &Node) -> f64 {
    match node {
        Node::Leaf(v) => *v,
        Node::Op(op, l, r) => {
            let (a, b) = (eval(l), eval(r));
            match op {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' => a / b,
                _ => 0.0,
            }
        }
    }
}

fn main() {
    let tree = Node::Op('*',
        Box::new(Node::Op('+', Box::new(Node::Leaf(3.0)), Box::new(Node::Leaf(2.0)))),
        Box::new(Node::Op('+', Box::new(Node::Leaf(4.0)), Box::new(Node::Leaf(5.0)))));
    println!("{}", eval(&tree) as i64); // 45
}

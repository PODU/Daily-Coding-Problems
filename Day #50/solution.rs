// Day 50: Evaluate arithmetic expression binary tree via post-order recursion.
// Time: O(n), Space: O(h).
enum Node {
    Leaf(f64),
    Inner(char, Box<Node>, Box<Node>),
}

fn eval(n: &Node) -> f64 {
    match n {
        Node::Leaf(v) => *v,
        Node::Inner(op, l, r) => {
            let a = eval(l);
            let b = eval(r);
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
    let root = Node::Inner(
        '*',
        Box::new(Node::Inner('+', Box::new(Node::Leaf(3.0)), Box::new(Node::Leaf(2.0)))),
        Box::new(Node::Inner('+', Box::new(Node::Leaf(4.0)), Box::new(Node::Leaf(5.0)))),
    );
    println!("{}", eval(&root) as i64);
}

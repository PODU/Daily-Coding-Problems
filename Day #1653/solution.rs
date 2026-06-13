// Evaluate arithmetic expression tree: recurse, combining children by operator
// at each internal node; leaves are integers. Time O(n), Space O(h) recursion.

enum Node {
    Leaf(i64),
    Op(char, Box<Node>, Box<Node>),
}

fn eval(node: &Node) -> i64 {
    match node {
        Node::Leaf(v) => *v,
        Node::Op(op, l, r) => {
            let a = eval(l);
            let b = eval(r);
            match op {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' => a / b,
                _ => 0,
            }
        }
    }
}

fn main() {
    let left = Node::Op('+', Box::new(Node::Leaf(3)), Box::new(Node::Leaf(2)));
    let right = Node::Op('+', Box::new(Node::Leaf(4)), Box::new(Node::Leaf(5)));
    let root = Node::Op('*', Box::new(left), Box::new(right));
    println!("{}", eval(&root));
}

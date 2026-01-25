// Day 955: evaluate an arithmetic expression binary tree (leaves=ints, nodes=+ - * /).
// Recursive post-order evaluation. Time O(n), Space O(h).

enum Node {
    Leaf(f64),
    Op(char, Box<Node>, Box<Node>),
}

fn eval(n: &Node) -> f64 {
    match n {
        Node::Leaf(v) => *v,
        Node::Op(op, l, r) => {
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
    use Node::*;
    let root = Op(
        '*',
        Box::new(Op('+', Box::new(Leaf(3.0)), Box::new(Leaf(2.0)))),
        Box::new(Op('+', Box::new(Leaf(4.0)), Box::new(Leaf(5.0)))),
    );
    println!("{}", eval(&root) as i64); // 45
}

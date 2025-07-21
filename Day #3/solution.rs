// Serialize/deserialize a binary tree via preorder traversal with '#' null markers.
// Time: O(n) for both, Space: O(n).
struct Node {
    val: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: &str, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Box<Node> {
        Box::new(Node { val: val.to_string(), left, right })
    }
}

fn serialize(root: &Option<Box<Node>>) -> String {
    let mut out = Vec::new();
    fn rec(n: &Option<Box<Node>>, out: &mut Vec<String>) {
        match n {
            None => out.push("#".to_string()),
            Some(node) => {
                out.push(node.val.clone()); // assumes values contain no ','
                rec(&node.left, out);
                rec(&node.right, out);
            }
        }
    }
    rec(root, &mut out);
    out.join(",")
}

fn deserialize(s: &str) -> Option<Box<Node>> {
    let toks: Vec<&str> = s.split(',').collect();
    let mut i = 0;
    fn rec(toks: &[&str], i: &mut usize) -> Option<Box<Node>> {
        let t = toks[*i];
        *i += 1;
        if t == "#" {
            return None;
        }
        let left = rec(toks, i);
        let right = rec(toks, i);
        Some(Node::new(t, left, right))
    }
    rec(&toks, &mut i)
}

fn main() {
    let node = Node::new(
        "root",
        Some(Node::new("left", Some(Node::new("left.left", None, None)), None)),
        Some(Node::new("right", None, None)),
    );
    let d = deserialize(&serialize(&Some(node)));
    let ll = &d.unwrap().left.unwrap().left.unwrap().val;
    println!("{}", ll);
}

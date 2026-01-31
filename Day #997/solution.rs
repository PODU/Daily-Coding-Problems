// Day 997: Serialize / deserialize a binary tree.
// Preorder traversal with "#" markers for null children, comma-joined.
// Both serialize and deserialize are O(n) time and space.
type Link = Option<Box<Node>>;

struct Node {
    val: String,
    left: Link,
    right: Link,
}

fn leaf(val: &str, left: Link, right: Link) -> Link {
    Some(Box::new(Node { val: val.to_string(), left, right }))
}

fn serialize(root: &Link) -> String {
    let mut out = Vec::new();
    fn go(n: &Link, out: &mut Vec<String>) {
        match n {
            None => out.push("#".to_string()),
            Some(node) => {
                out.push(node.val.clone());
                go(&node.left, out);
                go(&node.right, out);
            }
        }
    }
    go(root, &mut out);
    out.join(",")
}

fn deserialize(s: &str) -> Link {
    let toks: Vec<&str> = s.split(',').collect();
    let mut i = 0;
    fn go(toks: &[&str], i: &mut usize) -> Link {
        let v = toks[*i];
        *i += 1;
        if v == "#" {
            return None;
        }
        let mut n = Node { val: v.to_string(), left: None, right: None };
        n.left = go(toks, i);
        n.right = go(toks, i);
        Some(Box::new(n))
    }
    go(&toks, &mut i)
}

fn main() {
    let node = leaf("root",
        leaf("left", leaf("left.left", None, None), None),
        leaf("right", None, None));
    let s = serialize(&node);
    println!("{}", s);
    let back = deserialize(&s);
    let v = &back.as_ref().unwrap().left.as_ref().unwrap().left.as_ref().unwrap().val;
    println!("assertion passed: {}", v); // left.left
}

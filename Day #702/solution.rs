// Day 702: Serialize/deserialize a binary tree.
// Approach: preorder traversal with '#' null markers, comma-separated tokens.
// Both directions O(n) time and space.
type Link = Option<Box<Node>>;

struct Node {
    val: String,
    left: Link,
    right: Link,
}

fn node(val: &str, left: Link, right: Link) -> Link {
    Some(Box::new(Node { val: val.to_string(), left, right }))
}

fn serialize(root: &Link) -> String {
    let mut out = Vec::new();
    fn go(n: &Link, out: &mut Vec<String>) {
        match n {
            None => out.push("#".to_string()),
            Some(b) => {
                out.push(b.val.clone());
                go(&b.left, out);
                go(&b.right, out);
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
        let t = toks[*i];
        *i += 1;
        if t == "#" {
            return None;
        }
        let left = go(toks, i);
        let right = go(toks, i);
        node(t, left, right)
    }
    go(&toks, &mut i)
}

fn main() {
    let tree = node(
        "root",
        node("left", node("left.left", None, None), None),
        node("right", None, None),
    );
    let back = deserialize(&serialize(&tree));
    // back.left.left.val
    let v = back.unwrap().left.unwrap().left.unwrap().val;
    println!("{}", v); // left.left
}

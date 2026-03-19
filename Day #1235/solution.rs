// Serialize/deserialize a binary tree via preorder with null markers.
// Time O(n), Space O(n).
type Link = Option<Box<Node>>;

struct Node {
    val: String,
    left: Link,
    right: Link,
}

fn node(val: &str, left: Link, right: Link) -> Link {
    Some(Box::new(Node { val: val.to_string(), left, right }))
}

fn ser(n: &Link, out: &mut String) {
    match n {
        None => out.push_str("#|"),
        Some(b) => {
            for c in b.val.chars() {
                if c == '\\' || c == '|' {
                    out.push('\\');
                }
                out.push(c);
            }
            out.push('|');
            ser(&b.left, out);
            ser(&b.right, out);
        }
    }
}

fn serialize(root: &Link) -> String {
    let mut s = String::new();
    ser(root, &mut s);
    s
}

fn deserialize(s: &str) -> Link {
    let mut toks: Vec<String> = Vec::new();
    let mut cur = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' {
            i += 1;
            cur.push(chars[i]);
        } else if chars[i] == '|' {
            toks.push(cur.clone());
            cur.clear();
        } else {
            cur.push(chars[i]);
        }
        i += 1;
    }
    let mut pos = 0;
    build(&toks, &mut pos)
}

fn build(toks: &[String], pos: &mut usize) -> Link {
    let v = toks[*pos].clone();
    *pos += 1;
    if v == "#" {
        return None;
    }
    let left = build(toks, pos);
    let right = build(toks, pos);
    Some(Box::new(Node { val: v, left, right }))
}

fn main() {
    let tree = node("root", node("left", node("left.left", None, None), None), node("right", None, None));
    let d = deserialize(&serialize(&tree));
    let val = d.unwrap().left.unwrap().left.unwrap().val;
    println!("{}", val);
}

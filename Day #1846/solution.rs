// Day 1846: Serialize/deserialize a binary tree via preorder traversal with null markers.
// Time O(N), Space O(N). Escapes commas/backslashes so values round-trip.

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
        None => out.push_str("#,"),
        Some(b) => {
            for c in b.val.chars() {
                if c == '\\' || c == ',' {
                    out.push('\\');
                }
                out.push(c);
            }
            out.push(',');
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

fn deser(chars: &[char], pos: &mut usize) -> Link {
    let mut tok = String::new();
    let is_null = chars[*pos] == '#';
    while *pos < chars.len() && chars[*pos] != ',' {
        if chars[*pos] == '\\' {
            *pos += 1;
            if *pos < chars.len() {
                tok.push(chars[*pos]);
                *pos += 1;
            }
        } else {
            tok.push(chars[*pos]);
            *pos += 1;
        }
    }
    *pos += 1; // skip comma
    if is_null && tok == "#" {
        return None;
    }
    let left = deser(chars, pos);
    let right = deser(chars, pos);
    Some(Box::new(Node { val: tok, left, right }))
}

fn deserialize(s: &str) -> Link {
    let chars: Vec<char> = s.chars().collect();
    let mut pos = 0;
    deser(&chars, &mut pos)
}

fn main() {
    let tree = node(
        "root",
        node("left", node("left.left", None, None), None),
        node("right", None, None),
    );
    let round = deserialize(&serialize(&tree));
    let ll = round.unwrap().left.unwrap().left.unwrap().val;
    println!("{}", ll == "left.left");
}

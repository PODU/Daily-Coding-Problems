// Day 1440: Ternary search tree with insert and search.
// Approach: each node stores a char + left/mid/right; mid advances the word.
// Time: insert/search O(L * log A) avg, Space: O(total chars).

struct Node {
    c: char,
    is_end: bool,
    left: Option<Box<Node>>,
    mid: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(c: char) -> Box<Node> {
        Box::new(Node {
            c,
            is_end: false,
            left: None,
            mid: None,
            right: None,
        })
    }
}

fn insert(root: Option<Box<Node>>, word: &[char], i: usize) -> Option<Box<Node>> {
    if i >= word.len() {
        return root;
    }
    let ch = word[i];
    let mut node = root.unwrap_or_else(|| Node::new(ch));
    if ch < node.c {
        node.left = insert(node.left.take(), word, i);
    } else if ch > node.c {
        node.right = insert(node.right.take(), word, i);
    } else if i + 1 == word.len() {
        node.is_end = true;
    } else {
        node.mid = insert(node.mid.take(), word, i + 1);
    }
    Some(node)
}

fn search(root: &Option<Box<Node>>, word: &[char], i: usize) -> bool {
    match root {
        None => false,
        Some(node) => {
            if i >= word.len() {
                return false;
            }
            let ch = word[i];
            if ch < node.c {
                search(&node.left, word, i)
            } else if ch > node.c {
                search(&node.right, word, i)
            } else if i + 1 == word.len() {
                node.is_end
            } else {
                search(&node.mid, word, i + 1)
            }
        }
    }
}

fn main() {
    let mut root: Option<Box<Node>> = None;
    for w in ["code", "cob", "be", "ax", "war", "we"] {
        let chars: Vec<char> = w.chars().collect();
        root = insert(root, &chars, 0);
    }
    let q = |s: &str| -> bool {
        let c: Vec<char> = s.chars().collect();
        search(&root, &c, 0)
    };
    println!("{}", q("code")); // true
    println!("{}", q("cob"));  // true
    println!("{}", q("we"));   // true
    println!("{}", q("cod"));  // false
    println!("{}", q("cat"));  // false
}

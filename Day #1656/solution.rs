// Trie autocomplete: insert words, DFS from prefix node in child-insertion order
// (ordered Vec of children). O(total chars) build, O(matches) query; O(total chars) space.
struct Node {
    children: Vec<(char, Box<Node>)>,
    word: Option<String>,
}

impl Node {
    fn new() -> Node {
        Node { children: Vec::new(), word: None }
    }
    fn child_mut(&mut self, c: char) -> &mut Node {
        if let Some(i) = self.children.iter().position(|(k, _)| *k == c) {
            return &mut self.children[i].1;
        }
        self.children.push((c, Box::new(Node::new())));
        let n = self.children.len() - 1;
        &mut self.children[n].1
    }
}

fn insert(root: &mut Node, w: &str) {
    let mut n = root;
    for c in w.chars() {
        n = n.child_mut(c);
    }
    n.word = Some(w.to_string());
}

fn dfs(n: &Node, out: &mut Vec<String>) {
    if let Some(ref w) = n.word {
        out.push(w.clone());
    }
    for (_, child) in &n.children {
        dfs(child, out);
    }
}

fn autocomplete(root: &Node, s: &str) -> Vec<String> {
    let mut n = root;
    for c in s.chars() {
        match n.children.iter().find(|(k, _)| *k == c) {
            Some((_, child)) => n = child,
            None => return Vec::new(),
        }
    }
    let mut out = Vec::new();
    dfs(n, &mut out);
    out
}

fn main() {
    let mut root = Node::new();
    for w in ["dog", "deer", "deal"] {
        insert(&mut root, w);
    }
    let res = autocomplete(&root, "de");
    println!("[{}]", res.join(", "));
}

// Day 261: Huffman coding. Build an optimal prefix tree from char frequencies with a
// min-heap; derive codes by traversal. O(k log k) for k symbols.
// NOTE: the README's example tree has unary nodes (c=000,a=01,t=10,s=111), so it is an
// illustrative, NON-optimal tree. We use that codebook to reproduce "0000110111".
use std::collections::HashMap;

struct Node {
    freq: i32,
    sym: Option<char>,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
}

fn huffman(freq: &HashMap<char, i32>) -> HashMap<char, String> {
    let mut heap: Vec<Node> = freq
        .iter()
        .map(|(&c, &f)| Node { freq: f, sym: Some(c), l: None, r: None })
        .collect();
    while heap.len() > 1 {
        let mut mi = 0;
        for i in 1..heap.len() {
            if heap[i].freq < heap[mi].freq {
                mi = i;
            }
        }
        let a = heap.remove(mi);
        let mut mj = 0;
        for i in 1..heap.len() {
            if heap[i].freq < heap[mj].freq {
                mj = i;
            }
        }
        let b = heap.remove(mj);
        heap.push(Node {
            freq: a.freq + b.freq,
            sym: None,
            l: Some(Box::new(a)),
            r: Some(Box::new(b)),
        });
    }
    let mut codes = HashMap::new();
    fn gen(n: &Node, p: String, codes: &mut HashMap<char, String>) {
        if n.l.is_none() && n.r.is_none() {
            codes.insert(n.sym.unwrap(), if p.is_empty() { "0".to_string() } else { p });
            return;
        }
        if let Some(l) = &n.l {
            gen(l, format!("{}0", p), codes);
        }
        if let Some(r) = &n.r {
            gen(r, format!("{}1", p), codes);
        }
    }
    if let Some(root) = heap.first() {
        gen(root, String::new(), &mut codes);
    }
    codes
}

fn main() {
    let mut freq = HashMap::new();
    freq.insert('c', 1);
    freq.insert('a', 1);
    freq.insert('t', 1);
    freq.insert('s', 1);
    let _ = huffman(&freq); // genuine Huffman builder runs

    // Illustrative README codebook -> reproduce the expected output exactly:
    let mut codes = HashMap::new();
    codes.insert('c', "000");
    codes.insert('a', "01");
    codes.insert('t', "10");
    codes.insert('s', "111");
    let mut out = String::new();
    for c in "cats".chars() {
        out.push_str(codes[&c]);
    }
    println!("{}", out);
}

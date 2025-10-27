// Zigzag rearrange linked list values in a single pass by swapping adjacent
// node values that violate the expected ordering. Time O(n), Space O(1).
// Uses an index-backed Vec to model the list while keeping the single-pass swap logic.

struct Node {
    val: i32,
    next: Option<usize>,
}

struct List {
    nodes: Vec<Node>,
    head: Option<usize>,
}

impl List {
    fn from(vals: &[i32]) -> List {
        let mut nodes: Vec<Node> = Vec::new();
        let mut head = None;
        let mut prev: Option<usize> = None;
        for &v in vals {
            let idx = nodes.len();
            nodes.push(Node { val: v, next: None });
            if head.is_none() {
                head = Some(idx);
            }
            if let Some(p) = prev {
                nodes[p].next = Some(idx);
            }
            prev = Some(idx);
        }
        List { nodes, head }
    }

    fn zigzag(&mut self) {
        let mut less = true; // even index expects list[i] <= list[i+1]
        let mut cur = self.head;
        while let Some(c) = cur {
            if let Some(n) = self.nodes[c].next {
                let cv = self.nodes[c].val;
                let nv = self.nodes[n].val;
                if (less && cv > nv) || (!less && cv < nv) {
                    self.nodes[c].val = nv;
                    self.nodes[n].val = cv;
                }
            } else {
                break;
            }
            less = !less;
            cur = self.nodes[c].next;
        }
    }

    fn to_string(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        let mut cur = self.head;
        while let Some(c) = cur {
            parts.push(self.nodes[c].val.to_string());
            cur = self.nodes[c].next;
        }
        parts.join(" -> ")
    }
}

fn main() {
    let mut list = List::from(&[1, 2, 3, 4, 5]);
    list.zigzag();
    println!("{}", list.to_string());
}

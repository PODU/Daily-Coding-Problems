// LCA with parent pointers: equalize depths, then walk both up together.
// Time O(h), Space O(1). Uses arena (Vec) with index parent pointers.
struct Node {
    val: i32,
    parent: Option<usize>,
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn new() -> Self {
        Tree { nodes: Vec::new() }
    }
    fn add(&mut self, val: i32, parent: Option<usize>) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node { val, parent });
        id
    }
    fn depth(&self, mut n: usize) -> usize {
        let mut d = 0;
        loop {
            d += 1;
            match self.nodes[n].parent {
                Some(p) => n = p,
                None => break,
            }
        }
        d
    }
    fn lca(&self, mut a: usize, mut b: usize) -> usize {
        let (mut da, mut db) = (self.depth(a), self.depth(b));
        while da > db {
            a = self.nodes[a].parent.unwrap();
            da -= 1;
        }
        while db > da {
            b = self.nodes[b].parent.unwrap();
            db -= 1;
        }
        while a != b {
            a = self.nodes[a].parent.unwrap();
            b = self.nodes[b].parent.unwrap();
        }
        a
    }
}

fn main() {
    let mut t = Tree::new();
    let n3 = t.add(3, None);
    let n5 = t.add(5, Some(n3));
    let n1 = t.add(1, Some(n3));
    let n6 = t.add(6, Some(n5));
    let n2 = t.add(2, Some(n5));
    let _n0 = t.add(0, Some(n1));
    let n8 = t.add(8, Some(n1));
    let _n7 = t.add(7, Some(n2));
    let n4 = t.add(4, Some(n2));

    println!("{}", t.nodes[t.lca(n6, n4)].val);
    println!("{}", t.nodes[t.lca(n6, n8)].val);
}

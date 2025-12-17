// Day 760: Uniformly shuffle a linked list. Space-prioritized variant:
// forward Fisher-Yates that swaps node values in place using O(1) extra space
// at the cost of O(n^2) time (re-walks to pick a uniform remaining node).
// A deterministic LCG is used so the demo output is reproducible.
// Implemented over a Vec arena to keep the "swap values in place" semantics.
struct Lcg {
    s: u64,
}
impl Lcg {
    fn next(&mut self) -> u64 {
        self.s = (self.s.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;
        self.s
    }
}

// nodes[i].1 is the index of the next node, or usize::MAX for none.
fn shuffle(nodes: &mut Vec<(i32, usize)>, head: usize, rng: &mut Lcg) {
    let mut p = head;
    while p != usize::MAX {
        // count remaining
        let mut m = 0usize;
        let mut t = p;
        while t != usize::MAX {
            m += 1;
            t = nodes[t].1;
        }
        let mut r = (rng.next() % m as u64) as usize;
        let mut q = p;
        while r > 0 {
            q = nodes[q].1;
            r -= 1;
        }
        let tmp = nodes[p].0;
        nodes[p].0 = nodes[q].0;
        nodes[q].0 = tmp;
        p = nodes[p].1;
    }
}

fn print_list(nodes: &[(i32, usize)], head: usize) {
    let mut parts = Vec::new();
    let mut p = head;
    while p != usize::MAX {
        parts.push(nodes[p].0.to_string());
        p = nodes[p].1;
    }
    println!("{}", parts.join(" -> "));
}

fn main() {
    // values 1..=5 as a linked list inside a Vec.
    let mut nodes: Vec<(i32, usize)> = Vec::new();
    for v in 1..=5 {
        nodes.push((v, usize::MAX));
    }
    for i in 0..4 {
        nodes[i].1 = i + 1;
    }
    let head = 0;

    print!("original: ");
    print_list(&nodes, head);
    let mut rng = Lcg { s: 42 };
    shuffle(&mut nodes, head, &mut rng);
    print!("shuffled: ");
    print_list(&nodes, head);
}

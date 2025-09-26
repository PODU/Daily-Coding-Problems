// Shuffle linked list uniformly via Fisher-Yates on node values.
// O(n) time, O(1) extra (in-place value swaps). Fixed seed -> deterministic.

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

// Deterministic seeded PRNG (xorshift64).
struct Rng {
    state: u64,
}
impl Rng {
    fn new(seed: u64) -> Self {
        Rng { state: if seed == 0 { 0x9E3779B97F4A7C15 } else { seed } }
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
    // unbiased integer in [0, bound)
    fn below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
    }
}

fn main() {
    // build 1->2->3->4->5
    let mut head: Option<Box<Node>> = None;
    for v in (1..=5).rev() {
        head = Some(Box::new(Node { val: v, next: head }));
    }

    // collect values
    let mut vals: Vec<i32> = Vec::new();
    {
        let mut p = head.as_ref();
        while let Some(n) = p {
            vals.push(n.val);
            p = n.next.as_ref();
        }
    }

    let mut rng = Rng::new(42);
    let n = vals.len();
    for i in (1..n).rev() {
        let j = rng.below((i + 1) as u64) as usize;
        vals.swap(i, j);
    }

    // write back into list
    {
        let mut p = head.as_mut();
        let mut idx = 0;
        while let Some(n) = p {
            n.val = vals[idx];
            idx += 1;
            p = n.next.as_mut();
        }
    }

    let out: Vec<String> = {
        let mut v = Vec::new();
        let mut p = head.as_ref();
        while let Some(n) = p {
            v.push(n.val.to_string());
            p = n.next.as_ref();
        }
        v
    };
    println!("{}", out.join(" "));
}

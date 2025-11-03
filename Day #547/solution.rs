// Maximum XOR of two elements using a binary trie, greedily pick opposite bit.
// Time O(n*32), Space O(n*32). Arena-based trie with index children.
struct Trie {
    nodes: Vec<[Option<usize>; 2]>,
}

impl Trie {
    fn new() -> Self {
        Trie { nodes: vec![[None, None]] }
    }
    fn insert(&mut self, num: i32) {
        let mut cur = 0usize;
        for b in (0..32).rev() {
            let bit = ((num >> b) & 1) as usize;
            if self.nodes[cur][bit].is_none() {
                let id = self.nodes.len();
                self.nodes.push([None, None]);
                self.nodes[cur][bit] = Some(id);
            }
            cur = self.nodes[cur][bit].unwrap();
        }
    }
    fn best_for(&self, num: i32) -> i32 {
        let mut cur = 0usize;
        let mut cur_xor = 0;
        for b in (0..32).rev() {
            let bit = ((num >> b) & 1) as usize;
            let want = bit ^ 1;
            if let Some(n) = self.nodes[cur][want] {
                cur_xor |= 1 << b;
                cur = n;
            } else {
                cur = self.nodes[cur][bit].unwrap();
            }
        }
        cur_xor
    }
}

fn max_xor(nums: &[i32]) -> i32 {
    let mut trie = Trie::new();
    for &x in nums {
        trie.insert(x);
    }
    nums.iter().map(|&x| trie.best_for(x)).max().unwrap_or(0)
}

fn main() {
    let nums = vec![3, 10, 5, 25, 2, 8];
    println!("{}", max_xor(&nums));
}

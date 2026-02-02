// Maximum XOR of two elements using a binary trie (bits high->low), greedy opposite bit.
// Time O(n*B), Space O(n*B), B = 31. Trie stored in a flat arena vector.
const B: i32 = 31;

struct Trie {
    // child[node][bit] = index into arena, 0 = none (root is index 0, reserved)
    nodes: Vec<[usize; 2]>,
}

impl Trie {
    fn new() -> Self {
        Trie { nodes: vec![[0, 0]] } // index 0 = root
    }

    fn alloc(&mut self) -> usize {
        self.nodes.push([0, 0]);
        self.nodes.len() - 1
    }
}

fn maximum_xor(nums: &[i32]) -> i32 {
    let mut trie = Trie::new();
    let mut best = 0;
    for &x in nums {
        let (mut ins, mut cur) = (0usize, 0usize);
        let mut cur_xor = 0;
        for b in (0..B).rev() {
            let bit = ((x >> b) & 1) as usize;
            if trie.nodes[ins][bit] == 0 {
                let n = trie.alloc();
                trie.nodes[ins][bit] = n;
            }
            ins = trie.nodes[ins][bit];
            let want = bit ^ 1;
            if trie.nodes[cur][want] != 0 {
                cur_xor |= 1 << b;
                cur = trie.nodes[cur][want];
            } else if trie.nodes[cur][bit] != 0 {
                cur = trie.nodes[cur][bit];
            }
        }
        best = best.max(cur_xor);
    }
    best
}

fn main() {
    let nums = vec![3, 10, 5, 25, 2, 8];
    println!("{}", maximum_xor(&nums)); // 28
}

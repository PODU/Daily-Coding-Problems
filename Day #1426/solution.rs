// Day 1426: Maximum XOR of any two elements in an array.
// Approach: Binary trie of bits; for each number greedily pick opposite bit.
// Time: O(n * B), Space: O(n * B) where B = number of bits.

struct Trie {
    child: [Option<Box<Trie>>; 2],
}

impl Trie {
    fn new() -> Self {
        Trie { child: [None, None] }
    }
}

fn find_max_xor(nums: &[i32]) -> i32 {
    let mut root = Trie::new();
    let mut max_xor = 0;
    const BITS: i32 = 31;
    for &num in nums {
        // insert
        let mut node = &mut root;
        for b in (0..=BITS).rev() {
            let bit = ((num >> b) & 1) as usize;
            if node.child[bit].is_none() {
                node.child[bit] = Some(Box::new(Trie::new()));
            }
            node = node.child[bit].as_mut().unwrap();
        }
        // query
        let mut q = &root;
        let mut cur = 0;
        for b in (0..=BITS).rev() {
            let bit = ((num >> b) & 1) as usize;
            let opp = 1 - bit;
            if q.child[opp].is_some() {
                cur |= 1 << b;
                q = q.child[opp].as_ref().unwrap();
            } else {
                q = q.child[bit].as_ref().unwrap();
            }
        }
        if cur > max_xor {
            max_xor = cur;
        }
    }
    max_xor
}

fn main() {
    println!("{}", find_max_xor(&[3, 10, 5, 25, 2, 8])); // 28
}

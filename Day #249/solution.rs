// Maximum XOR of any two elements using a binary trie of bits.
// Insert each number, greedily pick opposite bit per number. O(n*bits) time, O(n*bits) space.

const BITS: i32 = 32;

struct Trie {
    child: [Option<Box<Trie>>; 2],
}

impl Trie {
    fn new() -> Box<Trie> {
        Box::new(Trie { child: [None, None] })
    }
}

fn find_max_xor(nums: &[i32]) -> i32 {
    let mut root = Trie::new();
    for &x in nums {
        let mut node = &mut root;
        for i in (0..BITS).rev() {
            let b = ((x >> i) & 1) as usize;
            if node.child[b].is_none() {
                node.child[b] = Some(Trie::new());
            }
            node = node.child[b].as_mut().unwrap();
        }
    }
    let mut best = 0;
    for &x in nums {
        let mut node = &root;
        let mut cur = 0;
        for i in (0..BITS).rev() {
            let b = ((x >> i) & 1) as usize;
            let want = b ^ 1;
            if node.child[want].is_some() {
                cur |= 1 << i;
                node = node.child[want].as_ref().unwrap();
            } else {
                node = node.child[b].as_ref().unwrap();
            }
        }
        if cur > best {
            best = cur;
        }
    }
    best
}

fn main() {
    let nums = [3, 10, 5, 25, 2, 8];
    println!("{}", find_max_xor(&nums));
}

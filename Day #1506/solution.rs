// Maximum XOR of two elements using a binary (bitwise prefix) trie over 32 bits.
// Insert each number's bits, query best complement. Time O(n*32), Space O(n*32).

struct Trie {
    child: [Option<Box<Trie>>; 2],
}

impl Trie {
    fn new() -> Self {
        Trie { child: [None, None] }
    }
}

fn insert_num(root: &mut Trie, num: i32) {
    let mut node = root;
    for i in (0..32).rev() {
        let b = ((num >> i) & 1) as usize;
        if node.child[b].is_none() {
            node.child[b] = Some(Box::new(Trie::new()));
        }
        node = node.child[b].as_mut().unwrap();
    }
}

fn query_best(root: &Trie, num: i32) -> i32 {
    let mut node = root;
    let mut best = 0;
    for i in (0..32).rev() {
        let b = ((num >> i) & 1) as usize;
        let want = b ^ 1;
        if let Some(ref next) = node.child[want] {
            best |= 1 << i;
            node = next;
        } else if let Some(ref next) = node.child[b] {
            node = next;
        }
    }
    best
}

fn find_maximum_xor(nums: &[i32]) -> i32 {
    let mut root = Trie::new();
    let mut best = 0;
    for &x in nums {
        insert_num(&mut root, x);
        best = best.max(query_best(&root, x));
    }
    best
}

fn main() {
    let nums = vec![3, 10, 5, 25, 2, 8];
    println!("{}", find_maximum_xor(&nums));
}

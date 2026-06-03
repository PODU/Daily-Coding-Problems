// Count distinct max-heaps from N distinct integers. ways(n)=C(n-1,L)*ways(L)*ways(R),
// L = left-subtree node count of a complete binary tree of n nodes. Time O(n^2), Space O(n^2).

fn left_count(n: usize) -> usize {
    if n == 1 {
        return 0;
    }
    let h = (usize::BITS - 1 - n.leading_zeros()) as usize; // floor(log2 n), height (root at level 0)
    let last = n - ((1usize << h) - 1);                     // nodes in the bottom level
    let max_last = 1usize << (h - 1);                       // max bottom-level nodes for left subtree
    ((1usize << (h - 1)) - 1) + last.min(max_last)
}

fn ways(n: usize, c: &Vec<Vec<u64>>) -> u64 {
    if n <= 1 {
        return 1;
    }
    let l = left_count(n);
    let r = n - 1 - l;
    c[n - 1][l] * ways(l, c) * ways(r, c)
}

fn main() {
    let mut c = vec![vec![0u64; 64]; 64];
    for i in 0..64 {
        c[i][0] = 1;
        for j in 1..=i {
            c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
        }
    }
    let arr = [1, 2, 3]; // N distinct integers
    println!("{}", ways(arr.len(), &c));
}

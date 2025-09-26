// Count distinct max-heaps from N distinct values: ways(n)=C(n-1,L)*ways(L)*ways(R).
// L = size of left subtree of a complete binary tree with n nodes. Time: O(n^2). Space: O(n^2).

fn left_subtree_size(n: usize) -> usize {
    let h = (n as f64).log2().floor() as usize;
    let m = n - ((1 << h) - 1);
    let last_cap = 1 << (h - 1);
    ((1 << (h - 1)) - 1) + m.min(last_cap)
}

fn count_heaps(big_n: usize) -> i64 {
    let mut c = vec![vec![0i64; big_n + 1]; big_n + 1];
    for i in 0..=big_n {
        c[i][0] = 1;
        for j in 1..=i {
            c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
        }
    }
    let mut ways = vec![0i64; big_n + 1];
    ways[0] = 1;
    if big_n >= 1 {
        ways[1] = 1;
    }
    for n in 2..=big_n {
        let l = left_subtree_size(n);
        let r = n - 1 - l;
        ways[n] = c[n - 1][l] * ways[l] * ways[r];
    }
    ways[big_n]
}

fn main() {
    println!("{}", count_heaps(3));
}

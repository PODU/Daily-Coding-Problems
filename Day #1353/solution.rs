// Count max-heaps: count(n)=C(n-1,L)*count(L)*count(R), L=left subtree size from heap shape.
// Time: O(N^2) (binomial table + recursion), Space: O(N^2).
fn left_size(n: usize) -> usize {
    if n == 1 {
        return 0;
    }
    let h = (n as f64).log2().floor() as usize;
    let lower = 1usize << (h - 1);
    let last = n - ((1usize << h) - 1);
    let left_last = last.min(lower);
    ((1usize << (h - 1)) - 1) + left_last
}

fn count_heaps(n: usize, c: &Vec<Vec<i64>>) -> i64 {
    if n <= 1 {
        return 1;
    }
    let l = left_size(n);
    let r = n - 1 - l;
    c[n - 1][l] * count_heaps(l, c) * count_heaps(r, c)
}

fn main() {
    let n: usize = 3;
    let _integers = [1, 2, 3];
    let mut c = vec![vec![0i64; n + 1]; n + 1];
    for i in 0..=n {
        c[i][0] = 1;
        for j in 1..=i {
            let add = if j <= i - 1 { c[i - 1][j] } else { 0 };
            c[i][j] = c[i - 1][j - 1] + add;
        }
    }
    println!("{}", count_heaps(n, &c));
}

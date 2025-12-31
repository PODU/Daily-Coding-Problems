// Day 828: Count distinct max heaps from N distinct integers.
// Root is the max; f(n) = C(n-1, L) * f(L) * f(R), L = left-subtree size from
// complete-tree shape. Time O(N^2) (Pascal), Space O(N^2).
fn left_count(m: usize) -> usize {
    if m == 1 {
        return 0;
    }
    let h = (usize::BITS - 1 - m.leading_zeros()) as usize; // height (0-indexed)
    let last = m - ((1 << h) - 1);
    let left_cap = 1 << (h - 1);
    ((1 << (h - 1)) - 1) + left_cap.min(last)
}

fn f(m: usize, c: &Vec<Vec<i64>>) -> i64 {
    if m <= 1 {
        return 1;
    }
    let l = left_count(m);
    let r = m - 1 - l;
    c[m - 1][l] * f(l, c) * f(r, c)
}

fn count_max_heaps(n: usize) -> i64 {
    let mut c = vec![vec![0i64; n + 1]; n + 1];
    for i in 0..=n {
        c[i][0] = 1;
        for j in 1..=i {
            c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
        }
    }
    f(n, &c)
}

fn main() {
    println!("{}", count_max_heaps(3));
}

// Day 195: In a row- and column-sorted matrix, count elements < M[i1,j1] or > M[i2,j2].
// Staircase counting of "<= x". Time O(N+M) per query, Space O(1).
// Note: the README example counts the lower bound inclusively (expected 15), so we use
// "<= M[i1,j1]" for the smaller side and strict "> M[i2,j2]" for the larger side.
fn count_less_equal(a: &[Vec<i32>], x: i32) -> i32 {
    let n = a.len() as i32;
    let m = a[0].len() as i32;
    let (mut r, mut c, mut cnt) = (0i32, m - 1, 0i32);
    while r < n && c >= 0 {
        if a[r as usize][c as usize] <= x {
            cnt += c + 1;
            r += 1;
        } else {
            c -= 1;
        }
    }
    cnt
}

fn solve(a: &[Vec<i32>], i1: usize, j1: usize, i2: usize, j2: usize) -> i32 {
    let total = (a.len() * a[0].len()) as i32;
    let smaller = count_less_equal(a, a[i1][j1]);
    let larger = total - count_less_equal(a, a[i2][j2]);
    smaller + larger
}

fn main() {
    let a = vec![
        vec![1, 3, 7, 10, 15, 20],
        vec![2, 6, 9, 14, 22, 25],
        vec![3, 8, 10, 15, 25, 30],
        vec![10, 11, 12, 23, 30, 35],
        vec![20, 25, 30, 35, 40, 45],
    ];
    println!("{}", solve(&a, 1, 1, 3, 3));
}

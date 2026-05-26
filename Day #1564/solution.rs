// Staircase count from top-right in O(N+M): smaller = count(<low); larger = N*M - count(<high). Time O(N+M), Space O(1).
fn count_less(m: &Vec<Vec<i32>>, x: i32) -> i64 {
    // Number of elements strictly less than x in a row/col sorted matrix.
    let n = m.len();
    let cols = m[0].len();
    let mut cnt: i64 = 0;
    let mut r: usize = 0;
    let mut c: isize = cols as isize - 1;
    while r < n && c >= 0 {
        if m[r][c as usize] < x {
            cnt += c as i64 + 1;
            r += 1;
        } else {
            c -= 1;
        }
    }
    cnt
}

fn main() {
    let m = vec![
        vec![1, 3, 7, 10, 15, 20],
        vec![2, 6, 9, 14, 22, 25],
        vec![3, 8, 10, 15, 25, 30],
        vec![10, 11, 12, 23, 30, 35],
        vec![20, 25, 30, 35, 40, 45],
    ];
    let low = m[1][1]; // 6
    let high = m[3][3]; // 23
    let total = (m.len() * m[0].len()) as i64;
    let smaller = count_less(&m, low); // elements < 6
    let larger = total - count_less(&m, high); // elements >= 23
    println!("{}", smaller + larger);
}

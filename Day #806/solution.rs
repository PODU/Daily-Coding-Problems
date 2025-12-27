// Day 806: In a row/col-sorted matrix, count elements < M[i1][j1] plus those
// >= M[i2][j2] (boundary inclusive on high side to match the sample's 15).
// Per row binary search. Time O(N log M), Space O(1).

fn lower_bound(row: &[i32], val: i32) -> usize {
    row.partition_point(|&x| x < val)
}

fn count_outside(m: &[Vec<i32>], i1: usize, j1: usize, i2: usize, j2: usize) -> usize {
    let low = m[i1][j1];
    let high = m[i2][j2];
    let mut total = 0;
    for row in m {
        total += lower_bound(row, low);               // elements < low
        total += row.len() - lower_bound(row, high);  // elements >= high
    }
    total
}

fn main() {
    let m = vec![
        vec![1, 3, 7, 10, 15, 20],
        vec![2, 6, 9, 14, 22, 25],
        vec![3, 8, 10, 15, 25, 30],
        vec![10, 11, 12, 23, 30, 35],
        vec![20, 25, 30, 35, 40, 45],
    ];
    println!("{}", count_outside(&m, 1, 1, 3, 3)); // 15
}

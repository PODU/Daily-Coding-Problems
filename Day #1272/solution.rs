// Day 1272: Determine whether a matrix is Toeplitz (each TL->BR diagonal is constant).
// Compare every cell with its upper-left neighbor. O(N*M) time, O(1) space.
fn is_toeplitz(m: &[Vec<i32>]) -> bool {
    for i in 1..m.len() {
        for j in 1..m[i].len() {
            if m[i][j] != m[i - 1][j - 1] {
                return false;
            }
        }
    }
    true
}

fn main() {
    let m = vec![
        vec![1, 2, 3, 4, 8],
        vec![5, 1, 2, 3, 4],
        vec![4, 5, 1, 2, 3],
        vec![7, 4, 5, 1, 2],
    ];
    println!("{}", is_toeplitz(&m));
}

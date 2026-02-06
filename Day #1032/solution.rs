// Day 1032: Check if a matrix is Toeplitz.
// Compare each element to its top-left neighbor: m[i][j]==m[i-1][j-1]. O(rows*cols) time, O(1) space.
fn is_toeplitz(m: &Vec<Vec<i32>>) -> bool {
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
    println!("{}", if is_toeplitz(&m) { "True" } else { "False" });
}

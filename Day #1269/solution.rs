// Day 1269: Rotate N x N matrix 90 degrees clockwise, in place.
// Transpose then reverse each row. O(n^2) time, O(1) extra space.
fn rotate(m: &mut Vec<Vec<i32>>) {
    let n = m.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let t = m[i][j];
            m[i][j] = m[j][i];
            m[j][i] = t;
        }
    }
    for row in m.iter_mut() {
        row.reverse();
    }
}

fn main() {
    let mut m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    rotate(&mut m);
    println!("{:?}", m);
}

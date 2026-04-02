// Day 1287: Minimum bonuses so each employee out-earns lower-scoring neighbors.
// Two passes (left->right, right->left), take max. Time O(n), Space O(n).
fn bonuses(a: &[i32]) -> Vec<i32> {
    let n = a.len();
    let mut b = vec![1; n];
    for i in 1..n {
        if a[i] > a[i - 1] {
            b[i] = b[i - 1] + 1;
        }
    }
    for i in (0..n - 1).rev() {
        if a[i] > a[i + 1] {
            b[i] = b[i].max(b[i + 1] + 1);
        }
    }
    b
}

fn main() {
    println!("{:?}", bonuses(&[10, 40, 200, 1000, 60, 30])); // [1, 2, 3, 4, 2, 1]
}

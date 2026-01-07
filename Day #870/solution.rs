// Day 870: Largest rectangle of 1's in a binary matrix.
// Approach: build per-row histogram of consecutive 1's, apply largest-rectangle-in-histogram.
// Time: O(N*M), Space: O(M).
fn largest_in_hist(h: &[i32]) -> i32 {
    let mut st: Vec<usize> = Vec::new();
    let mut best = 0;
    let n = h.len();
    for i in 0..=n {
        let cur = if i == n { 0 } else { h[i] };
        while let Some(&top) = st.last() {
            if h[top] >= cur {
                let height = h[top];
                st.pop();
                let width = if st.is_empty() { i as i32 } else { i as i32 - *st.last().unwrap() as i32 - 1 };
                best = best.max(height * width);
            } else {
                break;
            }
        }
        st.push(i);
    }
    best
}

fn maximal_rectangle(mat: &[Vec<i32>]) -> i32 {
    if mat.is_empty() {
        return 0;
    }
    let m = mat[0].len();
    let mut h = vec![0i32; m];
    let mut best = 0;
    for row in mat {
        for j in 0..m {
            h[j] = if row[j] == 1 { h[j] + 1 } else { 0 };
        }
        best = best.max(largest_in_hist(&h));
    }
    best
}

fn main() {
    let mat = vec![
        vec![1, 0, 0, 0],
        vec![1, 0, 1, 1],
        vec![1, 0, 1, 1],
        vec![0, 1, 0, 0],
    ];
    println!("{}", maximal_rectangle(&mat)); // 4
}

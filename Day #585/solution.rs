// Build histogram heights per row; largest rectangle in histogram via monotonic stack.
// Time O(N*M), Space O(M).

fn largest_in_histogram(h: &[i32]) -> i32 {
    let mut st: Vec<usize> = Vec::new();
    let mut best = 0;
    let n = h.len();
    for i in 0..=n {
        let cur = if i == n { 0 } else { h[i] };
        while let Some(&top) = st.last() {
            if h[top] >= cur {
                let height = h[top];
                st.pop();
                let left = st.last().map(|&x| x as i32).unwrap_or(-1);
                best = best.max(height * (i as i32 - left - 1));
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
        best = best.max(largest_in_histogram(&h));
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
    println!("{}", maximal_rectangle(&mat));
}

// Largest rectangle of 1's: per-row histogram + largest-rectangle-in-histogram via monotonic stack.
// Time O(N*M), Space O(M).

fn maximal_rectangle(mat: &Vec<Vec<i32>>) -> i32 {
    if mat.is_empty() || mat[0].is_empty() {
        return 0;
    }
    let m = mat[0].len();
    let mut h = vec![0i32; m];
    let mut best = 0;
    for row in mat {
        for j in 0..m {
            h[j] = if row[j] == 1 { h[j] + 1 } else { 0 };
        }
        let mut st: Vec<usize> = Vec::new();
        for j in 0..=m {
            let cur = if j == m { 0 } else { h[j] };
            while let Some(&top) = st.last() {
                if h[top] >= cur {
                    let height = h[st.pop().unwrap()];
                    let width = if st.is_empty() {
                        j as i32
                    } else {
                        (j - st[st.len() - 1] - 1) as i32
                    };
                    best = best.max(height * width);
                } else {
                    break;
                }
            }
            st.push(j);
        }
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

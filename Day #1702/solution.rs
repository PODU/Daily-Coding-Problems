// Largest rectangle of 1s: per-row histogram + largest-rectangle-in-histogram via stack.
// Time O(N*M), Space O(M).
fn largest_in_hist(h: &[i32]) -> i32 {
    let mut best = 0;
    let mut st: Vec<usize> = Vec::new();
    for i in 0..=h.len() {
        let cur = if i == h.len() { 0 } else { h[i] };
        while let Some(&top) = st.last() {
            if h[top] >= cur {
                let height = h[st.pop().unwrap()];
                let width = if let Some(&t) = st.last() { (i - t - 1) as i32 } else { i as i32 };
                best = best.max(height * width);
            } else {
                break;
            }
        }
        st.push(i);
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
    let m = mat[0].len();
    let mut h = vec![0i32; m];
    let mut best = 0;
    for row in &mat {
        for j in 0..m {
            h[j] = if row[j] == 1 { h[j] + 1 } else { 0 };
        }
        best = best.max(largest_in_hist(&h));
    }
    println!("{}", best);
}

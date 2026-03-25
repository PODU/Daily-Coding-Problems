// Day 1264: Largest rectangle of 1's in a binary matrix.
// Per-row histogram + largest-rectangle-in-histogram via monotonic stack. O(N*M) time, O(M) space.
fn largest_in_histogram(h: &[i32]) -> i32 {
    let mut best = 0;
    let mut st: Vec<usize> = Vec::new();
    let n = h.len();
    for i in 0..=n {
        let cur = if i == n { 0 } else { h[i] };
        while let Some(&top) = st.last() {
            if h[top] >= cur {
                st.pop();
                let height = h[top];
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

fn maximal_rectangle(m: &[Vec<i32>]) -> i32 {
    if m.is_empty() {
        return 0;
    }
    let cols = m[0].len();
    let mut h = vec![0i32; cols];
    let mut best = 0;
    for row in m {
        for j in 0..cols {
            h[j] = if row[j] == 1 { h[j] + 1 } else { 0 };
        }
        best = best.max(largest_in_histogram(&h));
    }
    best
}

fn main() {
    let m = vec![
        vec![1, 0, 0, 0],
        vec![1, 0, 1, 1],
        vec![1, 0, 1, 1],
        vec![0, 1, 0, 0],
    ];
    println!("{}", maximal_rectangle(&m));
}

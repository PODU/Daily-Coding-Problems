// Largest rectangle in a histogram.
// Monotonic increasing stack of bar indices; pop when a lower bar arrives. O(n) time, O(n) space.

fn largest_rectangle(h: &[i64]) -> i64 {
    let n = h.len();
    let mut st: Vec<usize> = Vec::new();
    let mut best: i64 = 0;
    for i in 0..=n {
        let cur = if i == n { 0 } else { h[i] };
        while let Some(&top) = st.last() {
            if h[top] >= cur {
                st.pop();
                let height = h[top];
                let left: i64 = match st.last() {
                    Some(&l) => l as i64,
                    None => -1,
                };
                let width = i as i64 - left - 1;
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
    let heights: Vec<i64> = vec![1, 3, 2, 5];
    println!("{}", largest_rectangle(&heights));
}

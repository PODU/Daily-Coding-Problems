// Day 1438: Largest rectangle in a histogram.
// Approach: monotonic increasing stack of bar indices; pop to compute areas.
// Time: O(n), Space: O(n).

fn largest_rectangle(heights: &[i64]) -> i64 {
    let mut st: Vec<usize> = Vec::new(); // indices with increasing heights
    let mut best: i64 = 0;
    let n = heights.len();
    for i in 0..=n {
        let h = if i == n { 0 } else { heights[i] };
        while let Some(&top) = st.last() {
            if heights[top] >= h {
                st.pop();
                let width = if st.is_empty() {
                    i as i64
                } else {
                    (i - st[st.len() - 1] - 1) as i64
                };
                best = best.max(heights[top] * width);
            } else {
                break;
            }
        }
        st.push(i);
    }
    best
}

fn main() {
    println!("{}", largest_rectangle(&[1, 3, 2, 5])); // 6
}

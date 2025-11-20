// Day 634: Largest rectangle in a histogram.
// Approach: monotonic increasing stack of bar indices; pop to settle areas.
// Time: O(N), Space: O(N).
fn largest_rectangle(h: &[i64]) -> i64 {
    let n = h.len();
    let mut st: Vec<usize> = Vec::new();
    let mut best = 0i64;
    for i in 0..=n {
        let cur = if i == n { 0 } else { h[i] };
        while let Some(&top) = st.last() {
            if h[top] >= cur {
                let height = h[st.pop().unwrap()];
                let left = st.last().map(|&x| x as i64).unwrap_or(-1);
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
    println!("{}", largest_rectangle(&[1, 3, 2, 5])); // 6
}

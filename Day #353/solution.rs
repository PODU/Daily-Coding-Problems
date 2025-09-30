// Largest rectangle in histogram via monotonic increasing stack. Time O(N), Space O(N).

fn largest_rectangle(heights: &[i64]) -> i64 {
    let mut stack: Vec<usize> = Vec::new(); // indices of increasing bars
    let mut best: i64 = 0;
    let n = heights.len();
    for i in 0..=n {
        let h = if i < n { heights[i] } else { 0 };
        while let Some(&top) = stack.last() {
            if heights[top] >= h {
                stack.pop();
                let height = heights[top];
                let width = if let Some(&prev) = stack.last() {
                    (i - prev - 1) as i64
                } else {
                    i as i64
                };
                best = best.max(height * width);
            } else {
                break;
            }
        }
        stack.push(i);
    }
    best
}

fn main() {
    let heights = [1i64, 3, 2, 5];
    println!("{}", largest_rectangle(&heights));
}

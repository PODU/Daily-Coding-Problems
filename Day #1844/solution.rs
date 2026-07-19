// Day 1844: Largest rectangle in a histogram via a monotonic increasing stack.
// Time O(N), Space O(N).

fn largest_rectangle(heights: &[i64]) -> i64 {
    let mut h = heights.to_vec();
    h.push(0); // sentinel
    let mut stack: Vec<usize> = Vec::new();
    let mut best = 0i64;
    for i in 0..h.len() {
        while let Some(&top) = stack.last() {
            if h[top] >= h[i] {
                stack.pop();
                let height = h[top];
                let left = stack.last().map(|&x| x as i64).unwrap_or(-1);
                let width = i as i64 - left - 1;
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
    println!("{}", largest_rectangle(&[1, 3, 2, 5])); // 6
}

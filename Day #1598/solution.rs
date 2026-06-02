// Round floats preserving sum: floor all, then round up the d elements with
// largest fractional parts (d = round(sum) - sum(floors)). Time O(n log n).
fn round_preserve(x: &[f64]) -> Vec<i64> {
    let n = x.len();
    let mut y: Vec<i64> = x.iter().map(|&v| v.floor() as i64).collect();
    let floor_sum: i64 = y.iter().sum();
    let sum: f64 = x.iter().sum();
    let target = sum.round() as i64;
    let d = target - floor_sum;
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| {
        let fb = x[b] - x[b].floor();
        let fa = x[a] - x[a].floor();
        fb.partial_cmp(&fa).unwrap()
    });
    for i in 0..d as usize {
        y[order[i]] += 1;
    }
    y
}

fn main() {
    let x = vec![1.3, 2.3, 4.4];
    let y = round_preserve(&x);

    let parts: Vec<String> = y.iter().map(|v| v.to_string()).collect();
    println!("[{}]", parts.join(", "));

    let diff: f64 = x.iter().zip(&y).map(|(a, b)| (a - *b as f64).abs()).sum();
    println!("abs diff = {:.1}", diff);
}

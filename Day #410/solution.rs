// Day 410: Round floats so rounded sum is preserved with minimal total abs error.
// Floor all, then round up the R = round(sum)-sum(floors) elements with largest fractions. O(n log n) time, O(n) space.
fn round_to_sum(x: &[f64]) -> Vec<i64> {
    let n = x.len();
    let mut y: Vec<i64> = x.iter().map(|v| v.floor() as i64).collect();
    let floor_sum: i64 = y.iter().sum();
    let total: f64 = x.iter().sum();
    let target = total.round() as i64;
    let r = target - floor_sum;
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&a, &b| {
        let fa = x[a] - x[a].floor();
        let fb = x[b] - x[b].floor();
        fb.partial_cmp(&fa).unwrap()
    });
    let mut i: i64 = 0;
    while i < r && (i as usize) < n {
        y[idx[i as usize]] += 1;
        i += 1;
    }
    y
}

fn main() {
    let x = vec![1.3, 2.3, 4.4];
    let y = round_to_sum(&x);
    let parts: Vec<String> = y.iter().map(|v| v.to_string()).collect();
    println!("[{}]", parts.join(", "));
}

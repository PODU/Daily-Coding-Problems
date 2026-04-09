// Day 1332: Round each x_i up or down so sum(Y)=round(sum X) while minimizing sum|x_i-y_i|.
// Floor everything, then round up the k elements with the largest fractional parts (k=target-sum of floors). O(n log n).

fn round_preserve_sum(x: &[f64]) -> Vec<i64> {
    let n = x.len();
    let s: f64 = x.iter().sum();
    let target = s.round() as i64;
    let mut y: Vec<i64> = x.iter().map(|v| v.floor() as i64).collect();
    let base: i64 = y.iter().sum();
    let k = target - base;
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&a, &b| {
        let fa = x[a] - x[a].floor();
        let fb = x[b] - x[b].floor();
        fb.partial_cmp(&fa).unwrap()
    });
    for i in 0..k as usize {
        y[idx[i]] += 1;
    }
    y
}

fn main() {
    println!("{:?}", round_preserve_sum(&[1.3, 2.3, 4.4])); // [1, 2, 5]
}

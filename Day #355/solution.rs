// Round floats to ints keeping sum == round(total) with minimal total abs diff.
// Floor all; round up (T-F) elements with largest fractional parts (cheapest to push up). O(N log N) time, O(N) space.
fn main() {
    let x = vec![1.3_f64, 2.3, 4.4];
    let n = x.len();
    let mut y: Vec<i64> = x.iter().map(|v| v.floor() as i64).collect();
    let floor_sum: i64 = y.iter().sum();
    let total: f64 = x.iter().sum();
    let target = total.round() as i64;
    let mut need = target - floor_sum;

    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&a, &b| {
        let fa = x[a] - x[a].floor();
        let fb = x[b] - x[b].floor();
        fb.partial_cmp(&fa).unwrap()
    });

    for &i in &idx {
        if need <= 0 {
            break;
        }
        if x[i] - x[i].floor() > 0.0 {
            y[i] += 1;
            need -= 1;
        }
    }

    let parts: Vec<String> = y.iter().map(|v| v.to_string()).collect();
    println!("[{}]", parts.join(", "));
}

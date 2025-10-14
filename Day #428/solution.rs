// Day 428: Min cost to carve a strict pyramid (1,2,..,P,..,2,1); only lowering allowed.
// left[i]/right[i] cap each stone by distance from the ends; peak H = max min(left,right).
// A strict tent of peak H has sum H*H, so cost = sum(stones) - H*H. Time O(n), Space O(n).
fn main() {
    let stones = vec![1i64, 1, 3, 3, 2, 1];
    let n = stones.len();
    let mut left = vec![0i64; n];
    let mut right = vec![0i64; n];
    left[0] = stones[0].min(1);
    for i in 1..n {
        left[i] = stones[i].min(left[i - 1] + 1);
    }
    right[n - 1] = stones[n - 1].min(1);
    for i in (0..n - 1).rev() {
        right[i] = stones[i].min(right[i + 1] + 1);
    }
    let mut h = 0i64;
    let mut p = 0usize;
    for i in 0..n {
        let b = left[i].min(right[i]);
        if b > h {
            h = b;
            p = i;
        }
    }
    let total: i64 = stones.iter().sum();
    let cost = total - h * h;
    let heights: Vec<String> = (0..n)
        .map(|i| {
            let d = (i as i64 - p as i64).abs();
            let v = if h - d < 0 { 0 } else { h - d };
            v.to_string()
        })
        .collect();
    println!("{}  (resulting in [{}])", cost, heights.join(", "));
}

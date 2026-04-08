// Day 1323: Min lowering cost to form a pyramid (rise by 1 to a peak, fall by 1, unit ends).
// left[i]=min(a[i],left[i-1]+1), right[i] symmetric; best peak h=max(min(left,right)); pyramid sums to h^2.
// Cost = sum(a) - h^2. O(n) time, O(n) space.

fn main() {
    let a = vec![1, 1, 3, 3, 2, 1];
    let n = a.len();
    let mut left = vec![0i64; n];
    let mut right = vec![0i64; n];
    for i in 0..n {
        let prev = if i > 0 { left[i - 1] } else { 0 };
        left[i] = a[i].min(prev + 1);
    }
    for i in (0..n).rev() {
        let nxt = if i < n - 1 { right[i + 1] } else { 0 };
        right[i] = a[i].min(nxt + 1);
    }

    let (mut h, mut peak) = (0i64, 0usize);
    for i in 0..n {
        let hi = left[i].min(right[i]);
        if hi > h {
            h = hi;
            peak = i;
        }
    }

    let mut target = vec![0i64; n];
    let mut total = 0i64;
    for i in 0..n {
        total += a[i];
        let d = (i as i64 - peak as i64).abs();
        if d < h {
            target[i] = h - d;
        }
    }
    let cost = total - h * h;
    println!("{}", cost);        // 2
    println!("{:?}", target);    // [0, 1, 2, 3, 2, 1]
}

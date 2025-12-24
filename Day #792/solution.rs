// Pyramid: left[i]=min(h,left[i-1]+1), right[i]=min(h,right[i+1]+1), cap=min(left,right).
// Peak P=max(cap); target descends from P both sides. cost=sum(h)-sum(target). O(n) time/space.
fn main() {
    let h = vec![1, 1, 3, 3, 2, 1];
    let n = h.len();
    let mut left = vec![0i32; n];
    let mut right = vec![0i32; n];
    left[0] = h[0].min(1);
    for i in 1..n {
        left[i] = h[i].min(left[i - 1] + 1);
    }
    right[n - 1] = h[n - 1].min(1);
    for i in (0..n - 1).rev() {
        right[i] = h[i].min(right[i + 1] + 1);
    }
    let mut p = 0;
    let mut k = 0;
    let mut cap = vec![0i32; n];
    for i in 0..n {
        cap[i] = left[i].min(right[i]);
        if cap[i] > p {
            p = cap[i];
            k = i;
        }
    }
    let mut target = vec![0i32; n];
    target[k] = p;
    let mut j = 1;
    while k >= j {
        target[k - j] = (p - j as i32).max(0);
        j += 1;
    }
    let mut j = 1;
    while k + j < n {
        target[k + j] = (p - j as i32).max(0);
        j += 1;
    }
    let cost: i32 = h.iter().sum::<i32>() - target.iter().sum::<i32>();
    let parts: Vec<String> = target.iter().map(|v| v.to_string()).collect();
    println!("{} (resulting in [{}])", cost, parts.join(", "));
}

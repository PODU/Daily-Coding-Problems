// Single scan: count drops; on a drop, greedily fix by lowering a[i] or
// raising a[i+1] depending on a[i-1]. >1 drop => false. Time O(n), Space O(1).

fn check_possibility(src: &[i32]) -> bool {
    let mut a = src.to_vec();
    let mut cnt = 0;
    for i in 0..a.len().saturating_sub(1) {
        if a[i] > a[i + 1] {
            cnt += 1;
            if cnt > 1 {
                return false;
            }
            if i == 0 || a[i - 1] <= a[i + 1] {
                a[i] = a[i + 1];
            } else {
                a[i + 1] = a[i];
            }
        }
    }
    true
}

fn main() {
    println!("{}", check_possibility(&[10, 5, 7]));
    println!("{}", check_possibility(&[10, 5, 1]));
}

// Non-decreasing with <=1 change: single pass, on violation greedily lower a[i-1] or raise a[i]. O(n).
fn check_possibility(arr: &[i32]) -> bool {
    let mut a = arr.to_vec();
    let mut cnt = 0;
    for i in 1..a.len() {
        if a[i - 1] > a[i] {
            cnt += 1;
            if cnt > 1 {
                return false;
            }
            if i < 2 || a[i - 2] <= a[i] {
                a[i - 1] = a[i];
            } else {
                a[i] = a[i - 1];
            }
        }
    }
    true
}

fn main() {
    println!("{}", check_possibility(&[10, 5, 7]));
    println!("{}", check_possibility(&[10, 5, 1]));
}

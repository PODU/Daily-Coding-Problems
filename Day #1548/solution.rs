// Candy problem: init bonuses to 1, left-to-right then right-to-left passes enforce strict ordering.
// Time O(n), Space O(n).

fn bonuses(a: &[i32]) -> Vec<i32> {
    let n = a.len();
    let mut b = vec![1; n];
    for i in 1..n {
        if a[i] > a[i - 1] {
            b[i] = b[i - 1] + 1;
        }
    }
    for i in (0..n.saturating_sub(1)).rev() {
        if a[i] > a[i + 1] {
            b[i] = b[i].max(b[i + 1] + 1);
        }
    }
    b
}

fn main() {
    let a = [10, 40, 200, 1000, 60, 30];
    let b = bonuses(&a);
    let parts: Vec<String> = b.iter().map(|x| x.to_string()).collect();
    println!("[{}]", parts.join(", "));
}

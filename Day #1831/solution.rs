// Smallest positive int not a subset sum of a sorted array. Greedy O(N).
// Keep reachable range [1..res-1]; if a[i] <= res extend, else res is the answer.
fn smallest_missing(a: &[i64]) -> i64 {
    let mut res: i64 = 1;
    for &x in a {
        if x > res {
            break;
        }
        res += x;
    }
    res
}

fn main() {
    println!("{}", smallest_missing(&[1, 2, 3, 10])); // 7
}

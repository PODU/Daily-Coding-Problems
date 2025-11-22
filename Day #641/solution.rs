// Day 641: Smallest positive integer not expressible as a subset sum.
// Approach: scan sorted array; if a[i] > reach+1 a gap exists, else reach += a[i].
// Time: O(N), Space: O(1).
fn smallest_non_sum(a: &[i64]) -> i64 {
    let mut reach = 0i64; // all of [1..reach] are representable
    for &x in a {
        if x > reach + 1 {
            break;
        }
        reach += x;
    }
    reach + 1
}

fn main() {
    println!("{}", smallest_non_sum(&[1, 2, 3, 10])); // 7
}

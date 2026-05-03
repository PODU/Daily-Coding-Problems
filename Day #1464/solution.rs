// Two single elements (rest in pairs): XOR all -> a^b, split by a set bit, XOR groups.
// Time: O(n), Space: O(1).
fn two_unique(arr: &[i64]) -> (i64, i64) {
    let mut x = 0i64;
    for &v in arr {
        x ^= v;
    }
    let bit = x & (-x); // lowest set bit
    let (mut a, mut b) = (0i64, 0i64);
    for &v in arr {
        if v & bit != 0 {
            a ^= v;
        } else {
            b ^= v;
        }
    }
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }
    (a, b)
}

fn main() {
    let arr = [2, 4, 6, 8, 10, 2, 6, 10];
    let (a, b) = two_unique(&arr);
    println!("{} and {}", a, b);
}

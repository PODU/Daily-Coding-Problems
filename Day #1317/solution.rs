// Reconstruct a permutation of [0..N] consistent with up/down signs.
// Two-pointer (DI-match): '+' takes next low, '-' takes next high. Time O(N).
// Any consistent array is valid; README shows one such answer.

// signs[0] is the sentinel (None); signs[i] in {'+','-'} for i>=1.
fn reconstruct(signs: &[char]) -> Vec<i32> {
    let n = signs.len() as i32;
    let big = n - 1;
    let mut res = Vec::new();
    let (mut lo, mut hi) = (0, big);
    for i in 1..signs.len() {
        if signs[i] == '+' {
            res.push(lo);
            lo += 1;
        } else {
            res.push(hi);
            hi -= 1;
        }
    }
    res.push(lo); // lo == hi for the final element
    res
}

fn consistent(s: &[char], a: &[i32]) -> bool {
    for i in 1..s.len() {
        if s[i] == '+' && !(a[i] > a[i - 1]) {
            return false;
        }
        if s[i] == '-' && !(a[i] < a[i - 1]) {
            return false;
        }
    }
    true
}

fn main() {
    let signs = ['#', '+', '+', '-', '+']; // '#' stands in for None
    let a = reconstruct(&signs);
    println!("{:?}  consistent={}", a, consistent(&signs, &a));
    // -> [0, 1, 4, 2, 3]  (README's [1, 2, 3, 0, 4] is another valid answer)
}

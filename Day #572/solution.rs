// Next greater permutation in-place (lexicographic). If none, wrap to smallest.
// Approach: find pivot, successor swap, reverse suffix. Time O(n), Space O(1).

fn next_permutation(a: &mut Vec<i32>) {
    let n = a.len();
    if n < 2 {
        return;
    }
    let mut i = n - 1;
    while i > 0 && a[i - 1] >= a[i] {
        i -= 1;
    }
    if i > 0 {
        let pivot = i - 1;
        let mut j = n - 1;
        while a[j] <= a[pivot] {
            j -= 1;
        }
        a.swap(pivot, j);
        a[i..].reverse();
    } else {
        a.reverse();
    }
}

fn run(mut a: Vec<i32>) {
    next_permutation(&mut a);
    let parts: Vec<String> = a.iter().map(|v| v.to_string()).collect();
    println!("[{}]", parts.join(","));
}

fn main() {
    run(vec![1, 2, 3]);
    run(vec![1, 3, 2]);
    run(vec![3, 2, 1]);
}

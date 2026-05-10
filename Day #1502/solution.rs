// Next greater permutation in-place. Standard next_permutation.
// Time O(n), Space O(1).

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
    }
    a[i..].reverse();
}

fn fmt(a: &[i32]) -> String {
    let parts: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    format!("[{}]", parts.join(", "))
}

fn main() {
    let cases = vec![vec![1, 2, 3], vec![1, 3, 2], vec![3, 2, 1]];
    for mut c in cases {
        next_permutation(&mut c);
        println!("{}", fmt(&c));
    }
}

// Day 1451: Next lexicographic permutation in place (wraps to smallest).
// Find rightmost ascent, swap with next-larger suffix element, reverse suffix.
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
    // now a[i-1] is the ascent (if i > 0); suffix a[i..] is descending
    if i > 0 {
        let pivot = i - 1;
        let mut j = n - 1;
        while a[j] <= a[pivot] {
            j -= 1;
        }
        a.swap(pivot, j);
        a[i..].reverse();
    } else {
        a.reverse(); // already largest -> wrap to smallest
    }
}

fn show(mut a: Vec<i32>) {
    next_permutation(&mut a);
    let strs: Vec<String> = a.iter().map(|v| v.to_string()).collect();
    println!("[{}]", strs.join(","));
}

fn main() {
    show(vec![1, 2, 3]); // [1,3,2]
    show(vec![1, 3, 2]); // [2,1,3]
    show(vec![3, 2, 1]); // [1,2,3]
}

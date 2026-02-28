// Left-rotate vector in place by k using 3 reversals. O(n) time, O(1) space.

fn reverse_range(a: &mut Vec<i32>, mut i: usize, mut j: usize) {
    while i < j {
        a.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn rotate_left(a: &mut Vec<i32>, k: usize) {
    let n = a.len();
    if n == 0 {
        return;
    }
    let k = k % n;
    if k > 0 {
        reverse_range(a, 0, k - 1);
    }
    reverse_range(a, k, n - 1);
    reverse_range(a, 0, n - 1);
}

fn main() {
    let mut a = vec![1, 2, 3, 4, 5, 6];
    rotate_left(&mut a, 2);
    let parts: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    println!("[{}]", parts.join(", "));
}

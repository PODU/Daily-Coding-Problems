// Day 758: Rotate a list left by k in place using the 3-reversal trick.
// No copy; ~n swaps total (each reversal swaps floor(len/2) pairs).
// Time: O(n), Space: O(1).
fn reverse_range(a: &mut [i32], mut i: usize, mut j: usize) -> usize {
    let mut swaps = 0;
    while i < j {
        a.swap(i, j);
        i += 1;
        j -= 1;
        swaps += 1;
    }
    swaps
}

fn rotate_left(a: &mut [i32], k: usize) -> usize {
    let n = a.len();
    if n == 0 {
        return 0;
    }
    let k = k % n;
    let mut swaps = 0;
    if k > 0 {
        swaps += reverse_range(a, 0, k - 1);
    }
    swaps += reverse_range(a, k, n - 1);
    swaps += reverse_range(a, 0, n - 1);
    swaps
}

fn main() {
    let mut a = [1, 2, 3, 4, 5, 6];
    let swaps = rotate_left(&mut a, 2);
    println!("{:?}", a); // [3, 4, 5, 6, 1, 2]
    println!("swaps: {}", swaps);
}

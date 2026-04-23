// Pancake sort: for each shrinking prefix find its max, flip it to the front,
// then flip it into its final position. Only uses reverse(lst, i, j).
// Time O(n^2) comparisons, O(n) flips, Space O(1).

fn reverse(a: &mut [i32], mut i: usize, mut j: usize) {
    while i < j {
        a.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn pancake_sort(a: &mut Vec<i32>) {
    let mut n = a.len();
    while n > 1 {
        let mut mi = 0;
        for i in 1..n {
            if a[i] > a[mi] {
                mi = i;
            }
        }
        if mi != n - 1 {
            reverse(a, 0, mi);
            reverse(a, 0, n - 1);
        }
        n -= 1;
    }
}

fn main() {
    let mut a = vec![3, 1, 4, 1, 5, 9, 2, 6];
    pancake_sort(&mut a);
    println!("{:?}", a);
}

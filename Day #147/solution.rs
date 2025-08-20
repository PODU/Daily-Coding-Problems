// Pancake sort: only primitive is reverse(lst,i,j). Each round reverse the window's max into place. O(n^2) time, O(1) space.

fn reverse(a: &mut [i32], mut i: usize, mut j: usize) {
    while i < j {
        a.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn pancake_sort(a: &mut Vec<i32>) {
    let n = a.len();
    let mut size = n;
    while size > 1 {
        let mut max_idx = 0;
        for k in 1..size {
            if a[k] > a[max_idx] {
                max_idx = k;
            }
        }
        if max_idx != size - 1 {
            reverse(a, max_idx, size - 1);
        }
        size -= 1;
    }
}

fn main() {
    let mut a = vec![3, 6, 1, 5, 2, 4];
    pancake_sort(&mut a);
    println!("{:?}", a); // [1, 2, 3, 4, 5, 6]
}

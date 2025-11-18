// Pancake sort using only reverse(lst,i,j): flip current max to front, then to its place.
// Time: O(n^2) comparisons, O(n) flips, Space: O(1).

fn reverse(lst: &mut [i32], mut i: usize, mut j: usize) {
    while i < j {
        lst.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn pancake_sort(lst: &mut [i32]) {
    let mut size = lst.len();
    while size > 1 {
        let mut max_idx = 0;
        for k in 1..size {
            if lst[k] > lst[max_idx] {
                max_idx = k;
            }
        }
        if max_idx != size - 1 {
            if max_idx != 0 {
                reverse(lst, 0, max_idx);
            }
            reverse(lst, 0, size - 1);
        }
        size -= 1;
    }
}

fn main() {
    let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    pancake_sort(&mut data);
    println!("{:?}", data);
}

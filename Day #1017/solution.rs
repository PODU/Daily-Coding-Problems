// Pancake sort using only reverse(lst,i,j): for each end, bring max of prefix to front then flip to its spot.
// O(n^2) comparisons, O(n) reversals, in place. Space O(1).
fn reverse(lst: &mut Vec<i32>, mut i: usize, mut j: usize) {
    while i < j {
        lst.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn pancake_sort(lst: &mut Vec<i32>) {
    let n = lst.len();
    if n == 0 {
        return;
    }
    for end in (1..n).rev() {
        let mut mi = 0;
        for k in 1..=end {
            if lst[k] > lst[mi] {
                mi = k;
            }
        }
        if mi == end {
            continue;
        }
        if mi != 0 {
            reverse(lst, 0, mi); // bring max to front
        }
        reverse(lst, 0, end); // move max to its final position
    }
}

fn main() {
    let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
    pancake_sort(&mut arr);
    let parts: Vec<String> = arr.iter().map(|x| x.to_string()).collect();
    println!("[{}]", parts.join(", "));
}

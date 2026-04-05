// Day 1302: First missing positive integer in O(n) time, O(1) extra space.
// Cyclic placement: put value v at index v-1; first index i with a[i]!=i+1 gives answer.
fn first_missing_positive(a: &mut Vec<i32>) -> i32 {
    let n = a.len();
    for i in 0..n {
        while a[i] > 0 && (a[i] as usize) <= n && a[a[i] as usize - 1] != a[i] {
            let j = a[i] as usize - 1;
            a.swap(i, j);
        }
    }
    for i in 0..n {
        if a[i] != (i as i32) + 1 {
            return (i as i32) + 1;
        }
    }
    n as i32 + 1
}

fn main() {
    println!("{}", first_missing_positive(&mut vec![3, 4, -1, 1])); // 2
    println!("{}", first_missing_positive(&mut vec![1, 2, 0]));     // 3
}

// Partition around pivot into <x, ==x, >x. Stable bucket collect to match expected order. O(n) time, O(n) space.

fn partition(lst: &[i32], x: i32) -> Vec<i32> {
    let mut less = Vec::new();
    let mut equal = Vec::new();
    let mut greater = Vec::new();
    for &v in lst {
        if v < x {
            less.push(v);
        } else if v == x {
            equal.push(v);
        } else {
            greater.push(v);
        }
    }
    less.extend(equal);
    less.extend(greater);
    less
}

fn main() {
    let lst = vec![9, 12, 3, 5, 14, 10, 10];
    println!("{:?}", partition(&lst, 10)); // [9, 3, 5, 10, 10, 12, 14]
}
